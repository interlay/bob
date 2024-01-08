// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {Utilities} from "./swap/Utilities.sol";
import {BitcoinTx} from "../src/bridge/BitcoinTx.sol";
import {TestLightRelay} from "../src/relay/TestLightRelay.sol";
import {HelloBitcoin} from "../src/hello-bitcoin/HelloBitcoin.sol";

contract ArbitaryUsdtToken is ERC20, Ownable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract HelloBitcoinTest is HelloBitcoin, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;
    ArbitaryUsdtToken usdtToken;

    address public constant dummyUsdtContractAddress = 0xF58de5056b7057D74f957e75bFfe865F571c3fB6;

    constructor() HelloBitcoin(testLightRelay, dummyUsdtContractAddress) {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");

        usdtToken = new ArbitaryUsdtToken("0xF58de5056b7057D74f957e75bFfe865F571c3fB6", "USDT");

        testLightRelay = new TestLightRelay();
        super.setRelay(testLightRelay);
    }

    function test_btcSellOrderFullFlow() public {
        //ToDo: shift the dummy bitcoin address/tx/proof in utilities
    }

    function test_ordinalSellOrderFullFlow() public {
        //ToDo: shift the dummy bitcoin address/tx/proof/ordinal ids in utilities
    }
}
