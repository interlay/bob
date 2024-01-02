# MarketPlace
[Git Source](https://github.com/bob-collective/bob/blob/master/src/swap/Marketplace.sol)

**Inherits:**
ERC2771Recipient


## State Variables
### ercErcOrders

```solidity
mapping(uint256 => Order) public ercErcOrders;
```


### nextOrderId

```solidity
uint256 public nextOrderId;
```


## Functions
### constructor


```solidity
constructor(address erc2771Forwarder);
```

### placeErcErcOrder


```solidity
function placeErcErcOrder(address sellingToken, uint256 saleAmount, address buyingToken, uint256 buyAmount) public;
```

### acceptErcErcOrder


```solidity
function acceptErcErcOrder(uint256 id, uint256 saleAmount) public;
```

### withdrawErcErcOrder


```solidity
function withdrawErcErcOrder(uint256 id) public;
```

### getOpenOrders


```solidity
function getOpenOrders() external view returns (Order[] memory, uint256[] memory);
```

## Events
### placeOrder

```solidity
event placeOrder(
    uint256 indexed orderId,
    address indexed requesterAddress,
    uint256 offeringAmount,
    address offeringToken,
    uint256 askingAmount,
    address askingToken
);
```

### acceptOrder

```solidity
event acceptOrder(uint256 indexed orderId, address indexed who, uint256 buyAmount, uint256 saleAmount);
```

### withdrawOrder

```solidity
event withdrawOrder(uint256 indexed orderId);
```

## Structs
### Order

```solidity
struct Order {
    uint256 offeringAmount;
    address offeringToken;
    uint256 askingAmount;
    address askingToken;
    address requesterAddress;
}
```

