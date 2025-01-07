# SolvBTCStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/SolvStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### solvBTCRouter

```solidity
ISolvBTCRouter public immutable solvBTCRouter;
```


### poolId

```solidity
bytes32 public immutable poolId;
```


### solvBTC

```solidity
IERC20 public immutable solvBTC;
```


## Functions
### constructor


```solidity
constructor(ISolvBTCRouter _solvBTCRouter, bytes32 _poolId, IERC20 _solvBTC);
```

### handleGatewayMessageWithSlippageArgs

Deposits tokens into Solv.

*Requires that the strategy is approved to spend the incoming tokens.*


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
|`tokenSent`|`IERC20`|The ERC20 token to deposit.|
|`amountIn`|`uint256`|The amount to be deposited.|
|`recipient`|`address`|The address to receive SolvBTC.|
|`args`|`StrategySlippageArgs`|Additional args for slippage protection.|


