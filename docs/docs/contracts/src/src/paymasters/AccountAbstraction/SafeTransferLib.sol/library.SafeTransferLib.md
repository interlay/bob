# SafeTransferLib
[Git Source](https://github.com/bob-collective/bob/blob/master/src/paymasters/AccountAbstraction/SafeTransferLib.sol)

**Authors:**
Solady (https://github.com/vectorized/solady/blob/main/src/utils/SafeTransferLib.sol), Modified from Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol)

Safe ETH and ERC20 transfer library that gracefully handles missing return values.

*Caution! This library won't check that a token has code, responsibility is delegated to the caller.*


## State Variables
### _GAS_STIPEND_NO_STORAGE_WRITES
*Suggested gas stipend for contract receiving ETH
that disallows any storage writes.*


```solidity
uint256 internal constant _GAS_STIPEND_NO_STORAGE_WRITES = 2300;
```


### _GAS_STIPEND_NO_GRIEF
*Suggested gas stipend for contract receiving ETH to perform a few
storage reads and writes, but low enough to prevent griefing.
Multiply by a small constant (e.g. 2), if needed.*


```solidity
uint256 internal constant _GAS_STIPEND_NO_GRIEF = 100000;
```


## Functions
### safeTransferFrom

*Sends `amount` of ERC20 `token` from `from` to `to`.
Reverts upon failure.
The `from` account must have at least `amount` approved for
the current contract to manage.*


```solidity
function safeTransferFrom(address token, address from, address to, uint256 amount) internal;
```

### safeTransferAllFrom

*Sends all of ERC20 `token` from `from` to `to`.
Reverts upon failure.
The `from` account must have at least `amount` approved for
the current contract to manage.*


```solidity
function safeTransferAllFrom(address token, address from, address to) internal returns (uint256 amount);
```

### safeTransfer

*Sends `amount` of ERC20 `token` from the current contract to `to`.
Reverts upon failure.*


```solidity
function safeTransfer(address token, address to, uint256 amount) internal;
```

### safeTransferAll

*Sends all of ERC20 `token` from the current contract to `to`.
Reverts upon failure.*


```solidity
function safeTransferAll(address token, address to) internal returns (uint256 amount);
```

### safeApprove

*Sets `amount` of ERC20 `token` for `to` to manage on behalf of the current contract.
Reverts upon failure.*


```solidity
function safeApprove(address token, address to, uint256 amount) internal;
```

### balanceOf

*Returns the amount of ERC20 `token` owned by `account`.
Returns zero if the `token` does not exist.*


```solidity
function balanceOf(address token, address account) internal view returns (uint256 amount);
```

## Errors
### ETHTransferFailed
*The ETH transfer has failed.*


```solidity
error ETHTransferFailed();
```

### TransferFromFailed
*The ERC20 `transferFrom` has failed.*


```solidity
error TransferFromFailed();
```

### TransferFailed
*The ERC20 `transfer` has failed.*


```solidity
error TransferFailed();
```

### ApproveFailed
*The ERC20 `approve` has failed.*


```solidity
error ApproveFailed();
```

