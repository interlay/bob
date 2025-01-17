// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs} from "../IStrategy.sol";
import {StrategySlippageArgs} from "../CommonStructs.sol";
import {BedrockStrategy} from "./BedrockStrategy.sol";
import {SolvLSTStrategy} from "./SolvStrategy.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

interface IAvalonIPool {
    /**
     * @notice Mints pool tokens to the recipient.
     * @param asset The address of the ERC20 token to supply to the pool.
     * @param amount The amount of the asset to supply.
     * @param onBehalfOf The address that will receive the supplied amount.
     * @param referralCode Optional referral code to track the origin of the supply.
     */
    function supply(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;

    /**
     * @notice Withdraws asset from the pool.
     * @param asset The address of the ERC20 token to withdraw from the pool.
     * @param amount The amount of the asset to withdraw.
     * @param to The address that will receive the withdrawn amount.
     * @return The actual amount withdrawn.
     */
    function withdraw(address asset, uint256 amount, address to) external returns (uint256);
}

/**
 * @title Strategy contract for Avalon Finance.
 */
contract AvalonLendingStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    IERC20 public immutable avBep20;
    IAvalonIPool public immutable pool;

    constructor(IERC20 _avBep20, IAvalonIPool _pool) {
        avBep20 = _avBep20;
        pool = _pool;
    }

    /**
     * @notice Mints lending tokens to the recipient.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive the lending tokens.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(pool), amountIn);
        uint256 amountBefore = avBep20.balanceOf(recipient);

        // referralCode: 0, as the action is executed directly by the contract without any intermediary.
        // This parameter is typically used to register the integrator initiating the operation for potential rewards.
        pool.supply(address(tokenSent), amountIn, recipient, 0);

        uint256 amountAfter = avBep20.balanceOf(recipient);
        require(amountAfter > amountBefore, "Insufficient supply provided");
        uint256 amountOut = amountAfter - amountBefore;
        require(amountOut >= args.amountOutMin, "Insufficient output amount");

        emit TokenOutput(address(avBep20), amountOut);
    }
}

contract AvalonLstStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    SolvLSTStrategy public immutable solvLSTStrategy;
    AvalonLendingStrategy public immutable avalonLendingStrategy;

    constructor(SolvLSTStrategy _solvLSTStrategy, AvalonLendingStrategy _avalonLendingStrategy) {
        solvLSTStrategy = _solvLSTStrategy;
        avalonLendingStrategy = _avalonLendingStrategy;
    }

    /**
     * @notice Mints lending tokens to the recipient.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive the lending tokens.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvLSTStrategy), amountIn);

        solvLSTStrategy.handleGatewayMessageWithSlippageArgs(
            tokenSent, amountIn, address(this), StrategySlippageArgs(0)
        );

        IERC20 solvLST = solvLSTStrategy.solvLST();
        uint256 solvLSTAmount = solvLST.balanceOf(address(this));
        solvLST.safeIncreaseAllowance(address(avalonLendingStrategy), solvLSTAmount);

        avalonLendingStrategy.handleGatewayMessageWithSlippageArgs(solvLST, solvLSTAmount, recipient, args);
    }
}
