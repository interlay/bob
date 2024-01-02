# WitnessTx
[Git Source](https://github.com/bob-collective/bob/blob/master/src/bridge/WitnessTx.sol)


## State Variables
### SEGWIT_MARKER

```solidity
bytes1 constant SEGWIT_MARKER = hex"00";
```


### SEGWIT_FLAG

```solidity
bytes1 constant SEGWIT_FLAG = hex"01";
```


## Functions
### validateWitnessProof

Validates the SPV proof of the Bitcoin transaction with witness data.
Reverts in case the validation or proof verification fail.


```solidity
function validateWitnessProof(WitnessInfo memory txInfo, WitnessProof memory proof)
    internal
    view
    returns (bytes32 wTxHash);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`txInfo`|`WitnessInfo`|Bitcoin transaction data.|
|`proof`|`WitnessProof`|Bitcoin proof data.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`wTxHash`|`bytes32`|Proven 32-byte transaction hash.|


### validateWitnessProofAndDifficulty

Validates the witness SPV proof using the relay.


```solidity
function validateWitnessProofAndDifficulty(
    BridgeState.Storage storage self,
    WitnessInfo memory txInfo,
    WitnessProof memory proof
) internal view returns (bytes32 wTxHash);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`self`|`BridgeState.Storage`||
|`txInfo`|`WitnessInfo`|Bitcoin transaction data.|
|`proof`|`WitnessProof`|Bitcoin proof data.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`wTxHash`|`bytes32`|Proven 32-byte transaction hash.|


## Structs
### WitnessInfo
Represents a Bitcoin transaction with the witness data.


```solidity
struct WitnessInfo {
    BitcoinTx.Info info;
    bytes witnessVector;
}
```

### WitnessProof
Represents data needed to perform a Bitcoin SPV proof with witness data.


```solidity
struct WitnessProof {
    bytes32 witnessNonce;
    bytes32 paymentMerkleRoot;
    BitcoinTx.Proof coinbaseProof;
    BitcoinTx.Proof paymentProof;
    BitcoinTx.Info coinbaseTx;
}
```

