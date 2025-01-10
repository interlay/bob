// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract BedrockStrategyForked -vv
contract BedrockStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6077077, address(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testBedrockStrategy() public {
        IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
        BedrockStrategy strategy = new BedrockStrategy(vault);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);
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
