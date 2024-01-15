// SPDX-License-Identifier: Unlicense
pragma solidity >=0.8.0;

import {DSTest} from "ds-test/test.sol";
import {Vm} from "forge-std/Vm.sol";
import "../../src/bridge/BitcoinTx.sol";

//common utilities for forge tests
contract Utilities is DSTest {
    Vm internal immutable vm = Vm(HEVM_ADDRESS);
    bytes32 internal nextUser = keccak256(abi.encodePacked("user address"));

    function getNextUserAddress() external returns (address payable) {
        //bytes32 to address conversion
        address payable user = payable(address(uint160(uint256(nextUser))));
        nextUser = keccak256(abi.encodePacked(nextUser));
        return user;
    }

    //create users with 100 ether balance
    function createUsers(uint256 userNum) external returns (address payable[] memory) {
        address payable[] memory users = new address payable[](userNum);
        for (uint256 i = 0; i < userNum; i++) {
            address payable user = this.getNextUserAddress();
            vm.deal(user, 100 ether);
            users[i] = user;
        }
        return users;
    }

    //move block.number forward by a given number of blocks
    function mineBlocks(uint256 numBlocks) external {
        uint256 targetBlock = block.number + numBlocks;
        vm.roll(targetBlock);
    }

    function dummyTransaction() public view returns (BitcoinTx.Info memory) {
        return BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"01996cf4e2f0016a1f092aaaba653c7eae5dd4b6eef1f9a2a94c64f34b2fecbd85010000006a47304402206f99da49ce586528ed8981842df30b4a5a91195fd2d83e440d4193fc16a944ec022055cfdf63a2c90638821f1b5ff1fdf77526163ae057a0d0de30a6e1d3009e7a29012102811832eef7216470f489991f1d87e36d2890755d2bbf827eb1e71804491506afffffffff",
            outputVector: hex"0200e9a435000000001976a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac804a5d05000000001976a9145c1addbd0e4e78479e71fdca0555d2d44b67378e88ac",
            locktime: hex"00000000"
        });
    }

    function dummyProof() public view returns (BitcoinTx.Proof memory) {
        return BitcoinTx.Proof({
            merkleProof: hex"0465f99dbe384bbc5d86a5242712e4154958e4b01f595f14b76f873ec349e14a16b17770af2bb48c9b2ce4dddf4631866fe3753e6c54bdcf18dfb2d4fb9983ee58e4f3be92087c843b815bbe1d5d686dc972552f7ffda4342319ceb5bea67ab0f2e463ec8ce8e3f580c5e2470ef20c5b33398ab9fea5ccbd0b3e3f6211305edafa068a28c8ac634df5bbc8064357295373b97db2600745f23ad6ebc87b66b4a8685aa8ff8e69abc5029dbf4b2fa03f05680c7a2c491410b23a5a6b27c5a91b89dac8cdd16a4460ce8ac8d17491025d29336440a133867f938a7f41cc7a64f3f04ac3817c3eb6a6a11dc30850ca4e80f9abbd42268bcc626138bc01639a902713425e7d3aca45647001fb32ff396c07027c5b081325530e74f936e6c4a8078a05f9717efd315534a84d047ee2ff0b2b93159a2b98eabb578af67ef7540a58e488b9c587a994c1a9a86937ad343ea734b7427678e3e6ba0be8f5045ce47e541bbc",
            txIndexInBlock: 1,
            bitcoinHeaders: abi.encodePacked(
                hex"04000000e0879a33a87bf9481385adae91fa9e93713b932cbe8a09030000000000000000ee5ded948d805bb71bee5de25b447c42527898cac93eee1afe04663bb8204b358627fe56f4960618304a7db1",
                hex"04000000c0de92e7326cb020b59ffc5998405e539863c57da088a7040000000000000000d8e7273d0198ba4f10dfd57d151327c32113fc244fd0587d161a5c5332a53651ed28fe56f4960618b24502cc"
                )
        });
    }

    struct OrdinalInformation {
        BitcoinTx.Info info;
        BitcoinTx.Proof proof;
        BitcoinTx.UTXO utxo;
    }

    OrdinalInformation public ordInfo;

    function dummyOrdinalInfo()
        public
        view
        returns (BitcoinTx.Info memory, BitcoinTx.Proof memory, BitcoinTx.UTXO memory)
    {
        BitcoinTx.Info memory info = BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"0176f251d17d821b938e39b508cd3e02233d71d9b9bfe387a42a050023d3788edb0100000000ffffffff",
            outputVector: hex"02a08601000000000022002086a303cdd2e2eab1d1679f1a813835dc5a1b65321077cdccaf08f98cbf04ca96ba2c0e0000000000160014e257eccafbc07c381642ce6e7e55120fb077fbed",
            locktime: hex"00000000"
        });

        BitcoinTx.Proof memory proof = BitcoinTx.Proof({
            merkleProof: hex"c2780870a9d6f7936aaf15bb0072fa8de81036562ee557ecf8e23cd59fd80e8730515f6e07efb958e2c84c33e770bf668e3dc1470437b528814177ac38caee720df1e32074eb7735a5b5b117e575d4f8b9630156b63f7fc4dd205e5ce01741b7",
            txIndexInBlock: 4,
            bitcoinHeaders: abi.encodePacked(
                hex"00a00020a672b6254445e7b2dd6e5433f52ea9596e6ce51776fa6ea66d0200000000000013d7683b2bfc7d7cde91c6792f62e6b9453ca2f1e72cdbf106ecabf767dd2ac5bcf98f628886021ac954f84d"
                )
        });

        BitcoinTx.UTXO memory utxo;
        utxo.txHash = hex"db8e78d32300052aa487e3bfb9d9713d23023ecd08b5398e931b827dd151f276";
        utxo.txOutputIndex = 1;
        utxo.txOutputValue = 0;

        return (info, proof, utxo);
    }
}
