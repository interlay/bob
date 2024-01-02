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
function _getPaymasterData(bytes memory paymasterData) private returns (IERC20 token, uint256 maxTokens);
```

### getSelector


```solidity
function getSelector(bytes calldata call) public view returns (uint32);
```

### _preRelayedCall


```solidity
function _preRelayedCall(
    GsnTypes.RelayRequest calldata relayRequest,
    bytes calldata signature,
    bytes calldata approvalData,
    uint256 maxPossibleGas
) internal virtual override returns (bytes memory context, bool revertOnRecipientRevert);
```

### _postRelayedCall


```solidity
function _postRelayedCall(
    bytes calldata context,
    bool,
    uint256 gasUseWithoutPost,
    GsnTypes.RelayData calldata relayData
) internal virtual override;
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

