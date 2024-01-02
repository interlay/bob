# OracleTokenPaymaster
[Git Source](https://github.com/bob-collective/bob/blob/master/src/paymasters/OracleTokenPaymaster.sol)

**Inherits:**
BasePaymaster

A very basic paymaster that makes the payer pay in ERC20 tokens.
- The token prices need to be provided by an IOracle.
- No swaps are done - the paymaster simply receives ERC20 tokens. This means
that over time, the paymaster's eth balance will decrease. It is up to the
owner of the contract to replenish the eth balance.
- The owner of the contract can withdraw their received erc20 balances.
- Users specify an erc20 address and a maximum amount they are willing to pay
for the tx. This reduces the trust put in the oracle.


## State Variables
### nativeTokenOracle

```solidity
IOracle nativeTokenOracle;
```


### tokenOracles

```solidity
mapping(IERC20 => TokenDetails) public tokenOracles;
```


### gasUsedByPost

```solidity
uint256 public gasUsedByPost;
```


## Functions
### constructor


```solidity
constructor(IOracle _nativeTokenOracle);
```

### versionPaymaster


```solidity
function versionPaymaster() external view virtual override returns (string memory);
```

### addOracle


```solidity
function addOracle(IERC20 _token, uint256 _decimals, IOracle _oracle) external onlyOwner;
```

### fetchPrice


```solidity
function fetchPrice(IOracle _oracle) internal view returns (uint192 price);
```

### _ethToTokens


```solidity
function _ethToTokens(IERC20 token, uint256 ethAmount) internal view returns (uint256);
```

### setPostGasUsage


```solidity
function setPostGasUsage(uint256 _gasUsedByPost) external onlyOwner;
```

### withdrawAll


```solidity
function withdrawAll(IERC20 token) external onlyOwner;
```

### getPayer


```solidity
function getPayer(GsnTypes.RelayRequest calldata relayRequest) public view virtual returns (address);
```

### _getPaymasterData


```solidity
function _getPaymasterData(bytes memory paymasterData) private returns (IERC20 token, uint256 maxTokens);
```

### _calculatePreCharge


```solidity
function _calculatePreCharge(IERC20 token, GsnTypes.RelayRequest calldata relayRequest, uint256 maxPossibleGas)
    internal
    returns (address payer, uint256 ethPrecharge, uint256 tokenPreCharge);
```

### _verifyPaymasterData


```solidity
function _verifyPaymasterData(GsnTypes.RelayRequest calldata relayRequest) internal view virtual override;
```

### __preRelayedCall


```solidity
function __preRelayedCall(
    GsnTypes.RelayRequest calldata relayRequest,
    bytes calldata signature,
    bytes calldata approvalData,
    uint256 maxPossibleGas
) public returns (bytes memory context, bool revertOnRecipientRevert);
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

### _postRelayedCallInternal


```solidity
function _postRelayedCallInternal(
    address payer,
    uint256 tokenPrecharge,
    uint256 valueRequested,
    uint256 gasUseWithoutPost,
    GsnTypes.RelayData calldata relayData,
    IERC20 token
) internal;
```

## Events
### PreRelayPayment

```solidity
event PreRelayPayment(uint256 ethAmount, IERC20 token, uint256 tokenAmount, address indexed payer);
```

### PostRelay

```solidity
event PostRelay(uint256 actualEthAmount, IERC20 token, uint256 actualTokenAmount, address payer);
```

## Structs
### TokenDetails

```solidity
struct TokenDetails {
    uint256 div;
    IOracle oracle;
}
```

