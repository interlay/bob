# IStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/IStrategy.sol)


## Functions
### handleGatewayMessage


```solidity
function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message) external;
```

## Events
### TokenOutput

```solidity
event TokenOutput(address tokenReceived, uint256 amountOut);
```

