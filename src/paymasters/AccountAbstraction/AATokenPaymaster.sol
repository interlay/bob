// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

// adapted from https://github.com/pimlicolabs/erc20-paymaster-contracts/blob/60c3ea1c2d75069f9aa7f8d35bc08545a2edb34b/src/PimlicoERC20Paymaster.sol
// Import the required libraries and contracts
import "@account-abstraction/contracts/core/BasePaymaster.sol";
import "@account-abstraction/contracts/core/Helpers.sol";
import "@account-abstraction/contracts/interfaces/UserOperation.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
// import "./interfaces/IOracle.sol";
import "@account-abstraction/contracts/core/EntryPoint.sol";
// import "./utils/SafeTransferLib.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import {IOracle} from "../Oracle.sol";
// import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
// import "./SafeTransferLib.sol";

// using SafeERC20 for IERC20;

/// @title PimlicoERC20Paymaster
/// @notice An ERC-4337 Paymaster contract by Pimlico which is able to sponsor gas fees in exchange for ERC20 tokens.
/// The contract refunds excess tokens if the actual gas cost is lower than the initially provided amount.
/// It also allows updating price configuration and withdrawing tokens by the contract owner.
/// The contract uses an Oracle to fetch the latest token prices.
/// @dev Inherits from BasePaymaster.

