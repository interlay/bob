# IAvalonIPool
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/AvalonStrategy.sol)


## Functions
### supply

Mints pool tokens to the recipient.


```solidity
function supply(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`asset`|`address`|The address of the ERC20 token to supply to the pool.|
|`amount`|`uint256`|The amount of the asset to supply.|
|`onBehalfOf`|`address`|The address that will receive the supplied amount.|
|`referralCode`|`uint16`|Optional referral code to track the origin of the supply.|


### withdraw

Withdraws asset from the pool.


```solidity
function withdraw(address asset, uint256 amount, address to) external returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`asset`|`address`|The address of the ERC20 token to withdraw from the pool.|
|`amount`|`uint256`|The amount of the asset to withdraw.|
|`to`|`address`|The address that will receive the withdrawn amount.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The actual amount withdrawn.|


