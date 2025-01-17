// SPDX-License-Identifier: GPL-3.0-only

// Forked from https://github.com/keep-network/tbtc-v2

pragma solidity ^0.8.17;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import {IRelay} from "../relay/IRelay.sol";

/// @title Bitcoin transaction
/// @notice Allows to reference Bitcoin raw transaction in Solidity.
/// @dev See https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format
// Raw Bitcoin transaction data:
//      | Bytes  |     Name     |        BTC type        |        Description        |
//      |--------|--------------|------------------------|---------------------------|
//      | 4      | version      | int32_t (LE)           | TX version number         |
//      | varies | tx_in_count  | compactSize uint (LE)  | Number of TX inputs       |
//      | varies | tx_in        | txIn[]                 | TX inputs                 |
//      | varies | tx_out_count | compactSize uint (LE)  | Number of TX outputs      |
//      | varies | tx_out       | txOut[]                | TX outputs                |
//      | 4      | lock_time    | uint32_t (LE)          | Unix time or block number |
//
//      Non-coinbase transaction input (txIn):
//
//      | Bytes  |       Name       |        BTC type        |                 Description                 |
//      |--------|------------------|------------------------|---------------------------------------------|
//      | 36     | previous_output  | outpoint               | The previous outpoint being spent           |
//      | varies | script_bytes     | compactSize uint (LE)  | The number of bytes in the signature script |
//      | varies | signature_script | char[]                 | The signature script, empty for P2WSH       |
//      | 4      | sequence         | uint32_t (LE)          | Sequence number                             |
//
//
//      The reference to transaction being spent (outpoint):
//
//      | Bytes | Name  |   BTC type    |               Description                |
//      |-------|-------|---------------|------------------------------------------|
//      |    32 | hash  | char[32]      | Hash of the transaction to spend         |
//      |    4  | index | uint32_t (LE) | Index of the specific output from the TX |
//
//
//      Transaction output (txOut):
//
//      | Bytes  |      Name       |     BTC type          |             Description              |
//      |--------|-----------------|-----------------------|--------------------------------------|
//      | 8      | value           | int64_t (LE)          | Number of satoshis to spend          |
//      | 1+     | pk_script_bytes | compactSize uint (LE) | Number of bytes in the pubkey script |
//      | varies | pk_script       | char[]                | Pubkey script                        |
//
//      compactSize uint format:
//
//      |                  Value                  | Bytes |                    Format                    |
//      |-----------------------------------------|-------|----------------------------------------------|
//      | >= 0 && <= 252                          | 1     | uint8_t                                      |
//      | >= 253 && <= 0xffff                     | 3     | 0xfd followed by the number as uint16_t (LE) |
//      | >= 0x10000 && <= 0xffffffff             | 5     | 0xfe followed by the number as uint32_t (LE) |
//      | >= 0x100000000 && <= 0xffffffffffffffff | 9     | 0xff followed by the number as uint64_t (LE) |
//
//      (*) compactSize uint is often references as VarInt)
//
//      Coinbase transaction input (txIn):
//
//      | Bytes  |       Name       |        BTC type        |                 Description                 |
//      |--------|------------------|------------------------|---------------------------------------------|
//      | 32     | hash             | char[32]               | A 32-byte 0x0  null (no previous_outpoint)  |
//      | 4      | index            | uint32_t (LE)          | 0xffffffff (no previous_outpoint)           |
//      | varies | script_bytes     | compactSize uint (LE)  | The number of bytes in the coinbase script  |
//      | varies | height           | char[]                 | The block height of this block (BIP34) (*)  |
//      | varies | coinbase_script  | none                   |  Arbitrary data, max 100 bytes              |
//      | 4      | sequence         | uint32_t (LE)          | Sequence number
//
//      (*)  Uses script language: starts with a data-pushing opcode that indicates how many bytes to push to
//           the stack followed by the block height as a little-endian unsigned integer. This script must be as
//           short as possible, otherwise it may be rejected. The data-pushing opcode will be 0x03 and the total
//           size four bytes until block 16,777,216 about 300 years from now.

