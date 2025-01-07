// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IAvalonIPool, AvalonLendingStrategy} from "../../../src/gateway/strategy/AvalonStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";

// Command to run this test with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract AvalonTBTCLendingStrategyForked -vv

contract AvalonTBTCLendingStrategyForked is Test {
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

    function testAvalonTBTCStrategy() public {
        // Instantiate the Avalon TBTC token and pool contracts
        IERC20 avalonTBTCToken = IERC20(0x5E007Ed35c7d89f5889eb6FD0cdCAa38059560ef);
        IAvalonIPool pool = IAvalonIPool(0x35B3F1BFe7cbE1e95A3DC2Ad054eB6f0D4c879b6);

        // Deploy a new AvalonLendingStrategy contract
        AvalonLendingStrategy strategy = new AvalonLendingStrategy(avalonTBTCToken, pool);

        // DUMMY_SENDER approves the strategy contract to spend 1 TBTC on their behalf
        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 ether);
        vm.stopPrank();

        // DUMMY_SENDER sends 1 TBTC to the strategy with slippage arguments
        vm.prank(Constants.DUMMY_SENDER);
        strategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 ether, // Amount: 1 TBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        // Assert that DUMMY_RECEIVER's token balance is still 0 (funds are in the pool)
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0 ether);

        // DUMMY_RECEIVER withdraws Received TBTC from the pool
        vm.prank(Constants.DUMMY_RECEIVER);
        pool.withdraw(address(token), 1 ether, Constants.DUMMY_RECEIVER);
        vm.stopPrank();

        // Assert that DUMMY_RECEIVER now has 1 TBTC in their balance
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 1 ether);
    }
}

// Command to run this test with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract AvalonWBTCLendingStrategyForked -vv

contract AvalonWBTCLendingStrategyForked is Test {
    // Instantiate TBTC token using its address from Constants
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

    function testAvalonWBTCStrategy() public {
        // Instantiate the Avalon WBTC token and pool contracts
        IERC20 avalonWBTCToken = IERC20(0xd6890176e8d912142AC489e8B5D8D93F8dE74D60);
        IAvalonIPool pool = IAvalonIPool(0x35B3F1BFe7cbE1e95A3DC2Ad054eB6f0D4c879b6);

        // Deploy a new AvalonLendingStrategy contract
        AvalonLendingStrategy strategy = new AvalonLendingStrategy(avalonWBTCToken, pool);

        // DUMMY_SENDER approves the strategy contract to spend 1 WBTC on their behalf
        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 * 1e8);
        vm.stopPrank();

        // DUMMY_SENDER sends 1 WBTC to the strategy with slippage arguments
        vm.prank(Constants.DUMMY_SENDER);
        strategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 * 1e8, // Amount: 1 WBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        // Assert that DUMMY_RECEIVER's token balance is still 0 (funds are in the pool)
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0);

        // DUMMY_RECEIVER withdraws Received WBTC from the pool
        vm.prank(Constants.DUMMY_RECEIVER);
        pool.withdraw(address(token), 1 * 1e8, Constants.DUMMY_RECEIVER);
        vm.stopPrank();

        // Assert that DUMMY_RECEIVER now has 1 WBTC in their balance
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 1 * 1e8);
    }
}
