# IPool
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/IonicStrategy.sol)

*Interface for the Ionic Finance Pool, allowing entry and exit from markets.*


## Functions
### enterMarkets


```solidity
function enterMarkets(address[] memory cTokens) external returns (uint256[] memory);
```

### exitMarket


```solidity
function exitMarket(address cTokenAddress) external returns (uint256);
```