contract PimlicoERC20Paymaster is BasePaymaster {
    uint256 public constant priceDenominator = 1e6;
    uint256 public constant REFUND_POSTOP_COST = 40000; // Estimated gas cost for refunding tokens after the transaction is completed

    // The token, tokenOracle, and nativeAssetOracle are declared as immutable,
    // meaning their values cannot change after contract creation.
    IERC20 public immutable token; // The ERC20 token used for transaction fee payments
    uint256 public immutable tokenDecimals;
    IOracle public immutable tokenOracle; // The Oracle contract used to fetch the latest token prices
    IOracle public immutable nativeAssetOracle; // The Oracle contract used to fetch the latest ETH prices

    uint192 public previousPrice; // The cached token price from the Oracle
    uint32 public priceMarkup; // The price markup percentage applied to the token price (1e6 = 100%)
    uint32 public priceUpdateThreshold; // The price update threshold percentage that triggers a price update (1e6 = 100%)

    event ConfigUpdated(uint32 priceMarkup, uint32 updateThreshold);

    event UserOperationSponsored(address indexed user, uint256 actualTokenNeeded, uint256 actualGasCost);

    /// @notice Initializes the PimlicoERC20Paymaster contract with the given parameters.
    /// @param _token The ERC20 token used for transaction fee payments.
    /// @param _entryPoint The EntryPoint contract used in the Account Abstraction infrastructure.
    /// @param _tokenOracle The Oracle contract used to fetch the latest token prices.
    /// @param _nativeAssetOracle The Oracle contract used to fetch the latest native asset (ETH, Matic, Avax, etc.) prices.
    /// @param _owner The address that will be set as the owner of the contract.
    constructor(
        IERC20 _token,
        IEntryPoint _entryPoint,
        IOracle _tokenOracle,
        IOracle _nativeAssetOracle,
        address _owner,
        uint8 _tokenDecimals
    ) BasePaymaster(_entryPoint) {
        token = _token;
        tokenOracle = _tokenOracle; // oracle for token -> usd
        nativeAssetOracle = _nativeAssetOracle; // oracle for native asset(eth/matic/avax..) -> usd
        priceMarkup = 110e4; // 110%  1e6 = 100%
        priceUpdateThreshold = 25e3; // 2.5%  1e6 = 100%
        transferOwnership(_owner);
        tokenDecimals = 10 ** _tokenDecimals;
        require(_tokenOracle.decimals() == 8, "PP-ERC20 : token oracle decimals must be 8");
        require(_nativeAssetOracle.decimals() == 8, "PP-ERC20 : native asset oracle decimals must be 8");
    }

    /// @notice Updates the price markup and price update threshold configurations.
    /// @param _priceMarkup The new price markup percentage (1e6 = 100%).
    /// @param _updateThreshold The new price update threshold percentage (1e6 = 100%).
    function updateConfig(uint32 _priceMarkup, uint32 _updateThreshold) external onlyOwner {
        require(_priceMarkup <= 120e4, "PP-ERC20 : price markup too high");
        require(_priceMarkup >= 1e6, "PP-ERC20 : price markeup too low");
        require(_updateThreshold <= 1e6, "PP-ERC20 : update threshold too high");
        priceMarkup = _priceMarkup;
        priceUpdateThreshold = _updateThreshold;
        emit ConfigUpdated(_priceMarkup, _updateThreshold);
    }

    /// @notice Allows the contract owner to withdraw a specified amount of tokens from the contract.
    /// @param to The address to transfer the tokens to.
    /// @param amount The amount of tokens to transfer.
    function withdrawToken(address to, uint256 amount) external onlyOwner {
        token.transfer(to, amount);
    }

    /// @notice Updates the token price by fetching the latest price from the Oracle.
    function updatePrice() external {
        // This function updates the cached ERC20/ETH price ratio
        uint192 tokenPrice = fetchPrice(tokenOracle);
        uint192 nativeAssetPrice = fetchPrice(nativeAssetOracle);
        previousPrice = (nativeAssetPrice * uint192(tokenDecimals)) / tokenPrice;
    }

    /// @notice Validates a paymaster user operation and calculates the required token amount for the transaction.
    /// @param userOp The user operation data.
    /// @param requiredPreFund The amount of tokens required for pre-funding.
    /// @return context The context containing the token amount and user sender address (if applicable).
    /// @return validationResult A uint256 value indicating the result of the validation (always 0 in this implementation).
    function _validatePaymasterUserOp(UserOperation calldata userOp, bytes32, uint256 requiredPreFund)
        internal
        override
        returns (bytes memory context, uint256 validationResult)
    {
        uint256 cachedPrice = previousPrice;
        require(cachedPrice != 0, "PP-ERC20 : price not set");
        uint256 length = userOp.paymasterAndData.length - 20;
        // 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdf is the mask for the last 6 bits 011111 which mean length should be 100000(32) || 000000(0)
        require(
            length & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdf == 0,
            "PP-ERC20 : invalid data length"
        );
        // NOTE: we assumed that nativeAsset's decimals is 18, if there is any nativeAsset with different decimals, need to change the 1e18 to the correct decimals
        uint256 tokenAmount = (
            (requiredPreFund + (REFUND_POSTOP_COST) * userOp.maxFeePerGas) * priceMarkup * cachedPrice
        ) / (1e18 * priceDenominator);
        if (length == 32) {
            require(tokenAmount <= uint256(bytes32(userOp.paymasterAndData[20:52])), "PP-ERC20 : token amount too high");
        }
        token.transferFrom(userOp.sender, address(this), tokenAmount);
        context = abi.encodePacked(tokenAmount, userOp.sender);
        // No return here since validationData == 0 and we have context saved in memory
        validationResult = 0;
    }

    /// @notice Performs post-operation tasks, such as updating the token price and refunding excess tokens.
    /// @dev This function is called after a user operation has been executed or reverted.
    /// @param mode The post-operation mode (either successful or reverted).
    /// @param context The context containing the token amount and user sender address.
    /// @param actualGasCost The actual gas cost of the transaction.
    function _postOp(PostOpMode mode, bytes calldata context, uint256 actualGasCost) internal override {
        if (mode == PostOpMode.postOpReverted) {
            return; // Do nothing here to not revert the whole bundle and harm reputation
        }
        uint192 tokenPrice = fetchPrice(tokenOracle);
        uint192 nativeAsset = fetchPrice(nativeAssetOracle);
        uint256 cachedPrice = previousPrice;
        uint192 price = (nativeAsset * uint192(tokenDecimals)) / tokenPrice;
        uint256 cachedUpdateThreshold = priceUpdateThreshold;
        if (
            (uint256(price) * priceDenominator) / cachedPrice > priceDenominator + cachedUpdateThreshold
                || (uint256(price) * priceDenominator) / cachedPrice < priceDenominator - cachedUpdateThreshold
        ) {
            previousPrice = uint192(int192(price));
            cachedPrice = uint192(int192(price));
        }
        // Refund tokens based on actual gas cost
        // NOTE: we assumed that nativeAsset's decimals is 18, if there is any nativeAsset with different decimals, need to change the 1e18 to the correct decimals
        uint256 actualTokenNeeded =
            ((actualGasCost + REFUND_POSTOP_COST * tx.gasprice) * priceMarkup * cachedPrice) / (1e18 * priceDenominator); // We use tx.gasprice here since we don't know the actual gas price used by the user
        if (uint256(bytes32(context[0:32])) > actualTokenNeeded) {
            // If the initially provided token amount is greater than the actual amount needed, refund the difference
            token.transfer(address(bytes20(context[32:52])), uint256(bytes32(context[0:32])) - actualTokenNeeded);
        } // If the token amount is not greater than the actual amount needed, no refund occurs

        emit UserOperationSponsored(address(bytes20(context[32:52])), actualTokenNeeded, actualGasCost);
    }

    /// @notice Fetches the latest price from the given Oracle.
    /// @dev This function is used to get the latest price from the tokenOracle or nativeAssetOracle.
    /// @param _oracle The Oracle contract to fetch the price from.
    /// @return price The latest price fetched from the Oracle.
    function fetchPrice(IOracle _oracle) internal view returns (uint192 price) {
        (uint80 roundId, int256 answer,, uint256 updatedAt, uint80 answeredInRound) = _oracle.latestRoundData();
        require(answer > 0, "PP-ERC20 : Chainlink price <= 0");
        // 2 days old price is considered stale since the price is updated every 24 hours
        require(updatedAt >= block.timestamp - 60 * 60 * 24 * 2, "PP-ERC20 : Incomplete round");
        require(answeredInRound >= roundId, "PP-ERC20 : Stale price");
        price = uint192(int192(answer));
    }
}
