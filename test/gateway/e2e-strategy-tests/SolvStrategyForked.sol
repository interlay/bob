// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ISolvBTCRouter, SolvBTCStrategy, SolvLSTStrategy} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract SolvStrategyForked -vv
contract SolvStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6077077, address(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testSolvBTCStrategy() public {
        IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
        SolvBTCStrategy strategy = new SolvBTCStrategy(
            ISolvBTCRouter(0x49b072158564Db36304518FFa37B1cFc13916A90),
            0x5664520240a46b4b3e9655c20cc3f9e08496a9b746a478e476ae3e04d6c8fc31,
            solvBTC
        );

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 * 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        assertEq(solvBTC.balanceOf(Constants.DUMMY_RECEIVER), 1 ether);
    }

    function testSolvLSTStrategy() public {
        IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
        IERC20 solvBTCBBN = IERC20(0xCC0966D8418d412c599A6421b760a847eB169A8c);
        SolvLSTStrategy strategy = new SolvLSTStrategy(
            ISolvBTCRouter(0x49b072158564Db36304518FFa37B1cFc13916A90),
            ISolvBTCRouter(0xbA46FcC16B464D9787314167bDD9f1Ce28405bA1),
            0x5664520240a46b4b3e9655c20cc3f9e08496a9b746a478e476ae3e04d6c8fc31,
            0x6899a7e13b655fa367208cb27c6eaa2410370d1565dc1f5f11853a1e8cbef033,
            solvBTC,
            solvBTCBBN
        );

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 * 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        assertEq(solvBTCBBN.balanceOf(Constants.DUMMY_RECEIVER), 1 ether);
    }
}
