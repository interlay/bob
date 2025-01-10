// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ISolvBTCRouter, SolvBTCStrategy, SolvLSTStrategy} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {ArbitaryErc20} from "./Utils.sol";

contract DummySolvRouter is ISolvBTCRouter {
    bool private doTransferAmount;
    ArbitaryErc20 private solvBTC;

    constructor(bool _doTransferAmount, ArbitaryErc20 _solvBTC) {
        doTransferAmount = _doTransferAmount;
        solvBTC = _solvBTC;
    }

    function createSubscription(bytes32, /* poolId */ uint256 amount) external override returns (uint256 shareValue) {
        if (doTransferAmount) {
            solvBTC.transfer(msg.sender, amount);
            return amount;
        }
        return 0;
    }
}

contract SolvStrategyTest is Test {
    ArbitaryErc20 wrappedBTC;
    ArbitaryErc20 solvBTC;
    ArbitaryErc20 solvLST;

    event TokenOutput(address tokenReceived, uint256 amountOut);

    function setUp() public {
        solvBTC = new ArbitaryErc20("", "");
        solvLST = new ArbitaryErc20("", "");
        wrappedBTC = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 1 ether); // Mint 100 tokens to this contract
    }

    function testSolvStrategy() public {
        ISolvBTCRouter router = new DummySolvRouter(true, solvBTC);
        SolvBTCStrategy strategy = new SolvBTCStrategy(router, bytes32(0), solvBTC);
        solvBTC.sudoMint(address(router), 1 ether);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(solvBTC), 1 ether);
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }

    function testSolvStrategyInsufficientOutputAmount() public {
        ISolvBTCRouter router = new DummySolvRouter(false, solvBTC);
        SolvBTCStrategy strategy = new SolvBTCStrategy(router, bytes32(0), solvBTC);
        solvBTC.sudoMint(address(router), 1 ether);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }

    function testSolvLstStrategy() public {
        ISolvBTCRouter btcRouter = new DummySolvRouter(true, solvBTC);
        ISolvBTCRouter lstRouter = new DummySolvRouter(true, solvLST);
        SolvLSTStrategy strategy = new SolvLSTStrategy(btcRouter, lstRouter, bytes32(0), bytes32(0), solvBTC, solvLST);

        solvBTC.sudoMint(address(btcRouter), 1 ether);
        solvLST.sudoMint(address(lstRouter), 1 ether);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(solvLST), 1 ether);
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }

    function testSolvLSTStrategyInsufficientOutputAmount() public {
        ISolvBTCRouter btcRouter = new DummySolvRouter(true, solvBTC);
        ISolvBTCRouter lstRouter = new DummySolvRouter(false, solvLST);
        SolvLSTStrategy strategy = new SolvLSTStrategy(btcRouter, lstRouter, bytes32(0), bytes32(0), solvBTC, solvLST);

        solvBTC.sudoMint(address(btcRouter), 1 ether);
        solvLST.sudoMint(address(lstRouter), 1 ether);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }
}
