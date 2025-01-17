// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {MarketPlace} from "../../src/swap/Marketplace.sol";
import {Utilities} from "./Utilities.sol";

contract ArbitaryErc20 is ERC20, Ownable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract MarketPlaceTest is MarketPlace, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    ArbitaryErc20 token1;
    ArbitaryErc20 token2;

    constructor() MarketPlace(address(0x00)) {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");

        token1 = new ArbitaryErc20("Bob Wrapped BTC", "zBTC");
        token2 = new ArbitaryErc20("Wrapped Ether", "WETH");
    }

    function testFullSwap() public {
        token1.sudoMint(alice, 1000);
        token2.sudoMint(bob, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 1000);
        this.placeErcErcOrder(address(token1), 600, address(token2), 60);

        vm.startPrank(bob);
        token2.approve(address(this), 100);
        this.acceptErcErcOrder(0, 60);

        assertEq(token1.balanceOf(alice), 400);
        assertEq(token2.balanceOf(alice), 60);
        assertEq(token1.balanceOf(bob), 600);
        assertEq(token2.balanceOf(bob), 40);
    }

    function testPartialSwap() public {
        token1.sudoMint(alice, 1000);
        token2.sudoMint(bob, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 1000);
        this.placeErcErcOrder(address(token1), 1000, address(token2), 100);

        vm.startPrank(bob);
        token2.approve(address(this), 100);
        this.acceptErcErcOrder(0, 75);

        assertEq(token1.balanceOf(alice), 0);
        assertEq(token2.balanceOf(alice), 75);
        assertEq(token1.balanceOf(bob), 750);
        assertEq(token2.balanceOf(bob), 25);
    }

    function testWithdraw() public {
        token1.sudoMint(alice, 1000);

        vm.startPrank(alice);
        token1.approve(address(this), 1000);
        this.placeErcErcOrder(address(token1), 1000, address(token2), 100);

        assertEq(token1.balanceOf(alice), 0);
        this.withdrawErcErcOrder(0);
        assertEq(token1.balanceOf(alice), 1000);
    }
}
