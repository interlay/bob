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
*The address of the ERC-20 contract. You can use this variable for any ERC-20 token,
not just USDT (Tether). Make sure to set this to the appropriate ERC-20 contract address.*


```solidity
IERC20 public usdtContractAddress;
```


### nextBtcOrderId
*Counter for generating unique identifiers for BTC to USDT swap orders.
The `nextBtcOrderId` is incremented each time a new BTC to USDT swap order is created,
ensuring that each order has a unique identifier.*


```solidity
uint256 nextBtcOrderId;
```


### nextOrdinalOrderId
*Counter for generating unique identifiers for ordinal sell orders.
The `nextOrdinalOrderId` is incremented each time a new ordinal sell order is created,
ensuring that each ordinal order has a unique identifier.*


```solidity
uint256 nextOrdinalOrderId;
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
constructor(IRelay _relay, address _usdtContractAddress);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_relay`|`IRelay`|The relay contract implementing the IRelay interface.|
|`_usdtContractAddress`|`address`|The address of the usdt contract.|


### setRelay

*Set the relay contract for the bridge.*


```solidity
function setRelay(IRelay _relay) internal;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_relay`|`IRelay`|The relay contract implementing the IRelay interface.|


### placeBtcSellOrder

*Initiates a BTC to USDT swap order.*


```solidity
function placeBtcSellOrder(uint256 sellAmountBtc, uint256 buyAmount) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`sellAmountBtc`|`uint256`|The amount of BTC to sell.|
|`buyAmount`|`uint256`|The amount of USDT to buy.|


### acceptBtcSellOrder

*Accepts a BTC to USDT swap order.*


```solidity
function acceptBtcSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the BTC to USDT swap order.|
|`bitcoinAddress`|`BitcoinAddress`|The Bitcoin address of the buyer.|


### completeBtcSellOrder

*Completes the BTC to USDT swap by validating the BTC transaction proof and transfer USDT to the seller.*


```solidity
function completeBtcSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the BTC to USDT swap order.|
|`transaction`|`BitcoinTx.Info`|Information about the BTC transaction.|
|`proof`|`BitcoinTx.Proof`|Proof of the BTC transaction's inclusion in the Bitcoin blockchain.|


### placeOrdinalSellOrder

*Initiates an ordinal sell order for swapping Ordinal to USDT.*


```solidity
function placeOrdinalSellOrder(OrdinalId calldata ordinalID, BitcoinTx.UTXO calldata utxo, uint256 buyAmount) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`ordinalID`|`OrdinalId`|The unique identifier for the ordinal sell order.|
|`utxo`|`BitcoinTx.UTXO`|The UTXO (Unspent Transaction Output) associated with the inscription.|
|`buyAmount`|`uint256`|The amount of USDT to buy.|


### acceptOrdinalSellOrder

*Accepts an ordinal sell order for swapping Oridinal to USDT.*


```solidity
function acceptOrdinalSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The ID of the ordinal sell order.|
|`bitcoinAddress`|`BitcoinAddress`|The Bitcoin address of the buyer.|


### completeOrdinalSellOrder

*Completes the ordinal sell order by validating the BTC transaction proof,
ensuring that the BTC transaction input spends the specified UTXO associated with the ordinal sell order,
and transfer USDT to the seller.*


```solidity
function completeOrdinalSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
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
### btcSellOrderSuccessfullyPlaced

```solidity
event btcSellOrderSuccessfullyPlaced(uint256 indexed orderId, uint256 sellAmountBtc, uint256 buyAmount);
```

### btcSellOrderBtcSellOrderAccepted

```solidity
event btcSellOrderBtcSellOrderAccepted(uint256 indexed id, BitcoinAddress bitcoinAddress);
```

### btcSuccessfullySendtoDestination

```solidity
event btcSuccessfullySendtoDestination(uint256 id);
```

### ordinalSellOrderSuccessfullyPlaced

```solidity
event ordinalSellOrderSuccessfullyPlaced(uint256 indexed id, OrdinalId ordinalID, uint256 buyAmount);
```

### ordinalSellOrderBtcSellOrderAccepted

```solidity
event ordinalSellOrderBtcSellOrderAccepted(uint256 indexed id, BitcoinAddress bitcoinAddress);
```

### ordinalSuccessfullySendtoDestination

```solidity
event ordinalSuccessfullySendtoDestination(uint256 id);
```

## Structs
### BtcSellOrder
*Struct representing a BTC to USDT swap order.*


```solidity
struct BtcSellOrder {
    uint256 sellAmountBtc;
    uint256 buyAmount;
    address btcSeller;
    BitcoinAddress btcBuyer;
    bool isOrderAccepted;
}
```

### OrdinalSellOrder
*Struct representing an ordinal sell order for swapping Ordinal to USDT.*


```solidity
struct OrdinalSellOrder {
    OrdinalId ordinalID;
    uint256 buyAmount;
    BitcoinTx.UTXO utxo;
    address ordinalSeller;
    BitcoinAddress ordinalBuyer;
    bool isOrderAccepted;
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

