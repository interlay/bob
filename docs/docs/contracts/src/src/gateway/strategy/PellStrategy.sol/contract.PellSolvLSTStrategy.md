# PellSolvLSTStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/PellStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### solvLSTStrategy

```solidity
SolvLSTStrategy public immutable solvLSTStrategy;
```


### pellStrategy

```solidity
PellStrategy public immutable pellStrategy;
```


## Functions
### constructor


```solidity
constructor(SolvLSTStrategy _solvLSTStrategy, PellStrategy _pellStrategy);
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

