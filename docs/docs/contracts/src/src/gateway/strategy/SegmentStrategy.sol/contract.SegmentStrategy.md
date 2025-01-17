# SegmentStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/SegmentStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### seBep20

```solidity
ISeBep20 public immutable seBep20;
```


## Functions
### constructor


```solidity
constructor(ISeBep20 _seBep20);
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


