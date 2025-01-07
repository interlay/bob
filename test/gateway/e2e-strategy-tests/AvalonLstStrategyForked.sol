// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {
    IAvalonIPool, AvalonLendingStrategy, AvalonLstStrategy
} from "../../../src/gateway/strategy/AvalonStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {SolvLSTStrategy, ISolvBTCRouter} from "../../../src/gateway/strategy/SolvStrategy.sol";

// Command to run this test with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract AvalonWBTCLstStrategyForked -vv

contract AvalonWBTCLstStrategyForked is Test {
    // Instantiate WBTC token using its address from Constants
    IERC20 token = IERC20(Constants.WBTC_ADDRESS);

    function setUp() public {
        // Set up the test environment by creating a fork of the BOB_PROD_PUBLIC_RPC_URL
        // Fixing the block number ensures reproducible test results
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), 6216882);

        // Transfer 100 WBTC tokens to DUMMY_SENDER
        vm.prank(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F);
        token.transfer(Constants.DUMMY_SENDER, 100 * 1e8);
        vm.stopPrank();
    }

    function testWbtcLstStrategy() public {
        IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
        IERC20 solvBTCBBN = IERC20(0xCC0966D8418d412c599A6421b760a847eB169A8c);
        SolvLSTStrategy solvLSTStrategy = new SolvLSTStrategy(
            ISolvBTCRouter(0x49b072158564Db36304518FFa37B1cFc13916A90),
            ISolvBTCRouter(0xbA46FcC16B464D9787314167bDD9f1Ce28405bA1),
            0x5664520240a46b4b3e9655c20cc3f9e08496a9b746a478e476ae3e04d6c8fc31,
            0x6899a7e13b655fa367208cb27c6eaa2410370d1565dc1f5f11853a1e8cbef033,
            solvBTC,
            solvBTCBBN
        );

        IERC20 avalonSolvBtcBBNToken = IERC20(0x2E6500A7Add9a788753a897e4e3477f651c612eb);
        IAvalonIPool pool = IAvalonIPool(0x35B3F1BFe7cbE1e95A3DC2Ad054eB6f0D4c879b6);
        AvalonLendingStrategy avalonLendingStrategy = new AvalonLendingStrategy(avalonSolvBtcBBNToken, pool);

        AvalonLstStrategy avalonLstStrategy = new AvalonLstStrategy(solvLSTStrategy, avalonLendingStrategy);

        // DUMMY_SENDER approves the strategy contract to spend 1 WBTC on their behalf
        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(avalonLstStrategy), 1 * 1e8);
        vm.stopPrank();

        assertEq(avalonSolvBtcBBNToken.balanceOf(address(Constants.DUMMY_RECEIVER)), 0);

        vm.prank(Constants.DUMMY_SENDER);
        avalonLstStrategy.handleGatewayMessageWithSlippageArgs(
            token, 1 * 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0)
        );
        vm.stopPrank();

        assertEq(avalonSolvBtcBBNToken.balanceOf(address(Constants.DUMMY_RECEIVER)), 1 ether);
    }
}
