# BitcoinTx
[Git Source](https://github.com/bob-collective/bob/blob/master/src/bridge/BitcoinTx.sol)

Allows to reference Bitcoin raw transaction in Solidity.

*See https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format*


## Functions
### validateProof

Validates the SPV proof of the Bitcoin transaction.
Reverts in case the validation or proof verification fail.


```solidity
function validateProof(BridgeState.Storage storage self, Info memory txInfo, Proof memory proof)
    internal
    view
    returns (bytes32 txHash);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`self`|`BridgeState.Storage`||
|`txInfo`|`Info`|Bitcoin transaction data.|
|`proof`|`Proof`|Bitcoin proof data.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`txHash`|`bytes32`|Proven 32-byte transaction hash.|


### evaluateProofDifficulty

Evaluates the given Bitcoin proof difficulty against the actual
Bitcoin chain difficulty provided by the relay oracle.
Reverts in case the evaluation fails.


```solidity
function evaluateProofDifficulty(BridgeState.Storage storage self, bytes memory bitcoinHeaders) internal view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`self`|`BridgeState.Storage`||
|`bitcoinHeaders`|`bytes`|Bitcoin headers chain being part of the SPV proof. Used to extract the observed proof difficulty.|


### getTxOutputValue

Processes the Bitcoin transaction output vector.


```solidity
function getTxOutputValue(bytes32 scriptPubKeyHash, bytes memory txOutputVector) internal pure returns (uint64 value);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`scriptPubKeyHash`|`bytes32`|Expected Bitcoin scriptPubKey keccak256 hash.|
|`txOutputVector`|`bytes`|Bitcoin transaction output vector. This function assumes vector's structure is valid so it must be validated using e.g. `BTCUtils.validateVout` function before it is passed here.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`value`|`uint64`|Outcomes of the processing.|


### getTxOutputValue

Processes all outputs from the transaction.


```solidity
function getTxOutputValue(
    bytes32 scriptPubKeyHash,
    bytes memory txOutputVector,
    TxOutputsProcessingInfo memory processInfo
) internal pure returns (uint64 value);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`scriptPubKeyHash`|`bytes32`|Expected Bitcoin scriptPubKey keccak256 hash.|
|`txOutputVector`|`bytes`|Bitcoin transaction output vector. This function assumes vector's structure is valid so it must be validated using e.g. `BTCUtils.validateVout` function before it is passed here.|
|`processInfo`|`TxOutputsProcessingInfo`|TxOutputsProcessingInfo identifying output starting index and the number of outputs.|


### reverseEndianness


```solidity
function reverseEndianness(bytes32 b) internal pure returns (bytes32 txHash);
```

### ensureTxInputSpendsUtxo


```solidity
function ensureTxInputSpendsUtxo(bytes memory _vin, BitcoinTx.UTXO memory utxo) internal pure;
```

## Structs
### Info
Represents Bitcoin transaction data.


```solidity
struct Info {
    bytes4 version;
    bytes inputVector;
    bytes outputVector;
    bytes4 locktime;
}
```

### Proof
Represents data needed to perform a Bitcoin SPV proof.


```solidity
struct Proof {
    bytes merkleProof;
    uint256 txIndexInBlock;
    bytes bitcoinHeaders;
}
```

### UTXO
Represents info about an unspent transaction output.


```solidity
struct UTXO {
    bytes32 txHash;
    uint32 txOutputIndex;
    uint64 txOutputValue;
}
```

### TxOutputsProcessingInfo
Represents temporary information needed during the processing of
the Bitcoin transaction outputs. This structure is an internal one
and should not be exported outside of the transaction processing code.

*Allows to mitigate "stack too deep" errors on EVM.*


```solidity
struct TxOutputsProcessingInfo {
    uint256 outputStartingIndex;
    uint256 outputsCount;
}
```

