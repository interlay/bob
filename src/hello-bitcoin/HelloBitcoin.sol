pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BitcoinTx} from "../bridge/BitcoinTx.sol";
import {IRelay} from "../bridge/IRelay.sol";
import {TestLightRelay} from "../relay/TestLightRelay.sol";
import {BridgeState} from "../bridge/BridgeState.sol";
import "forge-std/console.sol";

using SafeERC20 for IERC20;

contract HelloBitcoin {
    using BitcoinTx for BridgeState.Storage;

    /**
     * @dev Mapping to store BTC to USDT swap orders based on their unique identifiers.
     * Each order is associated with a unique ID, and the order details are stored in the BtcSellOrder struct.
     */
    mapping(uint256 => BtcSellOrder) public btcSellOrders;

    /**
     * @dev Mapping to store ordinal sell orders for swapping BTC to USDT based on their unique identifiers.
     * Each ordinal sell order is associated with a unique ID, and the order details are stored in the OrdinalSellOrder struct.
     */
    mapping(uint256 => OrdinalSellOrder) public ordinalSellOrders;

    /**
     * @dev The address of the ERC-20 contract. You can use this variable for any ERC-20 token,
     * not just USDT (Tether). Make sure to set this to the appropriate ERC-20 contract address.
     */
    IERC20 public usdtContractAddress;

    /**
     * @dev Counter for generating unique identifiers for BTC to USDT swap orders.
     * The `nextBtcOrderId` is incremented each time a new BTC to USDT swap order is created,
     * ensuring that each order has a unique identifier.
     */
    uint256 nextBtcOrderId;

    /**
     * @dev Counter for generating unique identifiers for ordinal sell orders.
     * The `nextOrdinalOrderId` is incremented each time a new ordinal sell order is created,
     * ensuring that each ordinal order has a unique identifier.
     */
    uint256 nextOrdinalOrderId;

    /**
     * @dev Struct representing a BTC to USDT swap order.
     */
    struct BtcSellOrder {
        uint256 sellAmountBtc; // Amount of BTC to be sold in the order.
        uint256 buyAmount; // Amount of USDT (or other ERC20) to be bought in the order.
        address btcSeller; // Address of the seller initiating the order.
        BitcoinAddress btcBuyer; // Bitcoin address of the buyer (initialized with an empty scriptPubKey).
        bool isOrderAccepted; // Flag indicating whether the order has been accepted.
    }

    /**
     * @dev Struct representing an ordinal sell order for swapping Ordinal to USDT.
     */
    struct OrdinalSellOrder {
        OrdinalId ordinalID; // Unique identifier for the ordinal sell order.
        uint256 buyAmount; // Amount of USDT (or other ERC20) to be bought in the order.
        BitcoinTx.UTXO utxo; // UTXO associated with the BTC to USDT swap order.
        address ordinalSeller; // Address of the seller initiating the ordinal order.
        BitcoinAddress ordinalBuyer; // Bitcoin address of the buyer (initialized with an empty scriptPubKey).
        bool isOrderAccepted; // Flag indicating whether the ordinal order has been accepted.
    }

    /**
     * @dev Struct representing a unique identifier for an ordinal sell order.
     */
    struct OrdinalId {
        bytes32 txId; // Transaction ID associated with the ordinal order.
        uint32 index; // Index associated with the ordinal order.
    }

    /**
     * @dev Struct representing a Bitcoin address with a scriptPubKey.
     */
    struct BitcoinAddress {
        bytes scriptPubKey; // Script public key associated with the Bitcoin address.
    }

    event btcSellOrderSuccessfullyPlaced(uint256 indexed orderId, uint256 sellAmountBtc, uint256 buyAmount);
    event btcSellOrderBtcSellOrderAccepted(uint256 indexed id, BitcoinAddress bitcoinAddress);
    event btcSuccessfullySendtoDestination(uint256 id);

    event ordinalSellOrderSuccessfullyPlaced(uint256 indexed id, OrdinalId ordinalID, uint256 buyAmount);
    event ordinalSellOrderBtcSellOrderAccepted(uint256 indexed id, BitcoinAddress bitcoinAddress);
    event ordinalSuccessfullySendtoDestination(uint256 id);

    BridgeState.Storage internal relay;
    TestLightRelay internal testLightRelay;

    /**
     * @dev Constructor to initialize the contract with the relay and ERC2771 forwarder.
     * @param _relay The relay contract implementing the IRelay interface.
     * @param _usdtContractAddress The address of the usdt contract.
     */
    // ToDo: add some link to the docs
    constructor(IRelay _relay, address _usdtContractAddress) {
        relay.relay = _relay;
        relay.txProofDifficultyFactor = 1;
        testLightRelay = TestLightRelay(address(relay.relay));
        usdtContractAddress = IERC20(_usdtContractAddress);
    }

    /**
     * @dev Set the relay contract for the bridge.
     * @param _relay The relay contract implementing the IRelay interface.
     */
    function setRelay(IRelay _relay) internal {
        relay.relay = _relay;
    }

    /**
     * @dev Initiates a BTC to USDT swap order.
     * @param sellAmountBtc The amount of BTC to sell.
     * @param buyAmount The amount of USDT to buy.
     */
    function placeBtcSellOrder(uint256 sellAmountBtc, uint256 buyAmount) public {
        require(sellAmountBtc > 0, "Sell amount must be greater than 0");
        require(buyAmount > 0, "Buy amount must be greater than 0");

        uint256 id = nextBtcOrderId++;
        btcSellOrders[id] = BtcSellOrder({
            sellAmountBtc: sellAmountBtc,
            buyAmount: buyAmount,
            btcSeller: msg.sender,
            btcBuyer: BitcoinAddress({scriptPubKey: new bytes(0)}),
            isOrderAccepted: false
        });

        emit btcSellOrderSuccessfullyPlaced(id, sellAmountBtc, buyAmount);
    }

    /**
     * @dev Accepts a BTC to USDT swap order.
     * @param id The ID of the BTC to USDT swap order.
     * @param bitcoinAddress The Bitcoin address of the buyer.
     */
    function acceptBtcSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public {
        BtcSellOrder storage placedOrder = btcSellOrders[id];

        require(placedOrder.isOrderAccepted == false, "Order has already been accepted");

        // "lock" selling token by transferring to contract
        IERC20(usdtContractAddress).safeTransferFrom(msg.sender, address(this), placedOrder.buyAmount);

        placedOrder.btcBuyer = bitcoinAddress;
        placedOrder.isOrderAccepted = true;

        emit btcSellOrderBtcSellOrderAccepted(id, bitcoinAddress);
    }

    /**
     * @dev Completes the BTC to USDT swap by validating the BTC transaction proof and transfer USDT to the seller.
     * @param id The ID of the BTC to USDT swap order.
     * @param transaction Information about the BTC transaction.
     * @param proof Proof of the BTC transaction's inclusion in the Bitcoin blockchain.
     */
    function completeBtcSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
        public
    {
        // Retrieve the accepted order based on the provided ID
        BtcSellOrder storage acceptedOrder = btcSellOrders[id];

        // Ensure that the order has been accepted and the caller is the original seller
        require(acceptedOrder.isOrderAccepted == true, "Order must be accepted");
        require(acceptedOrder.btcSeller == msg.sender, "Only the original seller can provide proof");

        // Set the difficulty of the relay based on the Bitcoin headers in the proof
        testLightRelay.setDifficultyFromHeaders(proof.bitcoinHeaders);

        // Validate the BTC transaction proof using the relay
        relay.validateProof(transaction, proof);

        // Check if the BTC transaction output matches the expected amount and recipient
        _checkBitcoinTxOutput(acceptedOrder.sellAmountBtc, acceptedOrder.btcBuyer, transaction);

        // Transfer the locked USDT to the original seller
        IERC20(usdtContractAddress).safeTransfer(acceptedOrder.btcSeller, acceptedOrder.buyAmount);

        // Remove the order from the mapping since it has been successfully processed
        delete btcSellOrders[id];

        // Emit an event indicating the successful completion of the BTC to USDT swap
        emit btcSuccessfullySendtoDestination(id);
    }

    /**
     * @dev Initiates an ordinal sell order for swapping Ordinal to USDT.
     * @param ordinalID The unique identifier for the ordinal sell order.
     * @param utxo The UTXO (Unspent Transaction Output) associated with the inscription.
     * @param buyAmount The amount of USDT to buy.
     */
    function placeOrdinalSellOrder(OrdinalId calldata ordinalID, BitcoinTx.UTXO calldata utxo, uint256 buyAmount)
        public
    {
        require(buyAmount > 0, "Buying amount should be greater than 0");

        uint256 id = nextOrdinalOrderId++;

        ordinalSellOrders[id] = OrdinalSellOrder({
            ordinalID: ordinalID,
            buyAmount: buyAmount,
            utxo: utxo,
            ordinalSeller: msg.sender,
            isOrderAccepted: false,
            ordinalBuyer: BitcoinAddress({scriptPubKey: new bytes(0)})
        });

        emit ordinalSellOrderSuccessfullyPlaced(id, ordinalID, buyAmount);
    }

    /**
     * @dev Accepts an ordinal sell order for swapping Oridinal to USDT.
     * @param id The ID of the ordinal sell order.
     * @param bitcoinAddress The Bitcoin address of the buyer.
     */
    function acceptOrdinalSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public {
        OrdinalSellOrder storage placedOrder = ordinalSellOrders[id];
        require(placedOrder.isOrderAccepted == false, "Order already accepted");

        // "lock" sell token by transferring to contract
        IERC20(usdtContractAddress).safeTransferFrom(msg.sender, address(this), placedOrder.buyAmount);

        placedOrder.ordinalBuyer = bitcoinAddress;
        placedOrder.isOrderAccepted = true;

        emit ordinalSellOrderBtcSellOrderAccepted(id, bitcoinAddress);
    }

    /**
     * @dev Completes the ordinal sell order by validating the BTC transaction proof,
     * ensuring that the BTC transaction input spends the specified UTXO associated with the ordinal sell order,
     * and transfer USDT to the seller.
     * @param id The ID of the ordinal sell order.
     * @param transaction Information about the BTC transaction.
     * @param proof Proof of the BTC transaction's inclusion in the Bitcoin blockchain.
     */
    function completeOrdinalSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
        public
    {
        OrdinalSellOrder storage acceptedOrder = ordinalSellOrders[id];

        // Ensure that the order has been accepted and the caller is the original seller
        require(acceptedOrder.isOrderAccepted == true, "Order must be accepted");
        require(acceptedOrder.ordinalSeller == msg.sender, "Only the original seller can provide proof");

        // Set the relay difficulty based on the Bitcoin headers in the proof
        testLightRelay.setDifficultyFromHeaders(proof.bitcoinHeaders);

        // Validate the BTC transaction proof using the relay
        relay.validateProof(transaction, proof);

        // Ensure that the BTC transaction input spends the specified UTXO associated with the ordinal sell order
        BitcoinTx.ensureTxInputSpendsUtxo(transaction.inputVector, acceptedOrder.utxo);

        // Check if the BTC transaction output is to the buyer's address
        _checkBitcoinTxOutput(0, acceptedOrder.ordinalBuyer, transaction);

        // ToDo: Check that the correct satoshis are being spent to the buyer's address if needed

        // Transfer the locked USDT to the original seller
        IERC20(usdtContractAddress).safeTransfer(acceptedOrder.ordinalSeller, acceptedOrder.buyAmount);

        // Remove the ordinal sell order from storage as it has been successfully processed
        delete ordinalSellOrders[id];

        // Emit an event to indicate the successful completion of the ordinal sell order
        emit ordinalSuccessfullySendtoDestination(id);
    }

    /**
     * Checks output script pubkey (recipient address) and amount.
     * Reverts if transaction amount is lower or bitcoin address is not found.
     *
     * @param expectedBtcAmount BTC amount requested in order.
     * @param bitcoinAddress Recipient's bitcoin address.
     * @param transaction Transaction fulfilling the order.
     */
    //ToDo: Should we move this into the library.
    function _checkBitcoinTxOutput(
        uint256 expectedBtcAmount,
        BitcoinAddress storage bitcoinAddress,
        BitcoinTx.Info calldata transaction
    ) private {
        // Prefixes scriptpubkey with its size to match script output data.
        bytes32 scriptPubKeyHash =
            keccak256(abi.encodePacked(uint8(bitcoinAddress.scriptPubKey.length), bitcoinAddress.scriptPubKey));

        uint256 txOutputValue = BitcoinTx.getTxOutputValue(scriptPubKeyHash, transaction.outputVector);

        require(txOutputValue >= expectedBtcAmount, "Bitcoin transaction amount is lower than in accepted order.");
    }
}
