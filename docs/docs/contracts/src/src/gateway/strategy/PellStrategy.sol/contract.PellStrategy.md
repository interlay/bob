# PellStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/PellStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### pellStrategyManager

```solidity
IPellStrategyManager public immutable pellStrategyManager;
```


### pellStrategy

```solidity
IPellStrategy public immutable pellStrategy;
```


## Functions
### constructor


```solidity
constructor(IPellStrategyManager _pellStrategyManager, IPellStrategy _pellStrategy);
```

### handleGatewayMessageWithSlippageArgs

Deposits tokens into Pell Network.

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
|`recipient`|`address`|The address to receive the shares.|
|`args`|`StrategySlippageArgs`|Additional args for slippage protection.|


### stakerStrategyShares


```solidity
function stakerStrategyShares(address recipient) public view returns (uint256);
```

