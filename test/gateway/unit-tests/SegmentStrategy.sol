// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {
    ISeBep20,
    SegmentStrategy,
    SegmentBedrockStrategy,
    SegmentSolvLSTStrategy
} from "../../../src/gateway/strategy/SegmentStrategy.sol";
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {SolvLSTStrategy, ISolvBTCRouter} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {ArbitaryErc20} from "./Utils.sol";
import {DummyBedrockVaultImplementation} from "./BedrockStrategy.sol";
import {DummySolvRouter} from "./SolvStrategy.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract DummySeBep20 is ISeBep20, ERC20, Ownable {
    bool private doMint;
    bool private suppressMintError;

    constructor(string memory name_, string memory symbol_, bool _doMint, bool _suppressMintError)
        ERC20(name_, symbol_)
    {
        doMint = _doMint;
        suppressMintError = _suppressMintError;
    }

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function mint(uint256 /*mintAmount*/ ) external pure override returns (uint256) {
        return 0;
    }

    function mintBehalf(address receiver, uint256 mintAmount) external override returns (uint256) {
        if (suppressMintError) {
            return 0;
        }
        if (doMint) {
            _mint(receiver, mintAmount);
            return 0;
        }
        return 1;
    }

    function balanceOfUnderlying(address /*owner*/ ) external pure override returns (uint256) {
        return 0;
    }

    function redeem(uint256 /*redeemTokens*/ ) external pure override returns (uint256) {
        return 0;
    }
}

contract SegmentStrategyTest is Test {
    ArbitaryErc20 wrappedBTC;
    ArbitaryErc20 uniBtcToken;
    ArbitaryErc20 solvBTC;
    ArbitaryErc20 solvLST;

    event TokenOutput(address tokenReceived, uint256 amountOut);

    function setUp() public {
        wrappedBTC = new ArbitaryErc20("", "");
        uniBtcToken = new ArbitaryErc20("", "");
        solvBTC = new ArbitaryErc20("", "");
        solvLST = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 100 ether);
    }

    function testSegmentStrategy() public {
        ISeBep20 seBep20 = new DummySeBep20("", "", true, false);

        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(segmentStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(seBep20), 1 ether);
        segmentStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testSegmentStrategyCouldNotMint() public {
        ISeBep20 seBep20 = new DummySeBep20("", "", false, false);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(segmentStrategy), 1 ether);

        vm.expectRevert("Could not mint token");
        segmentStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testSegmentStrategyInsufficientSupply() public {
        ISeBep20 seBep20 = new DummySeBep20("", "", false, true);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(segmentStrategy), 1 ether);

        vm.expectRevert("Insufficient supply provided");
        segmentStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testSegmentStrategyInsufficientOutput() public {
        ISeBep20 seBep20 = new DummySeBep20("", "", true, false);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(segmentStrategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        segmentStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(2 ether)
        );
    }

    function testSegmentBedrockStrategy() public {
        ISeBep20 seBep20 = new DummySeBep20("", "", true, false);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);
        IBedrockVault vault = new DummyBedrockVaultImplementation(uniBtcToken, true);

        uniBtcToken.sudoMint(address(vault), 1 ether);

        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

        SegmentBedrockStrategy segmentBedrockStrategy = new SegmentBedrockStrategy(bedrockStrategy, segmentStrategy);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(segmentBedrockStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(seBep20), 1 ether);
        segmentBedrockStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testSegmentSolvLstStrategy() public {
        ISeBep20 seBep20 = new DummySeBep20("", "", true, false);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);

        ISolvBTCRouter btcRouter = new DummySolvRouter(true, solvBTC);
        ISolvBTCRouter lstRouter = new DummySolvRouter(true, solvLST);
        SolvLSTStrategy solvLSTAtrategy =
            new SolvLSTStrategy(btcRouter, lstRouter, bytes32(0), bytes32(0), solvBTC, solvLST);
        solvBTC.sudoMint(address(btcRouter), 1 ether);
        solvLST.sudoMint(address(lstRouter), 1 ether);

        SegmentSolvLSTStrategy segmentSolvLSTStrategy = new SegmentSolvLSTStrategy(solvLSTAtrategy, segmentStrategy);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(segmentSolvLSTStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(seBep20), 1 ether);
        segmentSolvLSTStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }
}
