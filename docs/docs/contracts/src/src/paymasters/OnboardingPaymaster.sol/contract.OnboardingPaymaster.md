# OnboardingPaymaster
[Git Source](https://github.com/bob-collective/bob/blob/master/src/paymasters/OnboardingPaymaster.sol)

**Inherits:**
BasePaymaster


## State Variables
### whitelistedContract

```solidity
address public whitelistedContract;
```


### whitelistedSelector

```solidity
uint32 public whitelistedSelector;
```


### gasUsedByPost

```solidity
uint256 public gasUsedByPost;
```


## Functions
### constructor


```solidity
constructor(address _whitelistedContract, uint32 _whitelistedSelector);
```

### versionPaymaster


```solidity
function versionPaymaster() external view virtual override returns (string memory);
```

### setPostGasUsage


```solidity
function setPostGasUsage(uint256 _gasUsedByPost) external onlyOwner;
```

### _getPaymasterData


```solidity
function _getPaymasterData(bytes memory paymasterData) private pure returns (IERC20 token, uint256 maxTokens);
```

### getSelector


```solidity
function getSelector(bytes calldata call) public pure returns (uint32);
```

### _preRelayedCall


```solidity
function _preRelayedCall(GsnTypes.RelayRequest calldata relayRequest, bytes calldata, bytes calldata, uint256)
    internal
    virtual
    override
    returns (bytes memory context, bool revertOnRecipientRevert);
```

### _postRelayedCall


```solidity
function _postRelayedCall(bytes calldata context, bool, uint256, GsnTypes.RelayData calldata)
    internal
    virtual
    override;
```

## Events
### PreRelay

```solidity
event PreRelay(address indexed sender);
```

### PostRelay

```solidity
event PostRelay(address indexed sender);
```

