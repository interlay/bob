// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs} from "../IStrategy.sol";
import {StrategySlippageArgs} from "../CommonStructs.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Solv ABI for their router.
 */
interface ISolvBTCRouter {
    /**
     * Subscribe with payment currency (i.e. WBTC) and receive SolvBTC.
     * @param poolId SolvBTC fund ID.
     * @param currencyAmount Amount of currency to be paid.
     * @return shareValue Amount of SolvBTC to be received after subscription.
     */
    function createSubscription(bytes32 poolId, uint256 currencyAmount) external returns (uint256 shareValue);
}

/**
 * @title Strategy contract for SolvBTC.
 */
contract SolvBTCStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouter public immutable solvBTCRouter;
    bytes32 public immutable poolId;
    IERC20 public immutable solvBTC;

    constructor(ISolvBTCRouter _solvBTCRouter, bytes32 _poolId, IERC20 _solvBTC) {
        solvBTCRouter = _solvBTCRouter;
        poolId = _poolId;
        solvBTC = _solvBTC;
    }

    /**
     * @notice Deposits tokens into Solv.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive SolvBTC.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvBTCRouter), amountIn);

        uint256 shareValue = solvBTCRouter.createSubscription(poolId, amountIn);
        require(shareValue >= args.amountOutMin, "Insufficient output amount");

        solvBTC.safeTransfer(recipient, shareValue);

        emit TokenOutput(address(solvBTC), shareValue);
    }
}

/**
 * @title Strategy contract for e.g. SolvBTC.BBN.
 */
contract SolvLSTStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouter public immutable solvBTCRouter;
    ISolvBTCRouter public immutable solvLSTRouter;
    bytes32 public immutable solvBTCPoolId;
    bytes32 public immutable solvLSTPoolId;
    IERC20 public immutable solvBTC;
    IERC20 public immutable solvLST;

    constructor(
        ISolvBTCRouter _solvBTCRouter,
        ISolvBTCRouter _solvLSTRouter,
        bytes32 _solvBTCPoolId,
        bytes32 _solvLSTPoolId,
        IERC20 _solvBTC,
        IERC20 _solvLST
    ) {
        solvBTCRouter = _solvBTCRouter;
        solvLSTRouter = _solvLSTRouter;
        solvBTCPoolId = _solvBTCPoolId;
        solvLSTPoolId = _solvLSTPoolId;
        solvBTC = _solvBTC;
        solvLST = _solvLST;
    }

    /**
     * @notice Deposits tokens into Solv.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive e.g. SolvBTC.BBN.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvBTCRouter), amountIn);
        uint256 shareValueBTC = solvBTCRouter.createSubscription(solvBTCPoolId, amountIn);

        solvBTC.safeIncreaseAllowance(address(solvLSTRouter), shareValueBTC);
        uint256 shareValueLST = solvLSTRouter.createSubscription(solvLSTPoolId, shareValueBTC);
        require(shareValueLST >= args.amountOutMin, "Insufficient output amount");

        solvLST.safeTransfer(recipient, shareValueLST);

        emit TokenOutput(address(solvLST), shareValueLST);
    }
}
