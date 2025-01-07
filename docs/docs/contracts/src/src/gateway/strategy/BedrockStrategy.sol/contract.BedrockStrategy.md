# BedrockStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/BedrockStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### vault

```solidity
IBedrockVault public immutable vault;
```


## Functions
### constructor


```solidity
constructor(IBedrockVault _vault);
```

### handleGatewayMessageWithSlippageArgs

Deposits tokens into Bedrock to mint uniBTC.

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
|`recipient`|`address`|The address to receive uniBTC.|
|`args`|`StrategySlippageArgs`|Additional args for slippage protection.|


