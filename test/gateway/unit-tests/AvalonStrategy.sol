// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

import {IAvalonIPool, AvalonLendingStrategy} from "../../../src/gateway/strategy/AvalonStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";

contract DummyPoolImplementation is IAvalonIPool {
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
            // Supply logic for DummyPoolImplementation2: transfers tokens
            avalonToken.transfer(onBehalfOf, amount);
        }
        // If doSupply is false, no supply action is taken (DummyPoolImplementation behavior)
    }

    // Withdraw function (unchanged in both cases)
    function withdraw(address, uint256, address /* to */ ) external pure override returns (uint256) {
        return 0;
    }
}

contract ArbitaryErc20 is ERC20, Ownable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    // Mint function accessible only to the owner
    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount); // Provided by the OpenZeppelin ERC20 contract
    }
}

// forge test --match-contract AvalonLendingStrategyTest -vv
contract AvalonLendingStrategyTest is Test {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    ArbitaryErc20 lendingToken;
    ArbitaryErc20 avalonToken;

    function setUp() public {
        lendingToken = new ArbitaryErc20("Lending Token", "Lending Token");
        avalonToken = new ArbitaryErc20("Avalon Token", "Avalon Token");
        lendingToken.sudoMint(address(this), 100 ether); // Mint 100 tokens to this contract
    }

    function testLendingStrategyForValidAmount() public {
        IAvalonIPool dummyPool = new DummyPoolImplementation(avalonToken, true);
        avalonToken.sudoMint(address(dummyPool), 100 ether); // Mint 100 tokens to this contract

        AvalonLendingStrategy avalonStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        // Approve ionicStrategy to spend 100 tBTC tokens on behalf of this contract
        lendingToken.increaseAllowance(address(avalonStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(avalonToken), 1 ether);
        avalonStrategy.handleGatewayMessageWithSlippageArgs(
            lendingToken, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );

        assertEq(avalonToken.balanceOf(vm.addr(1)), 1 ether);
        assertEq(lendingToken.balanceOf(address(this)), 99 ether);
    }

    function testWhenInsufficientSupplyProvided() public {
        IAvalonIPool dummyPool = new DummyPoolImplementation(avalonToken, false);
        AvalonLendingStrategy avalonStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        // Approve ionicStrategy to spend 100 tBTC tokens on behalf of this contract
        lendingToken.increaseAllowance(address(avalonStrategy), 100);

        vm.expectRevert("Insufficient supply provided");
        avalonStrategy.handleGatewayMessageWithSlippageArgs(lendingToken, 100, vm.addr(1), StrategySlippageArgs(0));
    }

    function testWhenInsufficientOutputAmount() public {
        IAvalonIPool dummyPool = new DummyPoolImplementation(avalonToken, true);
        avalonToken.sudoMint(address(dummyPool), 100 ether); // Mint 100 tokens to this contract

        AvalonLendingStrategy avalonStrategy = new AvalonLendingStrategy(avalonToken, dummyPool);

        // Approve ionicStrategy to spend 100 tBTC tokens on behalf of this contract
        lendingToken.increaseAllowance(address(avalonStrategy), 100);

        vm.expectRevert("Insufficient output amount");
        avalonStrategy.handleGatewayMessageWithSlippageArgs(
            lendingToken, 100, vm.addr(1), StrategySlippageArgs(100 + 1)
        );
    }
}

// forge test --match-contract AvalonLstStrategyTest -vv
contract AvalonLstStrategyTest is Test {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    ArbitaryErc20 lendingToken;
    ArbitaryErc20 avalonToken;

    function setUp() public {
        //ToDo: When solv lst stragey unit test completed
    }
}
