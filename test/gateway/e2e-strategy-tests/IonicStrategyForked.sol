// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IIonicToken, IPool, IonicStrategy} from "../../../src/gateway/strategy/IonicStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";

// Command to run this test with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract IonicStrategyForked -vv
contract IonicStrategyForked is Test {
    // Instantiate TBTC token using its address from Constants
    IERC20 token = IERC20(Constants.TBTC_ADDRESS);

    function setUp() public {
        // Set up the test environment by creating a fork of the BOB_PROD_PUBLIC_RPC_URL
        // Fixing the block number ensures reproducible test results
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), 6077077);

        // Transfer 100 TBTC tokens to DUMMY_SENDER
        vm.prank(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C);
        token.transfer(Constants.DUMMY_SENDER, 100 ether);
        vm.stopPrank();
    }

    function testIonicStrategy() public {
        // Define Ionic tBTC v2 (iontBTC) token and the CErc20Delegate contract instance
        IIonicToken iontBtcToken = IIonicToken(0x68e0e4d875FDe34fc4698f40ccca0Db5b67e3693);

        // Define Comptroller contract for market entry
        IPool poolContract = IPool(0x9cFEe81970AA10CC593B83fB96eAA9880a6DF715);

        // Instantiate the strategy
        IonicStrategy ionicStrategy = new IonicStrategy(iontBtcToken, poolContract);

        // DUMMY_SENDER approves the strategy contract to spend 1 WBTC on their behalf
        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(ionicStrategy), 1 ether);
        vm.stopPrank();
        console.log("balance 1:", IERC20(address(iontBtcToken)).balanceOf(Constants.DUMMY_RECEIVER));

        // DUMMY_SENDER sends 1 WBTC to the strategy with slippage arguments
        vm.prank(Constants.DUMMY_SENDER);
        ionicStrategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 ether, // Amount: 1 TBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        // ToDo: remove magic number
        assertEq(IERC20(address(iontBtcToken)).balanceOf(Constants.DUMMY_RECEIVER), 4999999998829624675);
    }
}
