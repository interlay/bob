# HelloBitcoin
[Git Source](https://github.com/bob-collective/bob/blob/master/src/hello-bitcoin/HelloBitcoin.sol)


## State Variables
### btcSellOrders
*Mapping to store BTC to USDT swap orders based on their unique identifiers.
Each order is associated with a unique ID, and the order details are stored in the BtcSellOrder struct.*


```solidity
mapping(uint256 => BtcSellOrder) public btcSellOrders;
```


### ordinalSellOrders
*Mapping to store ordinal sell orders for swapping BTC to USDT based on their unique identifiers.
Each ordinal sell order is associated with a unique ID, and the order details are stored in the OrdinalSellOrder struct.*


```solidity
mapping(uint256 => OrdinalSellOrder) public ordinalSellOrders;
```


### usdtContractAddress
*The address of the USDT (Tether) ERC-20 contract.*


```solidity
address public usdtContractAddress;
```


### nextOrderId
*Counter for generating unique identifiers for BTC to USDT swap orders.
The `nextOrderId` is incremented each time a new BTC to USDT swap order is created,
ensuring that each order has a unique identifier.*


```solidity
uint256 nextOrderId;
```


### nextOrdinalId
*Counter for generating unique identifiers for ordinal sell orders.
The `nextOrdinalId` is incremented each time a new ordinal sell order is created,
ensuring that each ordinal order has a unique identifier.*


```solidity
uint256 nextOrdinalId;
```


### relay

```solidity
BridgeState.Storage internal relay;
```


### testLightRelay

```solidity
TestLightRelay internal testLightRelay;
```


## Functions
### constructor

*Constructor to initialize the contract with the relay and ERC2771 forwarder.*


```solidity
constructor(IRelay _relay, address setUsdtContractAddress);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_relay`|`IRelay`|The relay contract implementing the IRelay interface.|
|`setUsdtContractAddress`|`address`|The address of the usdt contract.|


### setRelay

*Set the relay contract for the bridge.*


```solidity
function setRelay(IRelay _relay) internal;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_relay`|`IRelay`|The relay contract implementing the IRelay interface.|


### swapBtcForUsdt

*Initiates a BTC to USDT swap order.*


```solidity
function swapBtcForUsdt(uint256 sellAmountBtc, uint256 buyAmountUsdt) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`sellAmountBtc`|`uint256`|The amount of BTC to sell.|
|`buyAmountUsdt`|`uint256`|The amount of USDT to buy.|


### acceptBtcToUsdtSwap

*Accepts a BTC to USDT swap order.*


```solidity
function acceptBtcToUsdtSwap(uint256 id, BitcoinAddress calldata bitcoinAddress) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the BTC to USDT swap order.|
|`bitcoinAddress`|`BitcoinAddress`|The Bitcoin address of the buyer.|


### proofBtcSendtoDestination

*Completes the BTC to USDT swap by validating the BTC transaction proof and transfer USDT to the seller.*


```solidity
function proofBtcSendtoDestination(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
    public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the BTC to USDT swap order.|
|`transaction`|`BitcoinTx.Info`|Information about the BTC transaction.|
|`proof`|`BitcoinTx.Proof`|Proof of the BTC transaction's inclusion in the Bitcoin blockchain.|


### swapOrdinalToUsdt

*Initiates an ordinal sell order for swapping Ordinal to USDT.*


```solidity
function swapOrdinalToUsdt(OrdinalId calldata ordinalID, BitcoinTx.UTXO calldata utxo, uint256 buyAmountUsdt) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`ordinalID`|`OrdinalId`|The unique identifier for the ordinal sell order.|
|`utxo`|`BitcoinTx.UTXO`|The UTXO (Unspent Transaction Output) associated with the inscription.|
|`buyAmountUsdt`|`uint256`|The amount of USDT to buy.|


### acceptOrdinalToUsdtSwap

*Accepts an ordinal sell order for swapping Oridinal to USDT.*


```solidity
function acceptOrdinalToUsdtSwap(uint256 id, BitcoinAddress calldata bitcoinAddress) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the ordinal sell order.|
|`bitcoinAddress`|`BitcoinAddress`|The Bitcoin address of the buyer.|


### proofOrdinalSendtoDestination

*Completes the ordinal sell order by validating the BTC transaction proof,
ensuring that the BTC transaction input spends the specified UTXO associated with the ordinal sell order,
and transfer USDT to the seller.*


```solidity
function proofOrdinalSendtoDestination(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
    public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the ordinal sell order.|
|`transaction`|`BitcoinTx.Info`|Information about the BTC transaction.|
|`proof`|`BitcoinTx.Proof`|Proof of the BTC transaction's inclusion in the Bitcoin blockchain.|


### _checkBitcoinTxOutput

Checks output script pubkey (recipient address) and amount.
Reverts if transaction amount is lower or bitcoin address is not found.


```solidity
function _checkBitcoinTxOutput(
    uint256 expectedBtcAmount,
    BitcoinAddress storage bitcoinAddress,
    BitcoinTx.Info calldata transaction
) private;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`expectedBtcAmount`|`uint256`|BTC amount requested in order.|
|`bitcoinAddress`|`BitcoinAddress`|Recipient's bitcoin address.|
|`transaction`|`BitcoinTx.Info`|Transaction fulfilling the order.|


## Events
### swapBtcForUsdtEvent

```solidity
event swapBtcForUsdtEvent(uint256 indexed orderId, uint256 sellAmountBtc, uint256 buyAmountUsdt);
```

### acceptBtcToUsdtSwapEvent

```solidity
event acceptBtcToUsdtSwapEvent(uint256 indexed id, BitcoinAddress bitcoinAddress);
```

### proofBtcSendtoDestinationEvent

```solidity
event proofBtcSendtoDestinationEvent(uint256 id);
```

### swapOrdinalToUsdtEvent

```solidity
event swapOrdinalToUsdtEvent(uint256 indexed id, OrdinalId ordinalID, uint256 buyAmountUsdt);
```

### acceptOrdinalToUsdtSwapEvent

```solidity
event acceptOrdinalToUsdtSwapEvent(uint256 indexed id, BitcoinAddress bitcoinAddress);
```

### proofOrdinalSellOrderEvent

```solidity
event proofOrdinalSellOrderEvent(uint256 id);
```

## Structs
### BtcSellOrder
*Struct representing a BTC to USDT swap order.*


```solidity
struct BtcSellOrder {
    uint256 sellAmountBtc;
    uint256 buyAmountUsdt;
    address btcSeller;
    BitcoinAddress btcBuyer;
    bool isOrderAccepted;
}
```

### OrdinalSellOrder
*Struct representing an ordinal sell order for swapping BTC to USDT.*


```solidity
struct OrdinalSellOrder {
    OrdinalId ordinalID;
    uint256 buyAmountUsdt;
    BitcoinTx.UTXO utxo;
    address ordinalSeller;
    bool isOrderAccepted;
    BitcoinAddress ordinalBuyer;
}
```

### OrdinalId
*Struct representing a unique identifier for an ordinal sell order.*


```solidity
struct OrdinalId {
    bytes32 txId;
    uint32 index;
}
```

### BitcoinAddress
*Struct representing a Bitcoin address with a scriptPubKey.*


```solidity
struct BitcoinAddress {
    bytes scriptPubKey;
}
```

