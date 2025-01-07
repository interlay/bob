# SolvLSTStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/SolvStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### solvBTCRouter

```solidity
ISolvBTCRouter public immutable solvBTCRouter;
```


### solvLSTRouter

```solidity
ISolvBTCRouter public immutable solvLSTRouter;
```


### solvBTCPoolId

```solidity
bytes32 public immutable solvBTCPoolId;
```


### solvLSTPoolId

```solidity
bytes32 public immutable solvLSTPoolId;
```


### solvBTC

```solidity
IERC20 public immutable solvBTC;
```


### solvLST

```solidity
IERC20 public immutable solvLST;
```


## Functions
### constructor


```solidity
constructor(
    ISolvBTCRouter _solvBTCRouter,
    ISolvBTCRouter _solvLSTRouter,
    bytes32 _solvBTCPoolId,
    bytes32 _solvLSTPoolId,
    IERC20 _solvBTC,
    IERC20 _solvLST
);
```

### handleGatewayMessageWithSlippageArgs

Deposits tokens into Solv.

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
|`recipient`|`address`|The address to receive e.g. SolvBTC.BBN.|
|`args`|`StrategySlippageArgs`|Additional args for slippage protection.|


