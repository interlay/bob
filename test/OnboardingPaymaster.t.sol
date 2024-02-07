// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {Utilities} from "./swap/Utilities.sol";
import {OnboardingPaymaster} from "../src/paymasters/OnboardingPaymaster.sol";
// import {}
import "lib/gsn/packages/contracts/src/BasePaymaster.sol";

contract OnboardingPaymasterTest is OnboardingPaymaster, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    constructor() OnboardingPaymaster(address(0x00), 0) {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");
    }

    function testDecodeSelector() public view {
        bytes memory rawBytes = hex"1234567890";
        this.getSelector(rawBytes);
    }
}
