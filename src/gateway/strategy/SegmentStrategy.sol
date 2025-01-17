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
 * @dev Segment ABI for their seTokens.
 */
interface ISeBep20 {
    function mint(uint256 mintAmount) external returns (uint256);
    function mintBehalf(address receiver, uint256 mintAmount) external returns (uint256);
    function balanceOfUnderlying(address owner) external returns (uint256);
    function redeem(uint256 redeemTokens) external returns (uint256);
}

/**
 * @title Strategy contract for Segment Finance.
 */
contract SegmentStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISeBep20 public immutable seBep20;

    constructor(ISeBep20 _seBep20) {
        seBep20 = _seBep20;
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
        tokenSent.safeIncreaseAllowance(address(seBep20), amountIn);

        IERC20 token = IERC20(address(seBep20));
        uint256 amountBefore = token.balanceOf(recipient);
        uint256 err = seBep20.mintBehalf(recipient, amountIn);
        require(err == 0, "Could not mint token");
        uint256 amountAfter = token.balanceOf(recipient);
        require(amountAfter > amountBefore, "Insufficient supply provided");
        uint256 amountOut = amountAfter - amountBefore;
        require(amountOut >= args.amountOutMin, "Insufficient output amount");

        emit TokenOutput(address(token), amountOut);
    }
}

contract SegmentBedrockStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    BedrockStrategy public immutable bedrockStrategy;
    SegmentStrategy public immutable segmentStrategy;

    constructor(BedrockStrategy _bedrockStrategy, SegmentStrategy _segmentStrategy) {
        bedrockStrategy = _bedrockStrategy;
        segmentStrategy = _segmentStrategy;
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
        uniBTC.safeIncreaseAllowance(address(segmentStrategy), uniBTCAmount);

        segmentStrategy.handleGatewayMessageWithSlippageArgs(uniBTC, uniBTCAmount, recipient, args);
    }
}

contract SegmentSolvLSTStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    SolvLSTStrategy public immutable solvLSTStrategy;
    SegmentStrategy public immutable segmentStrategy;

    constructor(SolvLSTStrategy _solvLSTStrategy, SegmentStrategy _segmentStrategy) {
        solvLSTStrategy = _solvLSTStrategy;
        segmentStrategy = _segmentStrategy;
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
        solvLST.safeIncreaseAllowance(address(segmentStrategy), solvLSTAmount);

        segmentStrategy.handleGatewayMessageWithSlippageArgs(solvLST, solvLSTAmount, recipient, args);
    }
}
