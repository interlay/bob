// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
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

using SafeERC20 for IERC20;

contract HelloBitcoinTest is HelloBitcoin, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;
    ArbitaryUsdtToken usdtToken = new ArbitaryUsdtToken("0xF58de5056b7057D74f957e75bFfe865F571c3fB6", "USDT");

    constructor() HelloBitcoin(testLightRelay, address(usdtToken)) {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);
        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");
        testLightRelay = new TestLightRelay();
        super.setRelay(testLightRelay);
    }

    function dummyBitcoinAddress() public returns (BitcoinAddress memory) {
        return BitcoinAddress({scriptPubKey: hex"76a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac"});
    }

    function dummyOrdinalBitcoinAddress() public returns (BitcoinAddress memory) {
        return BitcoinAddress({scriptPubKey: hex"0014e257eccafbc07c381642ce6e7e55120fb077fbed"});
    }

    function test_btcSellOrderFullFlow() public {
        usdtToken.sudoMint(bob, 100);

        vm.startPrank(alice);
        vm.expectEmit();
        emit swapBtcForUsdtEvent(0, 1000, 10);
        this.swapBtcForUsdt(1000, 10);

        vm.startPrank(bob);
        usdtToken.approve(address(this), 1000);
        vm.expectEmit();
        emit acceptBtcToUsdtSwapEvent(0, dummyBitcoinAddress());
        this.acceptBtcToUsdtSwap(0, dummyBitcoinAddress());

        vm.startPrank(alice);
        vm.expectEmit();
        emit proofBtcSendtoDestinationEvent(0);
        this.proofBtcSendtoDestination(0, utils.dummyTransaction(), utils.dummyProof());
    }

    function test_ordinalSellOrderFullFlow() public {
        (BitcoinTx.Info memory info, BitcoinTx.Proof memory proof, BitcoinTx.UTXO memory utxo) =
            utils.dummyOrdinalInfo();
        OrdinalId memory id;

        usdtToken.sudoMint(bob, 100);

        // swapOrdinalToUsdt by alice
        vm.startPrank(alice);
        vm.expectEmit();
        emit swapOrdinalToUsdtEvent(0, id, 100);
        this.swapOrdinalToUsdt(id, utxo, 100);

        // acceptOrdinalToUsdtSwap by bob
        vm.startPrank(bob);
        usdtToken.approve(address(this), 100);
        vm.expectEmit();
        emit acceptOrdinalToUsdtSwapEvent(0, dummyOrdinalBitcoinAddress());
        this.acceptOrdinalToUsdtSwap(0, dummyOrdinalBitcoinAddress());

        vm.startPrank(alice);
        vm.expectEmit();
        emit proofOrdinalSellOrderEvent(0);
        this.proofOrdinalSendtoDestination(0, info, proof);
    }
}
