// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ICErc20, ShoebillStrategy} from "../../../src/gateway/strategy/ShoebillStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateTbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract ShoebillTBTCStrategyForked -vv
contract ShoebillTBTCStrategyForked is ForkedStrategyTemplateTbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            5607192, address(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C), Constants.DUMMY_SENDER, 1 ether
        );
    }

    function testShoebillStrategy() public {
        ICErc20 cErc20 = ICErc20(0x2925dF9Eb2092B53B06A06353A7249aF3a8B139e);
        ShoebillStrategy shoebillStrategy = new ShoebillStrategy(cErc20);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(shoebillStrategy), 1 ether);

        shoebillStrategy.handleGatewayMessageWithSlippageArgs(
            token, 1 ether, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0)
        );
        vm.stopPrank();

        // ToDo: remove the magic number
        assertEq(cErc20.balanceOfUnderlying(address(Constants.DUMMY_RECEIVER)), 999999999973746162);
    }
}
