# ISolvBTCRouter
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/SolvStrategy.sol)

*Solv ABI for their router.*


## Functions
### createSubscription

Subscribe with payment currency (i.e. WBTC) and receive SolvBTC.


```solidity
function createSubscription(bytes32 poolId, uint256 currencyAmount) external returns (uint256 shareValue);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`poolId`|`bytes32`|SolvBTC fund ID.|
|`currencyAmount`|`uint256`|Amount of currency to be paid.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`shareValue`|`uint256`|Amount of SolvBTC to be received after subscription.|


