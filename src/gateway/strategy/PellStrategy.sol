// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs} from "../IStrategy.sol";
import {StrategySlippageArgs} from "../CommonStructs.sol";
import {BedrockStrategy} from "./BedrockStrategy.sol";
import {SolvLSTStrategy} from "./SolvStrategy.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Pell ABI for their strategy manager.
 */
interface IPellStrategyManager {
    function depositIntoStrategyWithStaker(address staker, IPellStrategy strategy, IERC20 token, uint256 amount)
        external
        returns (uint256 shares);
    function stakerStrategyShares(address staker, IPellStrategy strategy) external view returns (uint256);
}

interface IPellStrategy {}

/**
 * @title Strategy contract for Pell Network.
 */
contract PellStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    IPellStrategyManager public immutable pellStrategyManager;
    IPellStrategy public immutable pellStrategy;

    constructor(IPellStrategyManager _pellStrategyManager, IPellStrategy _pellStrategy) {
        pellStrategyManager = _pellStrategyManager;
        pellStrategy = _pellStrategy;
    }

    /**
     * @notice Deposits tokens into Pell Network.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive the shares.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(pellStrategyManager), amountIn);

        uint256 shares = pellStrategyManager.depositIntoStrategyWithStaker(recipient, pellStrategy, tokenSent, amountIn);
        require(shares >= args.amountOutMin, "Insufficient output amount");

        emit TokenOutput(address(0), shares);
    }

    function stakerStrategyShares(address recipient) public view returns (uint256) {
        return pellStrategyManager.stakerStrategyShares(recipient, pellStrategy);
    }
}

contract PellBedrockStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    BedrockStrategy public immutable bedrockStrategy;
    PellStrategy public immutable pellStrategy;

    constructor(BedrockStrategy _bedrockStrategy, PellStrategy _pellStrategy) {
        bedrockStrategy = _bedrockStrategy;
        pellStrategy = _pellStrategy;
    }

    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amount,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amount);
        tokenSent.safeIncreaseAllowance(address(bedrockStrategy), amount);

        bedrockStrategy.handleGatewayMessageWithSlippageArgs(tokenSent, amount, address(this), StrategySlippageArgs(0));

        IERC20 uniBTC = IERC20(bedrockStrategy.vault().uniBTC());
        uint256 uniBTCAmount = uniBTC.balanceOf(address(this));
        uniBTC.safeIncreaseAllowance(address(pellStrategy), uniBTCAmount);

        pellStrategy.handleGatewayMessageWithSlippageArgs(uniBTC, uniBTCAmount, recipient, args);
    }
}

contract PellSolvLSTStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    SolvLSTStrategy public immutable solvLSTStrategy;
    PellStrategy public immutable pellStrategy;

    constructor(SolvLSTStrategy _solvLSTStrategy, PellStrategy _pellStrategy) {
        solvLSTStrategy = _solvLSTStrategy;
        pellStrategy = _pellStrategy;
    }

    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amount,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amount);
        tokenSent.safeIncreaseAllowance(address(solvLSTStrategy), amount);

        solvLSTStrategy.handleGatewayMessageWithSlippageArgs(tokenSent, amount, address(this), StrategySlippageArgs(0));

        IERC20 solvLST = solvLSTStrategy.solvLST();
        uint256 solvLSTAmount = solvLST.balanceOf(address(this));
        solvLST.safeIncreaseAllowance(address(pellStrategy), solvLSTAmount);

        pellStrategy.handleGatewayMessageWithSlippageArgs(solvLST, solvLSTAmount, recipient, args);
    }
}
