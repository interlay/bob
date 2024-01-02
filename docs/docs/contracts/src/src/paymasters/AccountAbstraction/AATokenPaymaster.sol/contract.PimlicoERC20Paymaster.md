# PimlicoERC20Paymaster
[Git Source](https://github.com/bob-collective/bob/blob/master/src/paymasters/AccountAbstraction/AATokenPaymaster.sol)

**Inherits:**
BasePaymaster

An ERC-4337 Paymaster contract by Pimlico which is able to sponsor gas fees in exchange for ERC20 tokens.
The contract refunds excess tokens if the actual gas cost is lower than the initially provided amount.
It also allows updating price configuration and withdrawing tokens by the contract owner.
The contract uses an Oracle to fetch the latest token prices.

*Inherits from BasePaymaster.*


## State Variables
### priceDenominator

```solidity
uint256 public constant priceDenominator = 1e6;
```


### REFUND_POSTOP_COST

```solidity
uint256 public constant REFUND_POSTOP_COST = 40000;
```


### token

```solidity
IERC20 public immutable token;
```


### tokenDecimals

```solidity
uint256 public immutable tokenDecimals;
```


### tokenOracle

```solidity
IOracle public immutable tokenOracle;
```


### nativeAssetOracle

```solidity
IOracle public immutable nativeAssetOracle;
```


### previousPrice

```solidity
uint192 public previousPrice;
```


### priceMarkup

```solidity
uint32 public priceMarkup;
```


### priceUpdateThreshold

```solidity
uint32 public priceUpdateThreshold;
```


## Functions
### constructor

Initializes the PimlicoERC20Paymaster contract with the given parameters.


```solidity
constructor(
    IERC20 _token,
    IEntryPoint _entryPoint,
    IOracle _tokenOracle,
    IOracle _nativeAssetOracle,
    address _owner,
    uint8 _tokenDecimals
) BasePaymaster(_entryPoint);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_token`|`IERC20`|The ERC20 token used for transaction fee payments.|
|`_entryPoint`|`IEntryPoint`|The EntryPoint contract used in the Account Abstraction infrastructure.|
|`_tokenOracle`|`IOracle`|The Oracle contract used to fetch the latest token prices.|
|`_nativeAssetOracle`|`IOracle`|The Oracle contract used to fetch the latest native asset (ETH, Matic, Avax, etc.) prices.|
|`_owner`|`address`|The address that will be set as the owner of the contract.|
|`_tokenDecimals`|`uint8`||


### updateConfig

Updates the price markup and price update threshold configurations.


```solidity
function updateConfig(uint32 _priceMarkup, uint32 _updateThreshold) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_priceMarkup`|`uint32`|The new price markup percentage (1e6 = 100%).|
|`_updateThreshold`|`uint32`|The new price update threshold percentage (1e6 = 100%).|


### withdrawToken

Allows the contract owner to withdraw a specified amount of tokens from the contract.


```solidity
function withdrawToken(address to, uint256 amount) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|The address to transfer the tokens to.|
|`amount`|`uint256`|The amount of tokens to transfer.|


### updatePrice

Updates the token price by fetching the latest price from the Oracle.


```solidity
function updatePrice() external;
```

### _validatePaymasterUserOp

Validates a paymaster user operation and calculates the required token amount for the transaction.


```solidity
function _validatePaymasterUserOp(UserOperation calldata userOp, bytes32, uint256 requiredPreFund)
    internal
    override
    returns (bytes memory context, uint256 validationResult);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`userOp`|`UserOperation`|The user operation data.|
|`<none>`|`bytes32`||
|`requiredPreFund`|`uint256`|The amount of tokens required for pre-funding.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`context`|`bytes`|The context containing the token amount and user sender address (if applicable).|
|`validationResult`|`uint256`|A uint256 value indicating the result of the validation (always 0 in this implementation).|


### _postOp

Performs post-operation tasks, such as updating the token price and refunding excess tokens.

*This function is called after a user operation has been executed or reverted.*


```solidity
function _postOp(PostOpMode mode, bytes calldata context, uint256 actualGasCost) internal override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`mode`|`PostOpMode`|The post-operation mode (either successful or reverted).|
|`context`|`bytes`|The context containing the token amount and user sender address.|
|`actualGasCost`|`uint256`|The actual gas cost of the transaction.|


### fetchPrice

Fetches the latest price from the given Oracle.

*This function is used to get the latest price from the tokenOracle or nativeAssetOracle.*


```solidity
function fetchPrice(IOracle _oracle) internal view returns (uint192 price);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_oracle`|`IOracle`|The Oracle contract to fetch the price from.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`price`|`uint192`|The latest price fetched from the Oracle.|


## Events
### ConfigUpdated

```solidity
event ConfigUpdated(uint32 priceMarkup, uint32 updateThreshold);
```

### UserOperationSponsored

```solidity
event UserOperationSponsored(address indexed user, uint256 actualTokenNeeded, uint256 actualGasCost);
```

