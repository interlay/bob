# BtcMarketPlace
[Git Source](https://github.com/bob-collective/bob/blob/master/src/swap/Btc_Marketplace.sol)

**Inherits:**
ERC2771Recipient


## State Variables
### btcBuyOrders

```solidity
mapping(uint256 => BtcBuyOrder) public btcBuyOrders;
```


### acceptedBtcBuyOrders

```solidity
mapping(uint256 => AcceptedBtcBuyOrder) public acceptedBtcBuyOrders;
```


### btcSellOrders

```solidity
mapping(uint256 => BtcSellOrder) public btcSellOrders;
```


### acceptedBtcSellOrders

```solidity
mapping(uint256 => AcceptedBtcSellOrder) public acceptedBtcSellOrders;
```


### nextOrderId

```solidity
uint256 nextOrderId;
```


### REQUEST_EXPIRATION_SECONDS

```solidity
uint256 public constant REQUEST_EXPIRATION_SECONDS = 6 hours;
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


```solidity
constructor(IRelay _relay, address erc2771Forwarder);
```

### setRelay


```solidity
function setRelay(IRelay _relay) internal;
```

### placeBtcSellOrder


```solidity
function placeBtcSellOrder(uint256 amountBtc, address buyingToken, uint256 buyAmount) public;
```

### acceptBtcSellOrder


```solidity
function acceptBtcSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress, uint256 amountBtc)
    public
    returns (uint256);
```

### proofBtcSellOrder


```solidity
function proofBtcSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof) public;
```

### withdrawBtcSellOrder


```solidity
function withdrawBtcSellOrder(uint256 id) public;
```

### cancelAcceptedBtcSellOrder


```solidity
function cancelAcceptedBtcSellOrder(uint256 id) public;
```

### placeBtcBuyOrder


```solidity
function placeBtcBuyOrder(
    uint256 amountBtc,
    BitcoinAddress calldata bitcoinAddress,
    address sellingToken,
    uint256 saleAmount
) public;
```

### acceptBtcBuyOrder


```solidity
function acceptBtcBuyOrder(uint256 id, uint256 amountBtc) public returns (uint256);
```

### proofBtcBuyOrder


```solidity
function proofBtcBuyOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof) public;
```

### withdrawBtcBuyOrder


```solidity
function withdrawBtcBuyOrder(uint256 id) public;
```

### cancelAcceptedBtcBuyOrder


```solidity
function cancelAcceptedBtcBuyOrder(uint256 id) public;
```

### getOpenBtcSellOrders


```solidity
function getOpenBtcSellOrders() external view returns (BtcSellOrder[] memory, uint256[] memory);
```

### getOpenAcceptedBtcSellOrders


```solidity
function getOpenAcceptedBtcSellOrders() external view returns (AcceptedBtcSellOrder[] memory, uint256[] memory);
```

### getOpenBtcBuyOrders


```solidity
function getOpenBtcBuyOrders() external view returns (BtcBuyOrder[] memory, uint256[] memory);
```

### getOpenAcceptedBtcBuyOrders


```solidity
function getOpenAcceptedBtcBuyOrders() external view returns (AcceptedBtcBuyOrder[] memory, uint256[] memory);
```

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
### placeBtcSellOrderEvent

```solidity
event placeBtcSellOrderEvent(uint256 indexed orderId, uint256 amountBtc, address buyingToken, uint256 buyAmount);
```

### acceptBtcSellOrderEvent

```solidity
event acceptBtcSellOrderEvent(
    uint256 indexed id,
    uint256 indexed acceptId,
    BitcoinAddress bitcoinAddress,
    uint256 amountBtc,
    uint256 ercAmount,
    address ercToken
);
```

### proofBtcSellOrderEvent

```solidity
event proofBtcSellOrderEvent(uint256 id);
```

### withdrawBtcSellOrderEvent

```solidity
event withdrawBtcSellOrderEvent(uint256 id);
```

### cancelAcceptedBtcSellOrderEvent

```solidity
event cancelAcceptedBtcSellOrderEvent(uint256 id);
```

### placeBtcBuyOrderEvent

```solidity
event placeBtcBuyOrderEvent(uint256 amountBtc, BitcoinAddress bitcoinAddress, address sellingToken, uint256 saleAmount);
```

### acceptBtcBuyOrderEvent

```solidity
event acceptBtcBuyOrderEvent(
    uint256 indexed orderId, uint256 indexed acceptId, uint256 amountBtc, uint256 ercAmount, address ercToken
);
```

### proofBtcBuyOrderEvent

```solidity
event proofBtcBuyOrderEvent(uint256 id);
```

### withdrawBtcBuyOrderEvent

```solidity
event withdrawBtcBuyOrderEvent(uint256 id);
```

### cancelAcceptedBtcBuyOrderEvent

```solidity
event cancelAcceptedBtcBuyOrderEvent(uint256 id);
```

## Structs
### BtcSellOrder

```solidity
struct BtcSellOrder {
    uint256 amountBtc;
    address askingToken;
    uint256 askingAmount;
    address requester;
}
```

### AcceptedBtcSellOrder

```solidity
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
```

### BtcBuyOrder

```solidity
struct BtcBuyOrder {
    uint256 amountBtc;
    BitcoinAddress bitcoinAddress;
    address offeringToken;
    uint256 offeringAmount;
    address requester;
}
```

### AcceptedBtcBuyOrder

```solidity
struct AcceptedBtcBuyOrder {
    uint256 orderId;
    uint256 amountBtc;
    address ercToken;
    uint256 ercAmount;
    address requester;
    address accepter;
    uint256 acceptTime;
}
```

### BitcoinAddress

```solidity
struct BitcoinAddress {
    bytes scriptPubKey;
}
```

### TransactionProof

```solidity
struct TransactionProof {
    uint256 dummy;
}
```

