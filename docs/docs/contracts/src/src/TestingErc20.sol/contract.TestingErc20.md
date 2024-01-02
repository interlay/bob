# TestingErc20
[Git Source](https://github.com/bob-collective/bob/blob/master/src/TestingErc20.sol)

**Inherits:**
ERC20, ERC20Burnable, Ownable, ERC2771Recipient


## State Variables
### _numDecimals

```solidity
uint8 _numDecimals;
```


## Functions
### constructor


```solidity
constructor(string memory _name, string memory _symbol, uint8 _decimals) ERC20(_name, _symbol);
```

### setTrustedForwarder


```solidity
function setTrustedForwarder(address _forwarder) public onlyOwner;
```

### mint


```solidity
function mint(uint256 amount) external;
```

### _msgSender


```solidity
function _msgSender() internal view override(Context, ERC2771Recipient) returns (address sender);
```

### _msgData


```solidity
function _msgData() internal view override(Context, ERC2771Recipient) returns (bytes calldata);
```

### decimals


```solidity
function decimals() public view virtual override returns (uint8);
```

