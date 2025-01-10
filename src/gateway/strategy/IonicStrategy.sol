// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs} from "../IStrategy.sol";
import {StrategySlippageArgs} from "../CommonStructs.sol";
import {SolvLSTStrategy} from "./SolvStrategy.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Interface for the Ionic Finance token, providing minting, redeeming,
 * and balance query functions.
 */
interface IIonicToken {
    function mint(uint256 mintAmount) external returns (uint256);
    function redeem(uint256 redeemTokens) external returns (uint256);
}

/**
 * @dev Interface for the Ionic Finance Pool, allowing entry and exit from markets.
 */
interface IPool {
    function enterMarkets(address[] memory cTokens) external returns (uint256[] memory);
    function exitMarket(address cTokenAddress) external returns (uint256);
}

/**
 * @title Strategy contract for interacting with Ionic Finance markets.
 * @dev Implements IStrategyWithSlippageArgs and allows the contract to handle tokens with slippage arguments.
 */
contract IonicStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    // Immutable references to the Ionic token and pool interfaces.
    IIonicToken public immutable ioErc20;
    IPool public immutable pool;

    /**
     * @dev Constructor to initialize the Ionic token and pool interfaces.
     * @param _ioErc20 Address of the Ionic token.
     * @param _pool Address of the Ionic pool.
     */
    constructor(IIonicToken _ioErc20, IPool _pool) {
        ioErc20 = _ioErc20;
        pool = _pool;
    }

    /**
     * @dev Handles the transfer and minting of tokens with slippage control.
     * @param tokenSent The token to be transferred.
     * @param amountIn The amount of tokens to be transferred.
     * @param recipient The recipient address.
     * @param args Slippage arguments, including minimum acceptable output amount.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        // Transfer tokens from sender to this contract
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);

        // Approve the Ionic token contract to spend the transferred tokens
        tokenSent.safeIncreaseAllowance(address(ioErc20), amountIn);

        // Get the current balance of the recipient's Ionic token holdings
        IERC20 ionicToken = IERC20(address(ioErc20));
        uint256 amountBefore = ionicToken.balanceOf(recipient);

        // Enter the Ionic market with the specified token
        address[] memory markets = new address[](1);
        markets[0] = address(ioErc20);
        uint256[] memory results = pool.enterMarkets(markets);
        require(results[0] == 0, "Couldn't enter in Market");

        // Mint Ionic tokens with the transferred amount
        uint256 mintResult = ioErc20.mint(amountIn);
        require(mintResult == 0, "Could not mint token in Ionic market");

        // Calculate the amount of Ionic tokens to transfer to the recipient
        uint256 ionicAmountToTransfer = ionicToken.balanceOf(address(this));

        // Increase allowance for the recipient if necessary and transfer Ionic tokens
        ionicToken.safeTransfer(recipient, ionicAmountToTransfer);

        // Confirm the recipient's new balance and validate output amount
        uint256 amountAfter = IERC20(address(ioErc20)).balanceOf(recipient);
        require(amountAfter > amountBefore, "Insufficient supply provided");
        uint256 amountOut = amountAfter - amountBefore;
        require(amountOut >= args.amountOutMin, "Insufficient output amount");

        // Emit an event to record the token output
        emit TokenOutput(address(ionicToken), amountOut);
    }
}
