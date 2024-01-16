# HelloBitcoin
[Git Source](https://github.com/bob-collective/bob/blob/master/src/hello-bitcoin/HelloBitcoin.sol)


## State Variables
### btcSellOrders
*Mapping to store BTC to USDT (or other ERC20) swap orders based on their unique identifiers.
Each order is associated with a unique ID, and the order details are stored in the BtcSellOrder struct.*


```solidity
mapping(uint256 => BtcSellOrder) public btcSellOrders;
```


### ordinalSellOrders
*Mapping to store ordinal sell orders for swapping BTC to USDT (or other ERC20) based on their unique identifiers.
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

*Constructor to initialize the contract with the relay and ERC20 token address.*


```solidity
constructor(IRelay _relay, address _usdtContractAddress);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_relay`|`IRelay`|The relay contract implementing the IRelay interface.|
|`_usdtContractAddress`|`address`|The address of the USDT contract. Additional functionalities of the relay can be found in the documentation available at: https://docs.gobob.xyz/docs/contracts/src/src/relay/LightRelay.sol/contract.LightRelay|


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

Places a BTC sell order in the contract.

*Emits a `btcSellOrderSuccessfullyPlaced` event upon successful placement.*

*Requirements:
- `sellAmountBtc` must be greater than 0.
- `buyAmount` must be greater than 0.*


```solidity
function placeBtcSellOrder(uint256 sellAmountBtc, uint256 buyAmount) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`sellAmountBtc`|`uint256`|The amount of BTC to sell.|
|`buyAmount`|`uint256`|The corresponding amount to be received in exchange for the BTC.|


### acceptBtcSellOrder

Accepts a BTC sell order, providing the Bitcoin address for the buyer.

*Transfers the corresponding currency from the buyer to the contract and updates the order details.*

*Requirements:
- The specified order must not have been accepted previously.
- The buyer must transfer the required currency amount to the contract.*

*Emits a `btcSellOrderBtcSellOrderAccepted` event upon successful acceptance.*


```solidity
function acceptBtcSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The unique identifier of the BTC sell order.|
|`bitcoinAddress`|`BitcoinAddress`|The Bitcoin address of the buyer to receive the BTC.|


### completeBtcSellOrder

Completes a BTC sell order by validating and processing the provided Bitcoin transaction proof.

*This function is intended to be called by the original seller.*

*Requirements:
- The specified order must have been previously accepted.
- The caller must be the original seller of the BTC.
- The Bitcoin transaction proof must be valid.
- The BTC transaction output must match the expected amount and recipient.*

*Effects:
- Sets the relay difficulty based on the Bitcoin headers in the proof.
- Transfers the locked USDT amount to the original seller.
- Removes the order from the mapping after successful processing.*

*Emits a `btcSuccessfullySendtoDestination` event upon successful completion.*


```solidity
function completeBtcSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The unique identifier of the BTC sell order.|
|`transaction`|`BitcoinTx.Info`|Information about the Bitcoin transaction.|
|`proof`|`BitcoinTx.Proof`|Proof associated with the Bitcoin transaction.|


### placeOrdinalSellOrder

Places an ordinal sell order in the contract.

*Emits an `ordinalSellOrderSuccessfullyPlaced` event upon successful placement.*

*Requirements:
- `buyAmount` must be greater than 0.*

*Effects:
- Creates a new ordinal sell order with the provided details.*


```solidity
function placeOrdinalSellOrder(OrdinalId calldata ordinalID, BitcoinTx.UTXO calldata utxo, uint256 buyAmount) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`ordinalID`|`OrdinalId`|The unique identifier for the ordinal.|
|`utxo`|`BitcoinTx.UTXO`|Information about the Bitcoin UTXO associated with the ordinal.|
|`buyAmount`|`uint256`|The amount to be received in exchange for the ordinal.|


### acceptOrdinalSellOrder

Accepts an ordinal sell order, providing the Bitcoin address for the buyer.

*Transfers the corresponding currency from the buyer to the contract and updates the order details.*

*Requirements:
- The specified order must not have been accepted previously.
- The buyer must transfer the required currency amount to this contract.*

*Effects:
- "Locks" the selling token by transferring it to the contract.
- Updates the ordinal sell order with the buyer's Bitcoin address and marks the order as accepted.*

*Emits an `ordinalSellOrderBtcSellOrderAccepted` event upon successful acceptance.*


```solidity
function acceptOrdinalSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The unique identifier of the ordinal sell order.|
|`bitcoinAddress`|`BitcoinAddress`|The Bitcoin address of the buyer to receive the ordinal.|


### completeOrdinalSellOrder

Completes an ordinal sell order by validating and processing the provided Bitcoin transaction proof.

*This function is intended to be called by the original seller.*

*Requirements:
- The specified order must have been previously accepted.
- The caller must be the original seller of the ordinal.
- The Bitcoin transaction proof must be valid.
- The BTC transaction input must spend the specified UTXO associated with the ordinal sell order.
- The BTC transaction output must be to the buyer's address.*

*Effects:
- Sets the relay difficulty based on the Bitcoin headers in the proof.
- Validates the BTC transaction proof using the relay.
- Ensures that the BTC transaction input spends the specified UTXO.
- Checks the BTC transaction output to the buyer's address.
- Transfers the locked USDT amount to the original seller.
- Removes the ordinal sell order from storage after successful processing.*

*Emits an `ordinalSuccessfullySendtoDestination` event upon successful completion.*


```solidity
function completeOrdinalSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
    public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`id`|`uint256`|The unique identifier of the ordinal sell order.|
|`transaction`|`BitcoinTx.Info`|Information about the Bitcoin transaction.|
|`proof`|`BitcoinTx.Proof`|Proof associated with the Bitcoin transaction.|


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

