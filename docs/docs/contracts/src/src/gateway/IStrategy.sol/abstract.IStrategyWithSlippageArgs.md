# IStrategyWithSlippageArgs
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/IStrategy.sol)

**Inherits:**
[IStrategy](../../gateway/IStrategy.sol/interface.IStrategy.md)


## Functions
### handleGatewayMessageWithSlippageArgs


```solidity
function handleGatewayMessageWithSlippageArgs(
    IERC20 tokenSent,
    uint256 amountIn,
    address recipient,
    StrategySlippageArgs memory args
) public virtual;
```

### handleGatewayMessage


```solidity
function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message) external;
```

