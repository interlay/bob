// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {BtcMarketPlace} from "../../src/swap/Btc_Marketplace.sol";
import {Utilities} from "./Utilities.sol";
import {BitcoinTx} from "../../src/bridge/BitcoinTx.sol";
import {TestLightRelay} from "../../src/relay/TestLightRelay.sol";

contract ArbitaryErc20 is ERC20, Ownable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract MarketPlaceTest is BtcMarketPlace, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    ArbitaryErc20 token1;

    constructor() BtcMarketPlace(testLightRelay, address(0x00)) {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");

        token1 = new ArbitaryErc20("Some token", "TKN");

        testLightRelay = new TestLightRelay();
        super.setRelay(testLightRelay);
    }

    function dummyTransaction() public pure returns (BitcoinTx.Info memory) {
        return BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"01996cf4e2f0016a1f092aaaba653c7eae5dd4b6eef1f9a2a94c64f34b2fecbd85010000006a47304402206f99da49ce586528ed8981842df30b4a5a91195fd2d83e440d4193fc16a944ec022055cfdf63a2c90638821f1b5ff1fdf77526163ae057a0d0de30a6e1d3009e7a29012102811832eef7216470f489991f1d87e36d2890755d2bbf827eb1e71804491506afffffffff",
            outputVector: hex"0200e9a435000000001976a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac804a5d05000000001976a9145c1addbd0e4e78479e71fdca0555d2d44b67378e88ac",
            locktime: hex"00000000"
        });
    }

    function dummyProof() public pure returns (BitcoinTx.Proof memory) {
        return BitcoinTx.Proof({
            merkleProof: hex"0465f99dbe384bbc5d86a5242712e4154958e4b01f595f14b76f873ec349e14a16b17770af2bb48c9b2ce4dddf4631866fe3753e6c54bdcf18dfb2d4fb9983ee58e4f3be92087c843b815bbe1d5d686dc972552f7ffda4342319ceb5bea67ab0f2e463ec8ce8e3f580c5e2470ef20c5b33398ab9fea5ccbd0b3e3f6211305edafa068a28c8ac634df5bbc8064357295373b97db2600745f23ad6ebc87b66b4a8685aa8ff8e69abc5029dbf4b2fa03f05680c7a2c491410b23a5a6b27c5a91b89dac8cdd16a4460ce8ac8d17491025d29336440a133867f938a7f41cc7a64f3f04ac3817c3eb6a6a11dc30850ca4e80f9abbd42268bcc626138bc01639a902713425e7d3aca45647001fb32ff396c07027c5b081325530e74f936e6c4a8078a05f9717efd315534a84d047ee2ff0b2b93159a2b98eabb578af67ef7540a58e488b9c587a994c1a9a86937ad343ea734b7427678e3e6ba0be8f5045ce47e541bbc",
            txIndexInBlock: 1,
            bitcoinHeaders: abi.encodePacked(
                hex"04000000e0879a33a87bf9481385adae91fa9e93713b932cbe8a09030000000000000000ee5ded948d805bb71bee5de25b447c42527898cac93eee1afe04663bb8204b358627fe56f4960618304a7db1",
                hex"04000000c0de92e7326cb020b59ffc5998405e539863c57da088a7040000000000000000d8e7273d0198ba4f10dfd57d151327c32113fc244fd0587d161a5c5332a53651ed28fe56f4960618b24502cc"
                )
        });
    }

    function dummyBitcoinAddress() public pure returns (BitcoinAddress memory) {
        return BitcoinAddress({scriptPubKey: hex"76a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac"});
    }

    function testSellBtc() public {
        token1.sudoMint(bob, 100);

        vm.startPrank(alice);
        this.placeBtcSellOrder(1000, address(token1), 100);

        vm.startPrank(bob);
        token1.approve(address(this), 40);
        this.acceptBtcSellOrder(0, dummyBitcoinAddress(), 40);

        vm.startPrank(alice);
        this.proofBtcSellOrder(1, dummyTransaction(), dummyProof());
    }

    function testWithdrawSellBtcOrder() public {
        token1.sudoMint(bob, 100);

        vm.startPrank(alice);
        this.placeBtcSellOrder(1000, address(token1), 100);

        vm.startPrank(bob);
        token1.approve(address(this), 40);
        this.acceptBtcSellOrder(0, dummyBitcoinAddress(), 40);

        vm.startPrank(alice);
        this.withdrawBtcSellOrder(0);
        assertEq(token1.balanceOf(alice), 0);
        assertEq(token1.balanceOf(address(this)), 4);
    }

    function testCancelSellBtcOrder() public {
        token1.sudoMint(bob, 100);

        vm.startPrank(alice);
        this.placeBtcSellOrder(1000, address(token1), 100);

        vm.startPrank(bob);
        token1.approve(address(this), 40);
        this.acceptBtcSellOrder(0, dummyBitcoinAddress(), 40);

        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);
        assertEq(token1.balanceOf(address(this)), 4);
        this.cancelAcceptedBtcSellOrder(1);
        assertEq(token1.balanceOf(bob), 100);
        assertEq(token1.balanceOf(alice), 0);
        assertEq(token1.balanceOf(address(this)), 0);
    }

    function testBuyBtc() public {
        token1.sudoMint(alice, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 100);
        this.placeBtcBuyOrder(1000, dummyBitcoinAddress(), address(token1), 100);

        vm.startPrank(bob);
        this.acceptBtcBuyOrder(0, 40);

        this.proofBtcBuyOrder(1, dummyTransaction(), dummyProof());
        assertEq(token1.balanceOf(alice), 0);
        assertEq(token1.balanceOf(address(this)), 96);
        assertEq(token1.balanceOf(bob), 4);
    }

    function testWithdrawBuyBtc() public {
        token1.sudoMint(alice, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 100);
        this.placeBtcBuyOrder(1000, dummyBitcoinAddress(), address(token1), 100);

        vm.startPrank(bob);
        this.acceptBtcBuyOrder(0, 40);

        assertEq(token1.balanceOf(address(this)), 100);

        vm.startPrank(alice);
        this.withdrawBtcBuyOrder(0);
        assertEq(token1.balanceOf(address(this)), 4); // 4 remains locked for the accepted part
        assertEq(token1.balanceOf(alice), 96); // 96 unlocked
    }

    function testcancelAcceptedBtcBuyOrder() public {
        token1.sudoMint(alice, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 100);
        this.placeBtcBuyOrder(1000, dummyBitcoinAddress(), address(token1), 100);

        vm.startPrank(bob);
        this.acceptBtcBuyOrder(0, 40);

        assertEq(token1.balanceOf(address(this)), 100);

        vm.startPrank(alice);
        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);
        this.cancelAcceptedBtcBuyOrder(1);
        assertEq(token1.balanceOf(alice), 4); // 4 returned to alice
        assertEq(token1.balanceOf(address(this)), 96); // remainder remains available
    }
}
