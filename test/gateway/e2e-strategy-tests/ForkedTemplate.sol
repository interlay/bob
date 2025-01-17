// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IAvalonIPool, AvalonLendingStrategy} from "../../../src/gateway/strategy/AvalonStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";

abstract contract ForkedStrategyTemplateWbtc is Test {
    IERC20 public token;

    constructor() {
        token = IERC20(Constants.WBTC_ADDRESS);
    }

    function simulateForkAndTransfer(uint256 forkAtBlock, address sender, address receiver, uint256 amount) public {
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), forkAtBlock);
        vm.prank(sender);
        token.transfer(receiver, amount);
    }
}

abstract contract ForkedStrategyTemplateTbtc is Test {
    IERC20 public token;

    constructor() {
        token = IERC20(Constants.TBTC_ADDRESS);
    }

    function simulateForkAndTransfer(uint256 forkAtBlock, address sender, address receiver, uint256 amount) public {
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), forkAtBlock);
        vm.prank(sender);
        token.transfer(receiver, amount);
    }
}
