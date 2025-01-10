// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {
    IPellStrategyManager,
    IPellStrategy,
    PellStrategy,
    PellBedrockStrategy,
    PellSolvLSTStrategy
} from "../../../src/gateway/strategy/PellStrategy.sol";
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {SolvLSTStrategy, ISolvBTCRouter} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateTbtc, ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract PellStrategyForked -vv
contract PellStrategyForked is ForkedStrategyTemplateTbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6077077, address(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C), Constants.DUMMY_SENDER, 1 ether
        );
    }

    function testPellStrategy() public {
        PellStrategy pellStrategy = new PellStrategy(
            IPellStrategyManager(0x00B67E4805138325ce871D5E27DC15f994681bC1),
            IPellStrategy(0x0a5e1Fe85BE84430c6eb482512046A04b25D2484)
        );

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(pellStrategy), 1 ether);

        pellStrategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 ether, // Amount: 1 TBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        assertEq(pellStrategy.stakerStrategyShares(Constants.DUMMY_RECEIVER), 1 ether);
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract PellBedRockStrategyForked -vv
contract PellBedRockStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6642119, address(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testPellBedrockStrategy() public {
        IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

        PellStrategy pellStrategy = new PellStrategy(
            IPellStrategyManager(0x00B67E4805138325ce871D5E27DC15f994681bC1),
            IPellStrategy(0x631ae97e24f9F30150d31d958d37915975F12ed8)
        );

        PellBedrockStrategy pellBedrockstrategy = new PellBedrockStrategy(bedrockStrategy, pellStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(pellBedrockstrategy), 1e8);

        pellBedrockstrategy.handleGatewayMessageWithSlippageArgs(
            token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0)
        );
        vm.stopPrank();

        assertEq(pellStrategy.stakerStrategyShares(Constants.DUMMY_RECEIVER), 1e8);
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract PellBedRockLSTStrategyForked -vv
contract PellBedRockLSTStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6642119, address(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testPellBedrockLSTStrategy() public {
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

        PellStrategy pellStrategy = new PellStrategy(
            IPellStrategyManager(0x00B67E4805138325ce871D5E27DC15f994681bC1),
            IPellStrategy(0x6f0AfADE16BFD2E7f5515634f2D0E3cd03C845Ef)
        );

        PellSolvLSTStrategy strategy = new PellSolvLSTStrategy(solvLSTStrategy, pellStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(
            token,
            1e8, // Amount: 1 WBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        assertEq(pellStrategy.stakerStrategyShares(Constants.DUMMY_RECEIVER), 1e18);
    }
}