library BitcoinTx {
    using BTCUtils for bytes;
    using BTCUtils for uint256;
    using BytesLib for bytes;
    using ValidateSPV for bytes;
    using ValidateSPV for bytes32;

    /// @notice Represents Bitcoin transaction data.
    struct Info {
        /// @notice Bitcoin transaction version.
        /// @dev `version` from raw Bitcoin transaction data.
        ///      Encoded as 4-bytes signed integer, little endian.
        bytes4 version;
        /// @notice All Bitcoin transaction inputs, prepended by the number of
        ///         transaction inputs.
        /// @dev `tx_in_count | tx_in` from raw Bitcoin transaction data.
        ///
        ///      The number of transaction inputs encoded as compactSize
        ///      unsigned integer, little-endian.
        ///
        ///      Note that some popular block explorers reverse the order of
        ///      bytes from `outpoint`'s `hash` and display it as big-endian.
        ///      Solidity code expects hashes in little-endian, just like
        ///      they are represented in a raw Bitcoin transaction.
        bytes inputVector;
        /// @notice All Bitcoin transaction outputs prepended by the number of
        ///         transaction outputs.
        /// @dev `tx_out_count | tx_out` from raw Bitcoin transaction data.
        ///
        ///       The number of transaction outputs encoded as a compactSize
        ///       unsigned integer, little-endian.
        bytes outputVector;
        /// @notice Bitcoin transaction locktime.
        ///
        /// @dev `lock_time` from raw Bitcoin transaction data.
        ///      Encoded as 4-bytes unsigned integer, little endian.
        bytes4 locktime;
    }

    /// @notice Represents data needed to perform a Bitcoin SPV proof.
    struct Proof {
        /// @notice The merkle proof of transaction inclusion in a block.
        bytes merkleProof;
        /// @notice Transaction index in the block (0-indexed).
        uint256 txIndexInBlock;
        /// @notice Single byte-string of 80-byte bitcoin headers,
        ///         lowest height first.
        bytes bitcoinHeaders;
    }

    /// @notice Represents info about an unspent transaction output.
    struct UTXO {
        /// @notice Hash of the transaction the output belongs to.
        /// @dev Byte order corresponds to the Bitcoin internal byte order.
        bytes32 txHash;
        /// @notice Index of the transaction output (0-indexed).
        uint32 txOutputIndex;
        /// @notice Value of the transaction output.
        uint64 txOutputValue;
    }

    /// @notice Validates the SPV proof of the Bitcoin transaction.
    ///         Reverts in case the validation or proof verification fail.
    /// @param relay Bitcoin relay providing the current Bitcoin network difficulty.
    /// @param txProofDifficultyFactor The number of confirmations required on the Bitcoin chain.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return txHash Proven 32-byte transaction hash.
    function validateProof(IRelay relay, uint256 txProofDifficultyFactor, Info memory txInfo, Proof memory proof)
        internal
        view
        returns (bytes32 txHash)
    {
        require(txInfo.inputVector.validateVin(), "Invalid input vector provided");
        require(txInfo.outputVector.validateVout(), "Invalid output vector provided");

        txHash =
            abi.encodePacked(txInfo.version, txInfo.inputVector, txInfo.outputVector, txInfo.locktime).hash256View();

        require(
            txHash.prove(proof.bitcoinHeaders.extractMerkleRootLE(), proof.merkleProof, proof.txIndexInBlock),
            "Tx merkle proof is not valid for provided header and tx hash"
        );

        evaluateProofDifficulty(relay, txProofDifficultyFactor, proof.bitcoinHeaders);

        return txHash;
    }

    /// @notice Evaluates the given Bitcoin proof difficulty against the actual
    ///         Bitcoin chain difficulty provided by the relay oracle.
    ///         Reverts in case the evaluation fails.
    /// @param relay Bitcoin relay providing the current Bitcoin network difficulty.
    /// @param txProofDifficultyFactor The number of confirmations required on the Bitcoin chain.
    /// @param bitcoinHeaders Bitcoin headers chain being part of the SPV
    ///        proof. Used to extract the observed proof difficulty.
    function evaluateProofDifficulty(IRelay relay, uint256 txProofDifficultyFactor, bytes memory bitcoinHeaders)
        internal
        view
    {
        uint256 currentEpochDifficulty = relay.getCurrentEpochDifficulty();
        uint256 previousEpochDifficulty = relay.getPrevEpochDifficulty();

        uint256 requestedDiff = 0;
        uint256 firstHeaderDiff = bitcoinHeaders.extractTarget().calculateDifficulty();

        if (firstHeaderDiff == currentEpochDifficulty) {
            requestedDiff = currentEpochDifficulty;
        } else if (firstHeaderDiff == previousEpochDifficulty) {
            requestedDiff = previousEpochDifficulty;
        } else {
            revert("Not at current or previous difficulty");
        }

        uint256 observedDiff = bitcoinHeaders.validateHeaderChain();

        require(observedDiff != ValidateSPV.getErrBadLength(), "Invalid length of the headers chain");
        require(observedDiff != ValidateSPV.getErrInvalidChain(), "Invalid headers chain");
        require(observedDiff != ValidateSPV.getErrLowWork(), "Insufficient work in a header");

        require(
            observedDiff >= requestedDiff * txProofDifficultyFactor,
            "Insufficient accumulated difficulty in header chain"
        );
    }

    /// @notice Represents temporary information needed during the processing of
    ///         the Bitcoin transaction outputs. This structure is an internal one
    ///         and should not be exported outside of the transaction processing code.
    /// @dev Allows to mitigate "stack too deep" errors on EVM.
    struct TxOutputsProcessingInfo {
        // The first output starting index in the transaction.
        uint256 outputStartingIndex;
        // The number of outputs in the transaction.
        uint256 outputsCount;
    }

    /// @notice Represents an outcome of the Bitcoin transaction
    ///         outputs processing.
    struct TxOutputsInfo {
        // Sum of all outputs values spending to the output.
        uint64 value;
        // Optional EVM address specified in OP_RETURN.
        address evmAddress;
        // Optional hash specified in OP_RETURN.
        bytes32 hash;
    }

    /// @notice Processes all outputs from the transaction.
    /// @param txOutputVector Bitcoin transaction output vector. This function
    ///        assumes vector's structure is valid so it must be validated using
    ///        e.g. `BTCUtils.validateVout` function before it is passed here.
    /// @param scriptPubKeyHash Expected Bitcoin scriptPubKey keccak256 hash.
    function processTxOutputs(bytes memory txOutputVector, bytes32 scriptPubKeyHash)
        internal
        pure
        returns (TxOutputsInfo memory resultInfo)
    {
        // needed to avoid stack too deep errors
        TxOutputsProcessingInfo memory processInfo;

        // Determining the total number of transaction outputs in the same way as
        // for number of inputs. See `BitcoinTx.outputVector` docs for more details.
        (processInfo.outputStartingIndex, processInfo.outputsCount) = txOutputVector.parseVarInt();

        // To determine the first output starting index, we must jump over
        // the compactSize uint which prepends the output vector. One byte
        // must be added because `BtcUtils.parseVarInt` does not include
        // compactSize uint tag in the returned length.
        //
        // For >= 0 && <= 252, `BTCUtils.determineVarIntDataLengthAt`
        // returns `0`, so we jump over one byte of compactSize uint.
        //
        // For >= 253 && <= 0xffff there is `0xfd` tag,
        // `BTCUtils.determineVarIntDataLengthAt` returns `2` (no
        // tag byte included) so we need to jump over 1+2 bytes of
        // compactSize uint.
        //
        // Please refer `BTCUtils` library and compactSize uint
        // docs in `BitcoinTx` library for more details.
        processInfo.outputStartingIndex++;

        // Helper flag indicating whether there was at least one
        // output present
        bool outputPresent = false;

        // Outputs processing loop.
        for (uint256 i = 0; i < processInfo.outputsCount; i++) {
            uint256 outputLength = txOutputVector.determineOutputLengthAt(processInfo.outputStartingIndex);

            // Extract the value from given output.
            uint64 outputValue = txOutputVector.extractValueAt(processInfo.outputStartingIndex);

            // The output consists of an 8-byte value and a variable length
            // script. To hash that script we slice the output starting from
            // 9th byte until the end.
            uint256 scriptLength = outputLength - 8;
            uint256 outputScriptStart = processInfo.outputStartingIndex + 8;

            bytes32 outputScriptHash;
            /* solhint-disable-next-line no-inline-assembly */
            assembly {
                // The first argument to assembly keccak256 is the pointer.
                // We point to `txOutputVector` but at the position indicated
                // by `outputScriptStart`. To load that position, we
                // need to call `add(outputScriptStart, 32)` because
                // `outputScriptStart` has 32 bytes.
                outputScriptHash := keccak256(add(txOutputVector, add(outputScriptStart, 32)), scriptLength)
            }

            if (scriptPubKeyHash == outputScriptHash) {
                outputPresent = true;
                // Accumulate the total in case there are multiple
                // payments to the same output
                resultInfo.value += outputValue;
            } else {
                address outputEvmAddress = extractEvmAddressFromOutput(txOutputVector, processInfo.outputStartingIndex);
                if (outputEvmAddress != address(0)) {
                    // NOTE: this will overwrite if there are multiple OP_RETURN outputs
                    resultInfo.evmAddress = outputEvmAddress;
                }

                bytes32 outputHash = extractHashFromOutput(txOutputVector, processInfo.outputStartingIndex);
                if (outputHash != 0) {
                    // NOTE: this will overwrite if there are multiple OP_RETURN outputs
                    resultInfo.hash = outputHash;
                }
            }

            // Make the `outputStartingIndex` pointing to the next output by
            // increasing it by current output's length.
            processInfo.outputStartingIndex += outputLength;
        }

        require(outputPresent, "No output found for scriptPubKey");
    }

    function extractEvmAddressFromOutput(bytes memory _output, uint256 _at)
        internal
        pure
        returns (address evmAddress)
    {
        // OP_RETURN
        if (_output[_at + 9] != hex"6a") {
            return address(0);
        }
        bytes1 _dataLen = _output[_at + 10];
        if (uint256(uint8(_dataLen)) == 20) {
            uint256 opReturnStart = _at + 11;
            assembly {
                evmAddress := mload(add(_output, add(opReturnStart, 20)))
            }
        }
    }

    function extractHashFromOutput(bytes memory _output, uint256 _at) internal pure returns (bytes32 outputHash) {
        // OP_RETURN
        if (_output[_at + 9] != hex"6a") {
            return 0;
        }
        bytes1 _dataLen = _output[_at + 10];
        if (uint256(uint8(_dataLen)) == 32) {
            uint256 opReturnStart = _at + 11;
            assembly {
                outputHash := mload(add(_output, add(opReturnStart, 32)))
            }
        }
    }

    function reverseEndianness(bytes32 b) internal pure returns (bytes32 txHash) {
        bytes memory newValue = new bytes(b.length);
        for (uint256 i = 0; i < b.length; i++) {
            newValue[b.length - i - 1] = b[i];
        }
        assembly {
            txHash := mload(add(newValue, 32))
        }
    }

    function ensureTxInputSpendsUtxo(bytes memory _vin, BitcoinTx.UTXO memory utxo) internal pure {
        uint256 _varIntDataLen;
        uint256 _nIns;

        (_varIntDataLen, _nIns) = BTCUtils.parseVarInt(_vin);
        require(_varIntDataLen != BTCUtils.ERR_BAD_ARG, "Read overrun during VarInt parsing");

        uint256 _len = 0;
        uint256 _offset = 1 + _varIntDataLen;

        bytes32 expectedTxHash = reverseEndianness(utxo.txHash);

        for (uint256 _i = 0; _i < _nIns; _i++) {
            bytes32 outpointTxHash = _vin.extractInputTxIdLeAt(_offset);
            uint32 outpointIndex = BTCUtils.reverseUint32(uint32(_vin.extractTxIndexLeAt(_offset)));

            // check if it matches tx
            if (expectedTxHash == outpointTxHash && utxo.txOutputIndex == outpointIndex) {
                return;
            }

            _len = BTCUtils.determineInputLengthAt(_vin, _offset);
            require(_len != BTCUtils.ERR_BAD_ARG, "Bad VarInt in scriptSig");
            _offset = _offset + _len;
        }

        revert("Transaction does not spend the required utxo");
    }
}
