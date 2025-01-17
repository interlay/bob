// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ArbitaryErc20} from "./Utils.sol";
import {
    IAvalonIPool, AvalonLendingStrategy, AvalonLstStrategy
} from "../../../src/gateway/strategy/AvalonStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {ISolvBTCRouter, SolvLSTStrategy} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {DummySolvRouter} from "./SolvStrategy.sol";

contract DummyAvalonPoolImplementation is IAvalonIPool {
    ArbitaryErc20 avalonToken;
    bool private doSupply;

    // Constructor with a flag to determine supply behavior
    constructor(ArbitaryErc20 _avalonToken, bool _doSupply) {
        doSupply = _doSupply;
        avalonToken = _avalonToken;
    }

    // Supply function behavior changes based on the flag
    function supply(address, /* asset */ uint256 amount, address onBehalfOf, uint16 /* referralCode */ )
        external
        override
    {
        if (doSupply) {
            // Supply logic for DummyAvalonPoolImplementation: transfers tokens
            avalonToken.transfer(onBehalfOf, amount);
        }
        // If doSupply is false, no supply action is taken (DummyAvalonPoolImplementation behavior)
    }

    function withdraw(address, uint256, address /* to */ ) external pure override returns (uint256) {
        return 0;
    }
}

contract AvalonStrategyTest is Test {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    ArbitaryErc20 wrappedBTC;
    ArbitaryErc20 avalonToken;
    ArbitaryErc20 solvLST;

    function setUp() public {
        wrappedBTC = new ArbitaryErc20("", "");
        avalonToken = new ArbitaryErc20("", "");
        solvLST = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 100 ether); // Mint 100 tokens to this contract
    }

    function testLendingStrategyForValidAmount() public {
        IAvalonIPool dummyPool = new DummyAvalonPoolImplementation(avalonToken, true);
        avalonToken.sudoMint(address(dummyPool), 100 ether);

        AvalonLendingStrategy avalonStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        // Approve ionicStrategy to spend a token on behalf of this contract
        wrappedBTC.increaseAllowance(address(avalonStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(avalonToken), 1 ether);
        avalonStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );

        assertEq(avalonToken.balanceOf(vm.addr(1)), 1 ether);
        assertEq(wrappedBTC.balanceOf(address(this)), 99 ether);
    }

    function testWhenInsufficientSupplyProvided() public {
        IAvalonIPool dummyPool = new DummyAvalonPoolImplementation(avalonToken, false);
        AvalonLendingStrategy avalonStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        // Approve ionicStrategy to spend 100 tBTC tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(avalonStrategy), 1 ether);

        vm.expectRevert("Insufficient supply provided");
        avalonStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testWhenInsufficientOutputAmount() public {
        IAvalonIPool dummyPool = new DummyAvalonPoolImplementation(avalonToken, true);
        avalonToken.sudoMint(address(dummyPool), 100 ether); // Mint 100 tokens to this contract

        AvalonLendingStrategy avalonStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        wrappedBTC.increaseAllowance(address(avalonStrategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        avalonStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether + 1)
        );
    }

    function testAvalonLSTStrategyForValidAmount() public {
        ISolvBTCRouter btcRouter = new DummySolvRouter(true, wrappedBTC);
        ISolvBTCRouter lstRouter = new DummySolvRouter(true, solvLST);
        SolvLSTStrategy solvStrategy =
            new SolvLSTStrategy(btcRouter, lstRouter, bytes32(0), bytes32(0), wrappedBTC, solvLST);

        wrappedBTC.sudoMint(address(btcRouter), 1 ether);
        solvLST.sudoMint(address(lstRouter), 1 ether);

        IAvalonIPool dummyPool = new DummyAvalonPoolImplementation(avalonToken, true);
        avalonToken.sudoMint(address(dummyPool), 1 ether); // Mint 100 tokens to this contract

        AvalonLendingStrategy avalonLendingStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        AvalonLstStrategy avalonLSTStrategy = new AvalonLstStrategy(solvStrategy, avalonLendingStrategy);

        wrappedBTC.increaseAllowance(address(avalonLSTStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(avalonToken), 1 ether);
        avalonLSTStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }
}
