# SegmentSolvLSTStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/SegmentStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### solvLSTStrategy

```solidity
SolvLSTStrategy public immutable solvLSTStrategy;
```


### segmentStrategy

```solidity
SegmentStrategy public immutable segmentStrategy;
```


## Functions
### constructor


```solidity
constructor(SolvLSTStrategy _solvLSTStrategy, SegmentStrategy _segmentStrategy);
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

