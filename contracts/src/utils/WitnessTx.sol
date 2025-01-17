// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.8.17;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {SegWitUtils} from "@bob-collective/bitcoin-spv/SegWitUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import {BitcoinTx} from "./BitcoinTx.sol";
import {IRelay} from "../relay/IRelay.sol";

library WitnessTx {
    using BTCUtils for bytes;
    using BytesLib for bytes;
    using SegWitUtils for bytes;
    using ValidateSPV for bytes32;

    bytes1 constant SEGWIT_MARKER = hex"00";
    bytes1 constant SEGWIT_FLAG = hex"01";

    /// @notice Represents a Bitcoin transaction with the witness data.
    struct WitnessInfo {
        /// @notice Bitcoin transaction info.
        BitcoinTx.Info info;
        /// @notice Bitcoin transaction witness data.
        /// @dev Serialized according to the Bitcoin format.
        bytes witnessVector;
    }

    /// @notice Represents data needed to perform a Bitcoin SPV proof with witness data.
    struct WitnessProof {
        /// @dev From the coinbase witness data.
        bytes32 witnessNonce;
        /// @notice The *witness* merkle root of the payment.
        bytes32 paymentMerkleRoot;
        /// @notice Coinbase proof.
        BitcoinTx.Proof coinbaseProof;
        /// @notice Payment proof.
        BitcoinTx.Proof paymentProof;
        /// @notice Coinbase transaction.
        /// @dev Needed to extract the witness commitment.
        BitcoinTx.Info coinbaseTx;
    }

    /// @notice Validates the SPV proof of the Bitcoin transaction with witness data.
    ///         Reverts in case the validation or proof verification fail.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return wTxHash Proven 32-byte transaction hash.
    function validateWitnessProof(WitnessInfo memory txInfo, WitnessProof memory proof)
        internal
        view
        returns (bytes32 wTxHash)
    {
        require(proof.coinbaseTx.outputVector.validateVout(), "Invalid coinbase output vector provided");
        require(txInfo.info.inputVector.validateVin(), "Invalid payment input vector provided");
        require(txInfo.info.outputVector.validateVout(), "Invalid payment output vector provided");

        bytes32 coinbaseTxHash = abi.encodePacked(
            proof.coinbaseTx.version,
            proof.coinbaseTx.inputVector,
            proof.coinbaseTx.outputVector,
            proof.coinbaseTx.locktime
        ).hash256View();

        require(
            coinbaseTxHash.prove(
                proof.coinbaseProof.bitcoinHeaders.extractMerkleRootLE(),
                proof.coinbaseProof.merkleProof,
                proof.coinbaseProof.txIndexInBlock
            ),
            "Tx merkle proof is not valid for provided header and tx hash"
        );

        bytes32 paymentWTxId = abi.encodePacked(
            txInfo.info.version,
            SEGWIT_MARKER,
            SEGWIT_FLAG,
            txInfo.info.inputVector,
            txInfo.info.outputVector,
            txInfo.witnessVector,
            txInfo.info.locktime
        ).hash256View();

        require(
            paymentWTxId.prove(
                proof.paymentMerkleRoot, proof.paymentProof.merkleProof, proof.paymentProof.txIndexInBlock
            ),
            "Tx witness merkle proof is not valid for provided header and tx hash"
        );

        // witnessCommitment = SHA256(witnessMerkleRoot || witnessNonce)
        bytes32 witnessCommitment = abi.encodePacked(proof.paymentMerkleRoot, proof.witnessNonce).hash256View();

        // extract coinbase commitment from tx out
        bytes32 coinbaseWitnessCommitment = proof.coinbaseTx.outputVector.extractWitnessCommitment();
        require(coinbaseWitnessCommitment == witnessCommitment, "Invalid commitment");

        return paymentWTxId;
    }

    /// @notice Validates the witness SPV proof using the relay.
    /// @param relay Bitcoin relay providing the current Bitcoin network difficulty.
    /// @param txProofDifficultyFactor The number of confirmations required on the Bitcoin chain.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return wTxHash Proven 32-byte transaction hash.
    function validateWitnessProofAndDifficulty(
        IRelay relay,
        uint256 txProofDifficultyFactor,
        WitnessInfo memory txInfo,
        WitnessProof memory proof
    ) internal view returns (bytes32 wTxHash) {
        wTxHash = validateWitnessProof(txInfo, proof);
        BitcoinTx.evaluateProofDifficulty(relay, txProofDifficultyFactor, proof.coinbaseProof.bitcoinHeaders);
    }
}
