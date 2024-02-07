# Bridge
[Git Source](https://github.com/bob-collective/bob/blob/master/src/swap/Bridge.sol)

**Inherits:**
ERC2771Recipient


## State Variables
### number

```solidity
uint256 public number;
```


### collateralThreshold

```solidity
uint256 public collateralThreshold;
```


### nextOrderId

```solidity
uint256 nextOrderId;
```


### orders

```solidity
mapping(uint256 => Order) public orders;
```


### suppliedCollateral

```solidity
mapping(address => uint256) public suppliedCollateral;
```


### totalCollateral

```solidity
uint256 totalCollateral;
```


### wrapped

```solidity
BobWrappedBtc wrapped = new BobWrappedBtc();
```


## Functions
### constructor


```solidity
constructor(uint256 threshold);
```

### mint

lock COL in exchange for zBTC and cCOL


```solidity
function mint() public payable;
```

### requestSwap

request zBTC to be redeemed for given amount of BTC.


```solidity
function requestSwap(uint256 amountZbtc, uint256 amountBtc, BitcoinAddress calldata bitcoinAddress) public;
```

### acceptSwap


```solidity
function acceptSwap(uint256 id) public;
```

### cancelSwap


```solidity
function cancelSwap(uint256 id) public;
```

### executeSwap


```solidity
function executeSwap(uint256 id, TransactionProof calldata) public;
```

### liquidate


```solidity
function liquidate(uint256 amountZbtc) public;
```

### withdraw


```solidity
function withdraw() public;
```

### colToBtc


```solidity
function colToBtc(uint256 collateral) internal pure returns (uint256);
```

### btcToCol


```solidity
function btcToCol(uint256 collateral) internal pure returns (uint256);
```

## Structs
### Order

```solidity
struct Order {
    bool open;
    uint256 amountZbtc;
    uint256 amountBtc;
    address requesterAddress;
    address accepterAddress;
    BitcoinAddress bitcoinAddress;
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

