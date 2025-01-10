// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {
    IAvalonIPool, AvalonLendingStrategy, AvalonLstStrategy
} from "../../../src/gateway/strategy/AvalonStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateTbtc, ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";
import {SolvLSTStrategy, ISolvBTCRouter} from "../../../src/gateway/strategy/SolvStrategy.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract AvalonTBTCLendingStrategyForked -vv
contract AvalonTBTCLendingStrategyForked is ForkedStrategyTemplateTbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6077077, address(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C), Constants.DUMMY_SENDER, 1 ether
        );
    }

    function testAvalonTBTCStrategy() public {
        // Instantiate the Avalon TBTC token and pool contracts
        IERC20 avalonTBTCToken = IERC20(0x5E007Ed35c7d89f5889eb6FD0cdCAa38059560ef);
        IAvalonIPool pool = IAvalonIPool(0x35B3F1BFe7cbE1e95A3DC2Ad054eB6f0D4c879b6);

        // Deploy a new AvalonLendingStrategy contract
        AvalonLendingStrategy strategy = new AvalonLendingStrategy(avalonTBTCToken, pool);

        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 ether);
        vm.stopPrank();

        vm.prank(Constants.DUMMY_SENDER);
        strategy.handleGatewayMessageWithSlippageArgs(token, 1 ether, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        // Assert that receivers token balance is still 0 (funds are in the pool)
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0 ether);

        // receiver withdraws Received tokens from the pool
        vm.prank(Constants.DUMMY_RECEIVER);
        pool.withdraw(address(token), 1 ether, Constants.DUMMY_RECEIVER);
        vm.stopPrank();

        // Assert that receiver now has 1 token in their balance
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 1 ether);
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract AvalonWBTCLendingStrategyForked -vv
contract AvalonWBTCLendingStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6216882, address(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testAvalonWBTCStrategy() public {
        // Instantiate the Avalon WBTC token and pool contracts
        IERC20 avalonWBTCToken = IERC20(0xd6890176e8d912142AC489e8B5D8D93F8dE74D60);
        IAvalonIPool pool = IAvalonIPool(0x35B3F1BFe7cbE1e95A3DC2Ad054eB6f0D4c879b6);

        // Deploy a new AvalonLendingStrategy contract
        AvalonLendingStrategy strategy = new AvalonLendingStrategy(avalonWBTCToken, pool);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 * 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        // Assert that receiver's token balance is still 0 (funds are in the pool)
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0);

        // receiver withdraws Received WBTC from the pool
        vm.prank(Constants.DUMMY_RECEIVER);
        pool.withdraw(address(token), 1e8, Constants.DUMMY_RECEIVER);

        // Assert that receiver now has 1 WBTC in their balance
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 1e8);
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract AvalonWBTCLstStrategyForked -vv
contract AvalonWBTCLstStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6216882, address(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F), Constants.DUMMY_SENDER, 1e8
        );
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

        // Deploy a new AvalonLstStrategy contract
        AvalonLstStrategy avalonLstStrategy = new AvalonLstStrategy(solvLSTStrategy, avalonLendingStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(avalonLstStrategy), 1e8);

        // check before calling receiver has not avalonSolvBtcBBN tokens
        assertEq(avalonSolvBtcBBNToken.balanceOf(address(Constants.DUMMY_RECEIVER)), 0);

        avalonLstStrategy.handleGatewayMessageWithSlippageArgs(
            token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0)
        );
        vm.stopPrank();

        assertEq(avalonSolvBtcBBNToken.balanceOf(address(Constants.DUMMY_RECEIVER)), 1 ether);
    }
}
