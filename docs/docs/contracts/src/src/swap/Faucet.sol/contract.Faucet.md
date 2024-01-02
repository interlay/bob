# Faucet
[Git Source](https://github.com/bob-collective/bob/blob/master/src/swap/Faucet.sol)

**Inherits:**
Ownable, ERC2771Recipient


## State Variables
### nextTokenId

```solidity
uint256 nextTokenId;
```


### supportedErc20Addresses

```solidity
mapping(uint256 => address) supportedErc20Addresses;
```


## Functions
### addErc20


```solidity
function addErc20(address newErc20) public onlyOwner;
```

### mint


```solidity
function mint() public;
```

### _msgSender


```solidity
function _msgSender() internal view override(Context, ERC2771Recipient) returns (address sender);
```

### _msgData


```solidity
function _msgData() internal view override(Context, ERC2771Recipient) returns (bytes calldata);
```

