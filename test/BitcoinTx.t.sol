// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import {Test, console2} from "forge-std/Test.sol";

import {BitcoinTx} from "../src/bridge/BitcoinTx.sol";

contract BitcoinTxTest is Test {
    using BTCUtils for bytes;
    using ValidateSPV for bytes32;

    function test_GetTxHash() public {
        // b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735
        bytes32 txId = hex"3557632d64a45dd8ae2abc4e6d5d1a977e98db24c6a0ae186c265ed972011bb6";

        BitcoinTx.Info memory txInfo = BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"01ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a30000000000fdffffff",
            outputVector: hex"01102700000000000022512037679ea62eab55ebfd442c53c4ad46b6b75e45d8a8fa9cb31a87d0df268b029a",
            locktime: hex"00000000"
        });

        bytes32 txHash =
            abi.encodePacked(txInfo.version, txInfo.inputVector, txInfo.outputVector, txInfo.locktime).hash256View();

        console2.logBytes32(txHash);
        assertEq(txId, txHash);
    }

    function test_ProveTx() public view {
        bytes32 txId = hex"3557632d64a45dd8ae2abc4e6d5d1a977e98db24c6a0ae186c265ed972011bb6";

        bytes32 merkleRoot = hex"01c6691023f17fd78f2dffc85d9db21b84eb6e77352f494f9437168820dbfb90";
        bytes memory merkleProof =
            hex"ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a371db6dfce8daed1d809275e0862441b3cdfd314eceea5a79ee7aeec69cc70f614082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107aca93b96e7dea43442a944a6ab4f8bed0d25d3d372a836a6042375bc57fee5c5425f67a3920a489b23f9133fc84d7987d990acc7c2569a81b547a5f65385856d90100e54ec14dd40c23c3cf1e61a2a16a03aea0e85d236942ad538262528d6748d20dc6ca7c40d75ba7b782bc3d1302633c6def1531573c6420b99840ecffc0125f8e0f12ec4aa1d74fd5ec8d9a57c154267cb6ff0276835592cb8500d8c3c5650e84b83e73e9094de0c2bdaa4d661a3b1adacfae0f3c0f8007ab1b2be8dbf32f073068979a263152d6c234ad0f4b70f697168502d62ead0c0194bcf77321a85a1e127afc4477dcc3c3636a7818601d9ff43f837b15ef74d387c688fc0a45b79aec0b6";
        uint256 txIndexInBlock = 407;
        require(
            txId.prove(merkleRoot, merkleProof, txIndexInBlock),
            "Tx merkle proof is not valid for provided header and tx hash"
        );
    }

    function test_ReverseEndianness() public {
        bytes32 b = hex"db8e78d32300052aa487e3bfb9d9713d23023ecd08b5398e931b827dd151f276";
        (bytes32 txHash) = BitcoinTx.reverseEndianness(b);
        assertEq(txHash, hex"76f251d17d821b938e39b508cd3e02233d71d9b9bfe387a42a050023d3788edb");
    }

    function test_EnsureTxInputSpendsUtxo() public pure {
        bytes memory inputVector =
            hex"063941cf4ed4dad655dcbbf363347f2ddd3eb8851991c9f4f635cfe2a26ef2498f0d000000fc00473044022035a1616b0c034a9a17aaa409a60049b4da34148ebc84b97750b20b28a67751230220580a8bac27e7e31675adcbcc27937389d1f35c976d41f9dde0e62de4c94e38260147304402200cb5d3dbc523da3a99ca4da0fc8ba35d3266939a6c6eb6e6ba70979fc9c1e93302201e91fcc95928da5e01ccc5127000a792d407987503ef59e55d07b7bdb720eefb014c69522103cbc6a30564adc716a52fc28a9ead7b06611765f6f9ecd90d19567033ed9b01b421034c939adc400e67354b4df6afcbedea2a3dc5a4c4805631426363f2fcffd709bd210270fec2b3df4961de8a5e10febb3319922b288efa1300cf048752f3a413a44e1553aefffffffff81c0fe333bc864e0c9d1b32a1dcc14352694dabb015fb6c35f9fc2f0e32ca9d00000000fdfd00004830450221009cf1afe74a98a37798b7324fd2871410b2e269c14aeb05968ca8ce1f23ebe1b4022024164457fcbac57c978dba1bc3ae53378f26aa5380d1bec975da4f85ff9c16ea01473044022042f5950266ad1be284c7ec10d7ca4e7b6cdfbec0c009c5f73f2bdbd24e85eb2902203ac84a337b6e4448a6f7e5d6720e513deefa52a3bb11ce58255756fc9ab48dfe014c695221037fe3f1cffadb5a78862ec191411a824dbc6fbc162db232cc54c845d08527a8922102e983bee0d7339993c6c64ec038a3a576fe3f71b90471a35b22f566fbdefe00e92103bb54d778d8a51d87ddbadb81930c24c316be852cc8b8b3ada9ff50c70e19c9b553aeffffffffab937ce4cf7db7259523eb70e3a2534fae8c899bf101c96eca826caa8dbb44650000000000ffffffff405eaef05128f2d4b1af6fbf7f8584b6d9937b67f18e0f73c4bc1cdf2d6280ec00000000fdfd0000483045022100aba3ac85c6f81fb692cb1227b0a525514e8d6eb46f89a4f407e547dc70b89a5b02207cec256fedfa4407a96eefdb413f79618b3d199677e2b1031a5d66008e7346b701473044022072dc0e1f6208fde1d9a94d14ac945eec6b018af9857ba2f8c5e3df9ae694070e02207328cbeae5be5680152d6b7ab1c706dd3f3098977a94cc12458a44be7b804b58014c69522103a780028aebbf4948a667fc13c7d65926b584aa89d6dcf7f5ea2781f536c99b612103f336f404719f79111f4b06589de871fc9cb7da49a03d02da36dbeb046554c4752102200acc243b2f9d8b84369a28732236c2df0709d59b1199a0a451d332b4fc93c453aeffffffff977ca6132acad11a89ceae92ea15a540ae5366f19f871a61fb6ae6b4daa5b52f00000000fdfd00004730440220352b828c387a78968cb945cca1c7001397dbb5aa723ca817159e5237ec1d69e902201059d50234972874a2aff4473baf53657e8779fb360c83770ab6305b55836720014830450221008a71067a4a4e8acca22ac59bf91d14c4d25e8693a50fbf79ff7cc392e75d2bd5022079b7a6e3f663cb443cf3e2c512176ab36b96b5ee0c03ffbe05f4404f7ed2e338014c69522102fca18dc12a5f3d1dae0a3e76d77f5f79b89d76cbb24e861c7d982be60dd4bf5d210274627fee4ab8e5953b183ddc7edbd45488e4d6faf9df9c640e05035c4af9ec5c210291ed80108c6fb853de2e4f993c7b63a8a5935ee03ac90fed8cdd1b8725c5a7b253aefffffffff00c97eb214c453a7f51b55182d448cd410dc937dbfd967135548a8a2a1f7ade00000000fdfd000047304402200645636f91c792d54346a589d021fbfb7af8803f3d8feaff2f4cd1f2e703f6dc02205eb48fa9428b6e7d1d03bc1788025ff1da58a686903e46386929ce8f810064af01483045022100b32407c41ce04d92976ebfb55018a3cf6b9bf00315c21b29c237f18e0e4f4eec022063de776c5cbdf4d3322a5229bfbea21ec12bd5470c5423cd612c4ca2f823efc4014c69522103f570642ab999a8ccaccf6d275aabb24db32907e1a37b62ab6271865c5a8194ac2103cf7901f7a585cf32aa1d9024807639540062afba94fd466f7b50417e22376d6b2102c5fa500865c57f92204a28326d126fa8eba13a0b43f860ba1eb0d4b44ce5c47c53aeffffffff";
        BitcoinTx.UTXO memory utxo2;
        utxo2.txHash = hex"de7a1f2a8a8a54357196fddb37c90d41cd48d48251b5517f3a454c21eb970cf0";
        utxo2.txOutputIndex = 0;
        utxo2.txOutputValue = 0;

        BitcoinTx.ensureTxInputSpendsUtxo(inputVector, utxo2);
    }

    function test_EnsureTxInputSpendsUtxoInvalidEncodingOfInputVector() public {
        bytes memory inputVector = hex"de7a1f2a8a8a54357196fddb37c90d41cd48d48251b5517f3a454c21eb970cf0";
        BitcoinTx.UTXO memory utxo;
        utxo.txHash = hex"de7a1f2a8a8a54357196fddb37c90d41cd48d48251b5517f3a454c21eb970cf0";
        utxo.txOutputIndex = 0;
        utxo.txOutputValue = 0;

        vm.expectRevert("Bad VarInt in scriptSig");
        BitcoinTx.ensureTxInputSpendsUtxo(inputVector, utxo);
    }

    function test_EnsureTxInputSpendsUtxoWhenTxIsNotSpend() public {
        bytes memory inputVector =
            hex"063941cf4ed4dad655dcbbf363347f2ddd3eb8851991c9f4f635cfe2a26ef2498f0d000000fc00473044022035a1616b0c034a9a17aaa409a60049b4da34148ebc84b97750b20b28a67751230220580a8bac27e7e31675adcbcc27937389d1f35c976d41f9dde0e62de4c94e38260147304402200cb5d3dbc523da3a99ca4da0fc8ba35d3266939a6c6eb6e6ba70979fc9c1e93302201e91fcc95928da5e01ccc5127000a792d407987503ef59e55d07b7bdb720eefb014c69522103cbc6a30564adc716a52fc28a9ead7b06611765f6f9ecd90d19567033ed9b01b421034c939adc400e67354b4df6afcbedea2a3dc5a4c4805631426363f2fcffd709bd210270fec2b3df4961de8a5e10febb3319922b288efa1300cf048752f3a413a44e1553aefffffffff81c0fe333bc864e0c9d1b32a1dcc14352694dabb015fb6c35f9fc2f0e32ca9d00000000fdfd00004830450221009cf1afe74a98a37798b7324fd2871410b2e269c14aeb05968ca8ce1f23ebe1b4022024164457fcbac57c978dba1bc3ae53378f26aa5380d1bec975da4f85ff9c16ea01473044022042f5950266ad1be284c7ec10d7ca4e7b6cdfbec0c009c5f73f2bdbd24e85eb2902203ac84a337b6e4448a6f7e5d6720e513deefa52a3bb11ce58255756fc9ab48dfe014c695221037fe3f1cffadb5a78862ec191411a824dbc6fbc162db232cc54c845d08527a8922102e983bee0d7339993c6c64ec038a3a576fe3f71b90471a35b22f566fbdefe00e92103bb54d778d8a51d87ddbadb81930c24c316be852cc8b8b3ada9ff50c70e19c9b553aeffffffffab937ce4cf7db7259523eb70e3a2534fae8c899bf101c96eca826caa8dbb44650000000000ffffffff405eaef05128f2d4b1af6fbf7f8584b6d9937b67f18e0f73c4bc1cdf2d6280ec00000000fdfd0000483045022100aba3ac85c6f81fb692cb1227b0a525514e8d6eb46f89a4f407e547dc70b89a5b02207cec256fedfa4407a96eefdb413f79618b3d199677e2b1031a5d66008e7346b701473044022072dc0e1f6208fde1d9a94d14ac945eec6b018af9857ba2f8c5e3df9ae694070e02207328cbeae5be5680152d6b7ab1c706dd3f3098977a94cc12458a44be7b804b58014c69522103a780028aebbf4948a667fc13c7d65926b584aa89d6dcf7f5ea2781f536c99b612103f336f404719f79111f4b06589de871fc9cb7da49a03d02da36dbeb046554c4752102200acc243b2f9d8b84369a28732236c2df0709d59b1199a0a451d332b4fc93c453aeffffffff977ca6132acad11a89ceae92ea15a540ae5366f19f871a61fb6ae6b4daa5b52f00000000fdfd00004730440220352b828c387a78968cb945cca1c7001397dbb5aa723ca817159e5237ec1d69e902201059d50234972874a2aff4473baf53657e8779fb360c83770ab6305b55836720014830450221008a71067a4a4e8acca22ac59bf91d14c4d25e8693a50fbf79ff7cc392e75d2bd5022079b7a6e3f663cb443cf3e2c512176ab36b96b5ee0c03ffbe05f4404f7ed2e338014c69522102fca18dc12a5f3d1dae0a3e76d77f5f79b89d76cbb24e861c7d982be60dd4bf5d210274627fee4ab8e5953b183ddc7edbd45488e4d6faf9df9c640e05035c4af9ec5c210291ed80108c6fb853de2e4f993c7b63a8a5935ee03ac90fed8cdd1b8725c5a7b253aefffffffff00c97eb214c453a7f51b55182d448cd410dc937dbfd967135548a8a2a1f7ade00000000fdfd000047304402200645636f91c792d54346a589d021fbfb7af8803f3d8feaff2f4cd1f2e703f6dc02205eb48fa9428b6e7d1d03bc1788025ff1da58a686903e46386929ce8f810064af01483045022100b32407c41ce04d92976ebfb55018a3cf6b9bf00315c21b29c237f18e0e4f4eec022063de776c5cbdf4d3322a5229bfbea21ec12bd5470c5423cd612c4ca2f823efc4014c69522103f570642ab999a8ccaccf6d275aabb24db32907e1a37b62ab6271865c5a8194ac2103cf7901f7a585cf32aa1d9024807639540062afba94fd466f7b50417e22376d6b2102c5fa500865c57f92204a28326d126fa8eba13a0b43f860ba1eb0d4b44ce5c47c53aeffffffff";
        BitcoinTx.UTXO memory utxo;
        vm.expectRevert("Transaction does not spend the required utxo");
        BitcoinTx.ensureTxInputSpendsUtxo(inputVector, utxo);
    }

    function test_EnsureTxInputSpendsUtxoReadOverrun() public {
        bytes memory inputVector = hex"ff";
        BitcoinTx.UTXO memory utxo;
        vm.expectRevert("Read overrun during VarInt parsing");
        BitcoinTx.ensureTxInputSpendsUtxo(inputVector, utxo);
    }

    function test_EnsureTxInputSpendsUtxoEmptyBytes() public {
        bytes memory inputVector = new bytes(0);
        BitcoinTx.UTXO memory utxo;
        (bool success,) = address(this).call(
            abi.encodeWithSignature("ensureTxInputSpendsUtxo(bytes memory,BitcoinTx.UTXO memory)", inputVector, utxo)
        );
        // Panic: [FAIL. Reason: panic: array out-of-bounds access (0x32)]
        assertFalse(success);
    }

    function test_GetTxOutputValue() public {
        // b1273a6c00eba20ee8837e445599d1362e005f6e1a8525802ba57bc515461a3a
        uint64 value = BitcoinTx.getTxOutputValue(
            keccak256(hex"16001493adab0a7a8cb7675db135c9c97e81942025c2c9"),
            hex"02c67d16000000000016001493adab0a7a8cb7675db135c9c97e81942025c2c9aea79b4200000000160014f60834ef165253c571b11ce9fa74e46692fc5ec1"
        );
        assertEq(value, 1473990);
    }

    function test_GetTxOutputValueSingleOutputTransaction() public {
        // tx api: https://btc-testnet.gobob.xyz/tx/b0fe4bd36b17be89f131c2e652578def3cc0c3d5aa9e7a3f972365a8dc46dba8
        uint64 value = BitcoinTx.getTxOutputValue(
            keccak256(hex"160014d127b24a7e2aad2ddf21d4d940f6202158aa507d"),
            hex"01ab9ab90800000000160014d127b24a7e2aad2ddf21d4d940f6202158aa507d"
        );
        assertEq(value, 146381483);
    }

    function test_GetTxOutputValueMaxTransactionValue() public {
        // tx api: https://api.blockcypher.com/v1/btc/main/txs/d486aeb0e59181fd1addb4aa69ce04d638188fc1125c424899267e8ed6a8af24?limit=50&includeHex=true/
        uint64 value = BitcoinTx.getTxOutputValue(
            keccak256(hex"17a914ed498d84acb4532656fcf6947d0ceab6c77188bc87"),
            hex"0260536280ed03000017a9148e097444bb754122652208bf00f71a87b177b700874b5a115e2704000017a914ed498d84acb4532656fcf6947d0ceab6c77188bc87"
        );
        assertEq(value, 4567128431179);
    }

    function test_GetTxOutputValueWithInvalidPubkey() public {
        vm.expectRevert("No output found for scriptPubKey");
        BitcoinTx.getTxOutputValue(
            keccak256(hex"000000000000000000000000000000000000000000000000"),
            hex"02c67d16000000000016001493adab0a7a8cb7675db135c9c97e81942025c2c9aea79b4200000000160014f60834ef165253c571b11ce9fa74e46692fc5ec1"
        );
    }

    function test_GetTxOutputValueEmptyBytes() public {
        bytes memory emptyBytes = new bytes(0);
        (bool success,) = address(this).call(
            abi.encodeWithSignature(
                "getTxOutputValue(bytes32,bytes memory)",
                keccak256(hex"16001493adab0a7a8cb7675db135c9c97e81942025c2c9"),
                emptyBytes
            )
        );
        // Reason: panic: array out-of-bounds access (0x32)
        assertFalse(success);
    }

    function test_GetTxOutputValueWithInvalidInput() public {
        (bool success,) = address(this).call(
            abi.encodeWithSignature(
                "getTxOutputValue(bytes32,bytes memory)",
                keccak256(hex"16001493adab0a7a8cb7675db135c9c97e81942025c2c9"),
                hex"16001493adab0a7a8cb7675db135c9c97e81942025c2c9"
            )
        );
        // [FAIL. Reason: EvmError: OutOfGas] (gas: 9223372036854754743)
        assertFalse(success);
    }
}
