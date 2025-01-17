# AvalonLstStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/AvalonStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### solvLSTStrategy

```solidity
SolvLSTStrategy public immutable solvLSTStrategy;
```


### avalonLendingStrategy

```solidity
AvalonLendingStrategy public immutable avalonLendingStrategy;
```


## Functions
### constructor


```solidity
constructor(SolvLSTStrategy _solvLSTStrategy, AvalonLendingStrategy _avalonLendingStrategy);
```

### handleGatewayMessageWithSlippageArgs

Mints lending tokens to the recipient.

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
|`recipient`|`address`|The address to receive the lending tokens.|
|`args`|`StrategySlippageArgs`|Additional args for slippage protection.|


