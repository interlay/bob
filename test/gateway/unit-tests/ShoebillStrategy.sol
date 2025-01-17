// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {ICErc20, ShoebillStrategy} from "../../../src/gateway/strategy/ShoebillStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {ArbitaryErc20} from "./Utils.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract DummyShoeBillToken is ICErc20, ERC20, Ownable {
    bool private doMint;
    bool private mintInsufficientAmount;

    constructor(string memory name_, string memory symbol_, bool _doMint) ERC20(name_, symbol_) {
        doMint = _doMint;
    }

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function mint(uint256 mintAmount) external returns (uint256) {
        if (doMint) {
            _mint(_msgSender(), mintAmount);
            return 0;
        }
        return 1;
    }

    function balanceOfUnderlying(address /*owner*/ ) external pure returns (uint256) {
        return 0;
    }
}

contract ShoebillStrategyTest is Test {
    ICErc20 shoeBillToken;
    ArbitaryErc20 wrappedBTC;

    event TokenOutput(address tokenReceived, uint256 amountOut);

    function setUp() public {
        shoeBillToken = new DummyShoeBillToken("", "", true);
        wrappedBTC = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 100 ether); // Mint 100 tokens to this contract
    }

    function testShoeBillStrategy() public {
        ShoebillStrategy strategy = new ShoebillStrategy(shoeBillToken);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(shoeBillToken), 1 ether);
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }

    function testUnableToMintShoeBillToken() public {
        shoeBillToken = new DummyShoeBillToken("", "", false);
        ShoebillStrategy strategy = new ShoebillStrategy(shoeBillToken);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectRevert("Could not mint token");
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether));
    }

    function testMintedInsufficientAmountOfShoeBillToken() public {
        ShoebillStrategy strategy = new ShoebillStrategy(shoeBillToken);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(strategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        strategy.handleGatewayMessageWithSlippageArgs(wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(2 ether));
    }
}
