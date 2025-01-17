// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BitcoinTx} from "../utils/BitcoinTx.sol";
import {ERC2771Recipient} from "@opengsn/packages/contracts/src/ERC2771Recipient.sol";
import {TestLightRelay} from "../relay/TestLightRelay.sol";

using SafeERC20 for IERC20;

contract BtcMarketPlace is ERC2771Recipient {
    using BitcoinTx for TestLightRelay;

    mapping(uint256 => BtcBuyOrder) public btcBuyOrders;
    mapping(uint256 => AcceptedBtcBuyOrder) public acceptedBtcBuyOrders;
    mapping(uint256 => BtcSellOrder) public btcSellOrders;
    mapping(uint256 => AcceptedBtcSellOrder) public acceptedBtcSellOrders;

    uint256 nextOrderId;
    uint256 public constant REQUEST_EXPIRATION_SECONDS = 6 hours;

    TestLightRelay internal testLightRelay;
    uint256 internal txProofDifficultyFactor;

    constructor(TestLightRelay _relay, address erc2771Forwarder) {
        _setTrustedForwarder(erc2771Forwarder);
        setRelay(_relay);
        txProofDifficultyFactor = 1;
    }

    function setRelay(TestLightRelay _relay) internal {
        testLightRelay = _relay;
    }

    // TODO: should we merge buy&sell structs? They're structurally identical except for the
    // bitcoinaddress location.

    event placeBtcSellOrderEvent(uint256 indexed orderId, uint256 amountBtc, address buyingToken, uint256 buyAmount);
    event acceptBtcSellOrderEvent(
        uint256 indexed id,
        uint256 indexed acceptId,
        BitcoinAddress bitcoinAddress,
        uint256 amountBtc,
        uint256 ercAmount,
        address ercToken
    );
    event proofBtcSellOrderEvent(uint256 id);
    event withdrawBtcSellOrderEvent(uint256 id);
    event cancelAcceptedBtcSellOrderEvent(uint256 id);
    event placeBtcBuyOrderEvent(
        uint256 amountBtc, BitcoinAddress bitcoinAddress, address sellingToken, uint256 saleAmount
    );
    event acceptBtcBuyOrderEvent(
        uint256 indexed orderId, uint256 indexed acceptId, uint256 amountBtc, uint256 ercAmount, address ercToken
    );
    event proofBtcBuyOrderEvent(uint256 id);
    event withdrawBtcBuyOrderEvent(uint256 id);
    event cancelAcceptedBtcBuyOrderEvent(uint256 id);

    struct BtcSellOrder {
        uint256 amountBtc;
        address askingToken;
        uint256 askingAmount;
        address requester;
    }

    struct AcceptedBtcSellOrder {
        uint256 orderId;
        BitcoinAddress bitcoinAddress;
        uint256 amountBtc;
        address ercToken;
        uint256 ercAmount;
        address requester;
        address accepter;
        uint256 acceptTime;
    }

    struct BtcBuyOrder {
        uint256 amountBtc;
        BitcoinAddress bitcoinAddress;
        address offeringToken;
        uint256 offeringAmount;
        address requester;
    }

    struct AcceptedBtcBuyOrder {
        uint256 orderId;
        uint256 amountBtc;
        address ercToken;
        uint256 ercAmount;
        address requester;
        address accepter;
        uint256 acceptTime;
    }

    struct BitcoinAddress {
        bytes scriptPubKey;
    }

    struct TransactionProof {
        // todo: fields here
        uint256 dummy;
    }

    function placeBtcSellOrder(uint256 amountBtc, address buyingToken, uint256 buyAmount) public {
        require(buyingToken != address(0x0));
        require(amountBtc > 0);
        require(buyAmount > 0);

        uint256 id = nextOrderId++;
        btcSellOrders[id] = BtcSellOrder({
            amountBtc: amountBtc,
            askingToken: buyingToken,
            askingAmount: buyAmount,
            requester: _msgSender()
        });

        emit placeBtcSellOrderEvent(id, amountBtc, buyingToken, buyAmount);
    }

    function acceptBtcSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress, uint256 amountBtc)
        public
        returns (uint256)
    {
        BtcSellOrder storage order = btcSellOrders[id];

        require(amountBtc > 0);
        require(amountBtc <= order.amountBtc);

        uint256 sellAmount = (amountBtc * order.askingAmount) / order.amountBtc;
        assert(sellAmount > 0);
        assert(order.askingAmount >= sellAmount);
        order.askingAmount -= sellAmount;
        order.amountBtc -= amountBtc;

        // "lock" selling token by transferring to contract
        IERC20(order.askingToken).safeTransferFrom(_msgSender(), address(this), sellAmount);

        uint256 acceptId = nextOrderId++;
        acceptedBtcSellOrders[acceptId] = AcceptedBtcSellOrder({
            orderId: id,
            bitcoinAddress: bitcoinAddress,
            amountBtc: amountBtc,
            ercToken: order.askingToken,
            ercAmount: sellAmount,
            requester: order.requester,
            accepter: _msgSender(),
            acceptTime: block.timestamp
        });

        emit acceptBtcSellOrderEvent(id, acceptId, bitcoinAddress, amountBtc, sellAmount, order.askingToken);

        return acceptId;
    }

    function proofBtcSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
        public
    {
        AcceptedBtcSellOrder storage accept = acceptedBtcSellOrders[id];

        require(accept.requester == _msgSender());

        testLightRelay.setDifficultyFromHeaders(proof.bitcoinHeaders);
        testLightRelay.validateProof(txProofDifficultyFactor, transaction, proof);

        _checkBitcoinTxOutput(accept.amountBtc, accept.bitcoinAddress, transaction);

        IERC20(accept.ercToken).safeTransfer(accept.requester, accept.ercAmount);

        delete acceptedBtcSellOrders[id];
        emit proofBtcSellOrderEvent(id);
    }

    function withdrawBtcSellOrder(uint256 id) public {
        BtcSellOrder storage order = btcSellOrders[id];

        require(order.requester == _msgSender());

        delete btcSellOrders[id];

        emit withdrawBtcSellOrderEvent(id);
    }

    function cancelAcceptedBtcSellOrder(uint256 id) public {
        AcceptedBtcSellOrder storage order = acceptedBtcSellOrders[id];

        require(block.timestamp > order.acceptTime + REQUEST_EXPIRATION_SECONDS);

        require(order.accepter == _msgSender());
        // give accepter its tokens back
        IERC20(order.ercToken).safeTransfer(_msgSender(), order.ercAmount);

        delete acceptedBtcSellOrders[id];

        emit cancelAcceptedBtcSellOrderEvent(id);
    }

    function placeBtcBuyOrder(
        uint256 amountBtc,
        BitcoinAddress calldata bitcoinAddress,
        address sellingToken,
        uint256 saleAmount
    ) public {
        require(sellingToken != address(0x0));

        // "lock" selling token by transferring to contract
        IERC20(sellingToken).safeTransferFrom(_msgSender(), address(this), saleAmount);

        uint256 id = nextOrderId++;
        btcBuyOrders[id] = BtcBuyOrder({
            amountBtc: amountBtc,
            bitcoinAddress: bitcoinAddress,
            offeringToken: sellingToken,
            offeringAmount: saleAmount,
            requester: _msgSender()
        });

        emit placeBtcBuyOrderEvent(amountBtc, bitcoinAddress, sellingToken, saleAmount);
    }

    function acceptBtcBuyOrder(uint256 id, uint256 amountBtc) public returns (uint256) {
        BtcBuyOrder storage order = btcBuyOrders[id];

        require(amountBtc <= order.amountBtc);
        require(amountBtc > 0);

        // todo: make safe
        uint256 buyAmount = (amountBtc * order.offeringAmount) / order.amountBtc;

        assert(buyAmount > 0);

        assert(order.offeringAmount >= buyAmount);
        order.offeringAmount -= buyAmount;
        order.amountBtc -= amountBtc;

        AcceptedBtcBuyOrder memory accept = AcceptedBtcBuyOrder({
            orderId: id,
            amountBtc: amountBtc,
            ercToken: order.offeringToken,
            ercAmount: buyAmount,
            requester: order.requester,
            accepter: _msgSender(),
            acceptTime: block.timestamp
        });

        uint256 acceptId = nextOrderId++;

        acceptedBtcBuyOrders[acceptId] = accept;

        emit acceptBtcBuyOrderEvent(id, acceptId, amountBtc, buyAmount, order.offeringToken);

        return acceptId;
    }

    function proofBtcBuyOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof) public {
        AcceptedBtcBuyOrder storage accept = acceptedBtcBuyOrders[id];

        require(accept.accepter == _msgSender());

        testLightRelay.setDifficultyFromHeaders(proof.bitcoinHeaders);
        testLightRelay.validateProof(txProofDifficultyFactor, transaction, proof);

        BtcBuyOrder storage order = btcBuyOrders[accept.orderId];
        _checkBitcoinTxOutput(order.amountBtc, order.bitcoinAddress, transaction);

        IERC20(accept.ercToken).safeTransfer(accept.accepter, accept.ercAmount);

        delete acceptedBtcBuyOrders[id];

        emit proofBtcBuyOrderEvent(id);
    }

    function withdrawBtcBuyOrder(uint256 id) public {
        BtcBuyOrder storage order = btcBuyOrders[id];

        require(order.requester == _msgSender());

        // release the locked erc20s
        IERC20(order.offeringToken).safeTransfer(_msgSender(), order.offeringAmount);

        delete btcBuyOrders[id];

        emit withdrawBtcBuyOrderEvent(id);
    }

    function cancelAcceptedBtcBuyOrder(uint256 id) public {
        AcceptedBtcBuyOrder storage accept = acceptedBtcBuyOrders[id];

        require(accept.requester == _msgSender());

        require(block.timestamp > accept.acceptTime + REQUEST_EXPIRATION_SECONDS);

        // release the locked erc20s
        IERC20(accept.ercToken).safeTransfer(_msgSender(), accept.ercAmount);

        // note: we don't make the accepted amount available for new trades but if we want to,
        // we could implement that

        delete acceptedBtcBuyOrders[id];

        emit cancelAcceptedBtcBuyOrderEvent(id);
    }

    function getOpenBtcSellOrders() external view returns (BtcSellOrder[] memory, uint256[] memory) {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (btcSellOrders[i].requester != address(0x0)) {
                numOpenOrders++;
            }
        }

        BtcSellOrder[] memory ret = new BtcSellOrder[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (btcSellOrders[i].requester != address(0x0)) {
                ret[numPushed] = btcSellOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenAcceptedBtcSellOrders() external view returns (AcceptedBtcSellOrder[] memory, uint256[] memory) {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (acceptedBtcSellOrders[i].amountBtc > 0) {
                numOpenOrders++;
            }
        }

        AcceptedBtcSellOrder[] memory ret = new AcceptedBtcSellOrder[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (acceptedBtcSellOrders[i].amountBtc > 0) {
                ret[numPushed] = acceptedBtcSellOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenBtcBuyOrders() external view returns (BtcBuyOrder[] memory, uint256[] memory) {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (btcBuyOrders[i].requester != address(0x0)) {
                numOpenOrders++;
            }
        }

        BtcBuyOrder[] memory ret = new BtcBuyOrder[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (btcBuyOrders[i].requester != address(0x0)) {
                ret[numPushed] = btcBuyOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenAcceptedBtcBuyOrders() external view returns (AcceptedBtcBuyOrder[] memory, uint256[] memory) {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (acceptedBtcBuyOrders[i].amountBtc > 0) {
                numOpenOrders++;
            }
        }

        AcceptedBtcBuyOrder[] memory ret = new AcceptedBtcBuyOrder[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (acceptedBtcBuyOrders[i].amountBtc > 0) {
                ret[numPushed] = acceptedBtcBuyOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    /**
     * Checks output script pubkey (recipient address) and amount.
     * Reverts if transaction amount is lower or bitcoin address is not found.
     *
     * @param expectedBtcAmount BTC amount requested in order.
     * @param bitcoinAddress Recipient's bitcoin address.
     * @param transaction Transaction fulfilling the order.
     */
    function _checkBitcoinTxOutput(
        uint256 expectedBtcAmount,
        BitcoinAddress storage bitcoinAddress,
        BitcoinTx.Info calldata transaction
    ) private view {
        // Prefixes scriptpubkey with its size to match script output data.
        bytes32 scriptPubKeyHash =
            keccak256(abi.encodePacked(uint8(bitcoinAddress.scriptPubKey.length), bitcoinAddress.scriptPubKey));

        uint256 txOutputValue = BitcoinTx.processTxOutputs(transaction.outputVector, scriptPubKeyHash).value;

        require(txOutputValue >= expectedBtcAmount, "Bitcoin transaction amount is lower than in accepted order.");
    }
}
