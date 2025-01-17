// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/**
 * @title Commonly used constants.
 */
library Constants {
    // Used for providing a uniform interface for functions that deal with both ERC20 and native tokens.
    address constant WBTC_ADDRESS = 0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3;
    address constant TBTC_ADDRESS = 0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2;
    address constant DUMMY_SENDER = 0x999999cf1046e68e36E1aA2E0E07105eDDD1f08E;
    address constant DUMMY_RECEIVER = 0xa2adc38f06704Cd2e633e8656DbF8B3224E840b8;
}
