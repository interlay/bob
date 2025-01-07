// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs} from "../IStrategy.sol";
import {StrategySlippageArgs} from "../CommonStructs.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Bedrock ABI for their Vault.
 */
interface IBedrockVault {
    function mint(address _token, uint256 _amount) external;
    function redeem(address _token, uint256 _amount) external;
    function uniBTC() external view returns (address);
}

/**
 * @title Strategy contract for Bedrock (uniBTC).
 */
contract BedrockStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    IBedrockVault public immutable vault;

    constructor(IBedrockVault _vault) {
        vault = _vault;
    }

    /**
     * @notice Deposits tokens into Bedrock to mint uniBTC.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive uniBTC.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(vault), amountIn);

        vault.mint(address(tokenSent), amountIn);
        IERC20 uniBTC = IERC20(vault.uniBTC());
        uint256 uniBTCAmount = uniBTC.balanceOf(address(this));
        require(uniBTCAmount >= args.amountOutMin, "Insufficient output amount");
        // ToDo: Missing corner case to check Insufficient supply provided.
        uniBTC.safeTransfer(recipient, uniBTCAmount);

        emit TokenOutput(address(uniBTC), uniBTCAmount);
    }
}
