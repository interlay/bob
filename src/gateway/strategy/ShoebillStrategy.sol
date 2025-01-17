// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs} from "../IStrategy.sol";
import {StrategySlippageArgs} from "../CommonStructs.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Shoebill ABI for their sbTokens.
 */
interface ICErc20 {
    function mint(uint256 mintAmount) external returns (uint256);
    function balanceOfUnderlying(address owner) external returns (uint256);
}

/**
 * @title Strategy contract for Shoebill Finance.
 */
contract ShoebillStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ICErc20 public immutable cErc20;

    constructor(ICErc20 _cErc20) {
        cErc20 = _cErc20;
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
        tokenSent.safeIncreaseAllowance(address(cErc20), amountIn);

        IERC20 token = IERC20(address(cErc20));
        uint256 err = cErc20.mint(amountIn);
        require(err == 0, "Could not mint token");
        uint256 amountOut = token.balanceOf(address(this));
        require(amountOut >= args.amountOutMin, "Insufficient output amount");

        token.safeTransfer(recipient, amountOut);

        emit TokenOutput(address(token), amountOut);
    }
}
