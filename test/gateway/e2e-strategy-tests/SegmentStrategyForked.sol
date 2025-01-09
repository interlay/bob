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
import {
    ISeBep20,
    SegmentStrategy,
    SegmentBedrockStrategy,
    SegmentSolvLSTStrategy
} from "../../../src/gateway/strategy/SegmentStrategy.sol";
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {SolvLSTStrategy, ISolvBTCRouter} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {Constants} from "./Constants.sol";

contract SegmentStrategyForked is Test {
    // Instantiate TBTC token using its address from Constants
    IERC20 token = IERC20(Constants.TBTC_ADDRESS);

    function setUp() public {
        // Set up the test environment by creating a fork of the BOB_PROD_PUBLIC_RPC_URL
        // Fixing the block number ensures reproducible test results
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), 5607192);

        // Transfer 100 TBTC tokens to DUMMY_SENDER
        vm.prank(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C);
        token.transfer(Constants.DUMMY_SENDER, 100 ether);
        vm.stopPrank();
    }

    function testSegmentStrategy() public {
        ISeBep20 seBep20 = ISeBep20(0xD30288EA9873f376016A0250433b7eA375676077);
        SegmentStrategy strategy = new SegmentStrategy(seBep20);

        vm.prank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 ether);

        vm.prank(Constants.DUMMY_SENDER);
        strategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 ether, // Amount: 1 TBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );

        uint256 userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertGt(userSeBalance, 0, "User has seTokens");
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no TBTC tokens");

        vm.prank(Constants.DUMMY_RECEIVER);
        seBep20.redeem(userSeBalance);

        assertGt(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User received TBTC tokens after redeem");
    }
}

contract SegmentBedrockAndLstStrategyForked is Test {
    // Instantiate WBTC token using its address from Constants
    IERC20 token = IERC20(Constants.WBTC_ADDRESS);

    function setUp() public {
        // Set up the test environment by creating a fork of the BOB_PROD_PUBLIC_RPC_URL
        // Fixing the block number ensures reproducible test results
        vm.createSelectFork(vm.envString("BOB_PROD_PUBLIC_RPC_URL"), 6945930);

        // Transfer 100 WBTC tokens to DUMMY_SENDER
        vm.prank(0x5A8E9774d67fe846C6F4311c073e2AC34b33646F);
        token.transfer(Constants.DUMMY_SENDER, 100 * 1e8);
        vm.stopPrank();
    }

    function testSegmentBedrockStrategy() public {
        IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);
        ISeBep20 seBep20 = ISeBep20(0x7848F0775EebaBbF55cB74490ce6D3673E68773A);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);
        SegmentBedrockStrategy strategy = new SegmentBedrockStrategy(bedrockStrategy, segmentStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        uint256 userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertGt(userSeBalance, 0, "User has seTokens");
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no WBTC tokens");

        IERC20 uniBTC = IERC20(vault.uniBTC());
        assertEq(uniBTC.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no uniBTC tokens");

        vm.prank(Constants.DUMMY_RECEIVER);
        seBep20.redeem(userSeBalance);

        userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertEq(userSeBalance, 0, "User has redeemed");

        assertGt(uniBTC.balanceOf(Constants.DUMMY_RECEIVER), 0, "User increase in uniBTC Balance");
    }

    function testSegmentSolvLstStrategy() public {
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
        ISeBep20 seBep20 = ISeBep20(0x5EF2B8fbCc8aea2A9Dbe2729F0acf33E073Fa43e);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);
        SegmentSolvLSTStrategy strategy = new SegmentSolvLSTStrategy(solvLSTStrategy, segmentStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        uint256 userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertGt(userSeBalance, 0, "User has seTokens");
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no WBTC tokens");

        vm.prank(Constants.DUMMY_RECEIVER);
        seBep20.redeem(userSeBalance);

        userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertEq(userSeBalance, 0, "User has redeemed");
        assertGt(solvBTCBBN.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has SolvBTC.BBN tokens");
    }
}
