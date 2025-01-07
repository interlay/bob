// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";

// Command to run this test with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract BedrockStrategyForked -vv

contract BedrockStrategyForked is Test {
    // Instantiate WBTC token using its address from Constants
    IERC20 token = IERC20(Constants.WBTC_ADDRESS);

    function setUp() public {
        // creating fork and fixing the block number, so the test can be repeated
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), 6077077);
        // Transfer 100 WBTC tokens to DUMMY_SENDER
        vm.prank(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F);
        token.transfer(Constants.DUMMY_SENDER, 100 * 1e8);
        vm.stopPrank();
    }

    function testBedrockStrategy() public {
        IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
        BedrockStrategy strategy = new BedrockStrategy(vault);

        // DUMMY_SENDER approves the strategy contract to spend 1 WBTC on their behalf
        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 * 1e8);
        vm.stopPrank();

        // DUMMY_SENDER sends 1 WBTC to the strategy with slippage arguments
        vm.prank(Constants.DUMMY_SENDER);
        strategy.handleGatewayMessageWithSlippageArgs(
            token,
            1e8, // Amount: 1 WBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        IERC20 uniBTC = IERC20(vault.uniBTC());
        assertEq(uniBTC.balanceOf(Constants.DUMMY_RECEIVER), 1e8, "User uniBTC balance mismatch");
    }
}
