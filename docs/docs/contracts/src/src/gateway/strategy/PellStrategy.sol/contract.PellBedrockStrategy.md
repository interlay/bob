# PellBedrockStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/PellStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### bedrockStrategy

```solidity
BedrockStrategy public immutable bedrockStrategy;
```


### pellStrategy

```solidity
PellStrategy public immutable pellStrategy;
```


## Functions
### constructor


```solidity
constructor(BedrockStrategy _bedrockStrategy, PellStrategy _pellStrategy);
```

### handleGatewayMessageWithSlippageArgs


```solidity
function handleGatewayMessageWithSlippageArgs(
    IERC20 tokenSent,
    uint256 amount,
    address recipient,
    StrategySlippageArgs memory args
) public override;
```

