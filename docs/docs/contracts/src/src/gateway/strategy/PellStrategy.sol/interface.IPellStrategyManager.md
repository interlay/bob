# IPellStrategyManager
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/PellStrategy.sol)

*Pell ABI for their strategy manager.*


## Functions
### depositIntoStrategyWithStaker


```solidity
function depositIntoStrategyWithStaker(address staker, IPellStrategy strategy, IERC20 token, uint256 amount)
    external
    returns (uint256 shares);
```

### stakerStrategyShares


```solidity
function stakerStrategyShares(address staker, IPellStrategy strategy) external view returns (uint256);
```

