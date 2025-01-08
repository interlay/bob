// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ICErc20, ShoebillStrategy} from "../../../src/gateway/strategy/ShoebillStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";

// Command to run this test with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract ShoebillTBTCStrategyForked -vv

contract ShoebillTBTCStrategyForked is Test {
    // Instantiate TBTC token using its address from Constants
    IERC20 token = IERC20(Constants.TBTC_ADDRESS);

    function setUp() public {
        // creating fork and fixing the block number, so the test can be repeated
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), 5607192);
        // Transfer 100 TBTC tokens to DUMMY_SENDER
        vm.prank(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C);
        token.transfer(Constants.DUMMY_SENDER, 100 ether);
        vm.stopPrank();
    }

    function testShoebillStrategy() public {
        ICErc20 cErc20 = ICErc20(0x2925dF9Eb2092B53B06A06353A7249aF3a8B139e);
        ShoebillStrategy shoebillStrategy = new ShoebillStrategy(cErc20);

        // DUMMY_SENDER approves the strategy contract to spend 1 WBTC on their behalf
        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(shoebillStrategy), 1 ether);
        vm.stopPrank();

        // DUMMY_SENDER sends 1 WBTC to the strategy with slippage arguments
        vm.prank(Constants.DUMMY_SENDER);
        shoebillStrategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 ether, // Amount: 1 TBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        // ToDo: remove the magic number
        assertEq(cErc20.balanceOfUnderlying(address(Constants.DUMMY_RECEIVER)), 999999999973746162);
    }
}
