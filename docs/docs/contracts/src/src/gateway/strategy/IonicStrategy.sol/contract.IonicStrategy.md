# IonicStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/IonicStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context

*Implements IStrategyWithSlippageArgs and allows the contract to handle tokens with slippage arguments.*


## State Variables
### ioErc20

```solidity
IIonicToken public immutable ioErc20;
```


### pool

```solidity
IPool public immutable pool;
```


## Functions
### constructor

*Constructor to initialize the Ionic token and pool interfaces.*


```solidity
constructor(IIonicToken _ioErc20, IPool _pool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ioErc20`|`IIonicToken`|Address of the Ionic token.|
|`_pool`|`IPool`|Address of the Ionic pool.|


### handleGatewayMessageWithSlippageArgs

*Handles the transfer and minting of tokens with slippage control.*


```solidity
function handleGatewayMessageWithSlippageArgs(
    IERC20 tokenSent,
    uint256 amountIn,
    address recipient,
    StrategySlippageArgs memory args
) public override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`tokenSent`|`IERC20`|The token to be transferred.|
|`amountIn`|`uint256`|The amount of tokens to be transferred.|
|`recipient`|`address`|The recipient address.|
|`args`|`StrategySlippageArgs`|Slippage arguments, including minimum acceptable output amount.|


