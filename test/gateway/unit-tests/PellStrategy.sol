// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
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
import {ArbitaryErc20} from "./Utils.sol";
import {DummyBedrockVaultImplementation} from "./BedrockStrategy.sol";
import {DummySolvRouter} from "./SolvStrategy.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract DummyPellStrategyManager is IPellStrategyManager {
    function depositIntoStrategyWithStaker(
        address, /*staker*/
        IPellStrategy, /*strategy*/
        IERC20, /*token*/
        uint256 amount
    ) external pure returns (uint256 shares) {
        return amount;
    }

    function stakerStrategyShares(address, /*staker*/ IPellStrategy /*strategy*/ ) external pure returns (uint256) {
        return 0;
    }
}

// Dummy implementation of IPellStrategy
contract DummyPellStrategy is IPellStrategy {}

contract PellStrategyTest is Test {
    ArbitaryErc20 shareToken;
    ArbitaryErc20 wrappedBTC;
    ArbitaryErc20 solvLST;
    ArbitaryErc20 uniBtcToken;

    event TokenOutput(address tokenReceived, uint256 amountOut);

    function setUp() public {
        shareToken = new ArbitaryErc20("", "");
        solvLST = new ArbitaryErc20("", "");
        wrappedBTC = new ArbitaryErc20("", "");
        uniBtcToken = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 100 ether); // Mint 100 tokens to this contract
    }

    function testPellStrategy() public {
        IPellStrategyManager pellStrategyManager = new DummyPellStrategyManager();
        IPellStrategy pellStrategy = new DummyPellStrategy();
        PellStrategy strategy = new PellStrategy(pellStrategyManager, pellStrategy);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        wrappedBTC.sudoMint(address(pellStrategyManager), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(0), 1 ether);
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }

    function testPellStrategyInsufficientShareIncrease() public {
        IPellStrategyManager pellStrategyManager = new DummyPellStrategyManager();
        IPellStrategy pellStrategy = new DummyPellStrategy();
        PellStrategy strategy = new PellStrategy(pellStrategyManager, pellStrategy);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        wrappedBTC.sudoMint(address(pellStrategyManager), 1 ether);

        vm.expectRevert("Insufficient output amount");
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(2 ether));
    }

    function testPellBedrockStrategy() public {
        IPellStrategyManager pellStrategyManager = new DummyPellStrategyManager();
        IPellStrategy pellStrategyI = new DummyPellStrategy();
        PellStrategy pellStrategy = new PellStrategy(pellStrategyManager, pellStrategyI);

        IBedrockVault vault = new DummyBedrockVaultImplementation(uniBtcToken, true);
        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

        PellBedrockStrategy pellBedrockStrategy = new PellBedrockStrategy(bedrockStrategy, pellStrategy);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(pellBedrockStrategy), 1 ether);
        uniBtcToken.sudoMint(address(vault), 1 ether);
        wrappedBTC.sudoMint(address(pellStrategyManager), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(0), 1 ether);
        pellBedrockStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testPellSolvLstStrategy() public {
        ISolvBTCRouter btcRouter = new DummySolvRouter(true, wrappedBTC);
        ISolvBTCRouter lstRouter = new DummySolvRouter(true, solvLST);
        SolvLSTStrategy solvLSTstrategy =
            new SolvLSTStrategy(btcRouter, lstRouter, bytes32(0), bytes32(0), wrappedBTC, solvLST);

        wrappedBTC.sudoMint(address(btcRouter), 1 ether);
        solvLST.sudoMint(address(lstRouter), 1 ether);

        IPellStrategyManager pellStrategyManager = new DummyPellStrategyManager();
        IPellStrategy pellStrategyI = new DummyPellStrategy();
        PellStrategy pellStrategy = new PellStrategy(pellStrategyManager, pellStrategyI);

        PellSolvLSTStrategy pellLSTStrategy = new PellSolvLSTStrategy(solvLSTstrategy, pellStrategy);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(pellLSTStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(0), 1 ether);
        pellLSTStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }
}
