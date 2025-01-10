// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

import {BedrockStrategy, IBedrockVault} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {ArbitaryErc20} from "./Utils.sol";

contract DummyBedrockVaultImplementation is IBedrockVault {
    ArbitaryErc20 uniBtcToken;
    bool private doMint;

    // Constructor with a flag to determine supply behavior
    constructor(ArbitaryErc20 _uniBtcToken, bool _doMint) {
        doMint = _doMint;
        uniBtcToken = _uniBtcToken;
    }

    function mint(address, /* token */ uint256 amount) external override {
        if (doMint) {
            uniBtcToken.transfer(msg.sender, amount);
        }
    }

    function redeem(address token, uint256 amount) external override {}

    function uniBTC() external view returns (address) {
        return address(uniBtcToken);
    }
}

contract BedrockStrategyTest is Test {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    ArbitaryErc20 wrappedBTC;
    ArbitaryErc20 uniBtcToken;

    function setUp() public {
        wrappedBTC = new ArbitaryErc20("", "");
        uniBtcToken = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 1 ether); // Mint 100 tokens to this contract
    }

    function testDepositTokenIntoVault() public {
        IBedrockVault vault = new DummyBedrockVaultImplementation(uniBtcToken, true);
        uniBtcToken.sudoMint(address(vault), 1 ether);

        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

        // Approve strategy to spend a token on behalf of this contract
        wrappedBTC.increaseAllowance(address(bedrockStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(uniBtcToken), 1 ether);
        bedrockStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testBedrockDepositFailsDueToInsufficientAmount() public {
        IBedrockVault vault = new DummyBedrockVaultImplementation(uniBtcToken, false);
        uniBtcToken.sudoMint(address(vault), 1 ether);

        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

        // Approve strategy to spend 100 tBTC tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(bedrockStrategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        bedrockStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }
}
