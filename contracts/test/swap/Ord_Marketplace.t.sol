// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {BtcMarketPlace} from "../../src/swap/Btc_Marketplace.sol";
import {Utilities} from "./Utilities.sol";
import {BitcoinTx} from "../../src/utils/BitcoinTx.sol";
import {TestLightRelay} from "../../src/relay/TestLightRelay.sol";
import "../../src/swap/Ord_Marketplace.sol";

contract ArbitaryErc20 is ERC20, Ownable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract OrdMarketPlaceTest is OrdMarketplace, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    ArbitaryErc20 token1;

    struct Ordinal {
        BitcoinTx.Info info;
        BitcoinTx.Proof proof;
        BitcoinAddress requester;
        BitcoinTx.UTXO utxo;
        OrdinalId id;
    }

    Ordinal[3] ordinalsInfo;

    constructor() OrdMarketplace(testLightRelay) {
        // data from testnet
        // https://btc-testnet.gobob.xyz/tx/c872fb11bbca1241aced71c692e7d0b0cf46aadb390ce66ddfcf5fbd8e5bc26f
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

        // https://btc-testnet.gobob.xyz/tx/c872fb11bbca1241aced71c692e7d0b0cf46aadb390ce66ddfcf5fbd8e5bc26f
        BitcoinAddress memory requester =
            BitcoinAddress({scriptPubKey: hex"0014e257eccafbc07c381642ce6e7e55120fb077fbed"});
        OrdinalId memory id;

        ordinalsInfo[0] = Ordinal({info: info, proof: proof, requester: requester, utxo: utxo, id: id});

        // data from mainnet
        // get data from sdk txid: 67b7dda5dbc6b6dff1a5487e0eb01db8a22f5cf9ed1c2427cdb1f4986e14e79a
        BitcoinTx.Info memory info2 = BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"063941cf4ed4dad655dcbbf363347f2ddd3eb8851991c9f4f635cfe2a26ef2498f0d000000fc00473044022035a1616b0c034a9a17aaa409a60049b4da34148ebc84b97750b20b28a67751230220580a8bac27e7e31675adcbcc27937389d1f35c976d41f9dde0e62de4c94e38260147304402200cb5d3dbc523da3a99ca4da0fc8ba35d3266939a6c6eb6e6ba70979fc9c1e93302201e91fcc95928da5e01ccc5127000a792d407987503ef59e55d07b7bdb720eefb014c69522103cbc6a30564adc716a52fc28a9ead7b06611765f6f9ecd90d19567033ed9b01b421034c939adc400e67354b4df6afcbedea2a3dc5a4c4805631426363f2fcffd709bd210270fec2b3df4961de8a5e10febb3319922b288efa1300cf048752f3a413a44e1553aefffffffff81c0fe333bc864e0c9d1b32a1dcc14352694dabb015fb6c35f9fc2f0e32ca9d00000000fdfd00004830450221009cf1afe74a98a37798b7324fd2871410b2e269c14aeb05968ca8ce1f23ebe1b4022024164457fcbac57c978dba1bc3ae53378f26aa5380d1bec975da4f85ff9c16ea01473044022042f5950266ad1be284c7ec10d7ca4e7b6cdfbec0c009c5f73f2bdbd24e85eb2902203ac84a337b6e4448a6f7e5d6720e513deefa52a3bb11ce58255756fc9ab48dfe014c695221037fe3f1cffadb5a78862ec191411a824dbc6fbc162db232cc54c845d08527a8922102e983bee0d7339993c6c64ec038a3a576fe3f71b90471a35b22f566fbdefe00e92103bb54d778d8a51d87ddbadb81930c24c316be852cc8b8b3ada9ff50c70e19c9b553aeffffffffab937ce4cf7db7259523eb70e3a2534fae8c899bf101c96eca826caa8dbb44650000000000ffffffff405eaef05128f2d4b1af6fbf7f8584b6d9937b67f18e0f73c4bc1cdf2d6280ec00000000fdfd0000483045022100aba3ac85c6f81fb692cb1227b0a525514e8d6eb46f89a4f407e547dc70b89a5b02207cec256fedfa4407a96eefdb413f79618b3d199677e2b1031a5d66008e7346b701473044022072dc0e1f6208fde1d9a94d14ac945eec6b018af9857ba2f8c5e3df9ae694070e02207328cbeae5be5680152d6b7ab1c706dd3f3098977a94cc12458a44be7b804b58014c69522103a780028aebbf4948a667fc13c7d65926b584aa89d6dcf7f5ea2781f536c99b612103f336f404719f79111f4b06589de871fc9cb7da49a03d02da36dbeb046554c4752102200acc243b2f9d8b84369a28732236c2df0709d59b1199a0a451d332b4fc93c453aeffffffff977ca6132acad11a89ceae92ea15a540ae5366f19f871a61fb6ae6b4daa5b52f00000000fdfd00004730440220352b828c387a78968cb945cca1c7001397dbb5aa723ca817159e5237ec1d69e902201059d50234972874a2aff4473baf53657e8779fb360c83770ab6305b55836720014830450221008a71067a4a4e8acca22ac59bf91d14c4d25e8693a50fbf79ff7cc392e75d2bd5022079b7a6e3f663cb443cf3e2c512176ab36b96b5ee0c03ffbe05f4404f7ed2e338014c69522102fca18dc12a5f3d1dae0a3e76d77f5f79b89d76cbb24e861c7d982be60dd4bf5d210274627fee4ab8e5953b183ddc7edbd45488e4d6faf9df9c640e05035c4af9ec5c210291ed80108c6fb853de2e4f993c7b63a8a5935ee03ac90fed8cdd1b8725c5a7b253aefffffffff00c97eb214c453a7f51b55182d448cd410dc937dbfd967135548a8a2a1f7ade00000000fdfd000047304402200645636f91c792d54346a589d021fbfb7af8803f3d8feaff2f4cd1f2e703f6dc02205eb48fa9428b6e7d1d03bc1788025ff1da58a686903e46386929ce8f810064af01483045022100b32407c41ce04d92976ebfb55018a3cf6b9bf00315c21b29c237f18e0e4f4eec022063de776c5cbdf4d3322a5229bfbea21ec12bd5470c5423cd612c4ca2f823efc4014c69522103f570642ab999a8ccaccf6d275aabb24db32907e1a37b62ab6271865c5a8194ac2103cf7901f7a585cf32aa1d9024807639540062afba94fd466f7b50417e22376d6b2102c5fa500865c57f92204a28326d126fa8eba13a0b43f860ba1eb0d4b44ce5c47c53aeffffffff",
            outputVector: hex"4a6a170300000000002251207b973e017d99a513455d212caba6948c931476b70e54c58cc74448284a29bb31d06006000000000022512006119466965cd6b2a508ff44f2d8f0ec0939d13db065569f4852a54862fe0074c0cf6a000000000022512052204592e5116c27410ee0bbe1020464bdf2efe2f665b2b8c7710feccc3bc6555363220000000000225120ff4d1a953be8418ab8261bdc00a135783e7fb19d988b8e1b8239e419c80dbe2acef125000000000022512056d50486caf8eec321edf8a6c32f554cd864e90a791ea2b38919092982defc54296c0b0000000000160014ab15fbee29d9b23ac151d6ca9c0b11f234bcc0eb7d450200000000001976a914281106328aab1c5869982e05a2b6f6682982d21c88aca5d7b00000000000160014b50d63a757ca40afaf4fc2cca237e70eeaf6764209ff09000000000022512063ec8ef6c645016843d3a42618efaed45d4c9e17dc0a5cd4125951bb3117439c8c14060000000000225120f8ef2f6da914c706f60dab5c8d2439ba6c9b4ae8b171144dadc6a535910ad6eeb0c83b00000000002251209f9d8635c3737d9001a7aadf950dbb2337fe031dc368d15a059a20ad355bfac0bb6f3400000000002251201cf34ca4332a54937d3ca1604ca9b9af510e071171220fe29e96ff12f8686d83305fd400000000002251204564aee4d3aaa8d50d160bb6aa25a4a22baacf4153de378e56ae9aad88379f4415cbb0000000000017a914a98a04f77cb54c448aedcee5441a5b832b8ddb9587dd500200000000001600149db3a5ebc853369d359786376d31ae506d23c9d92e0a0500000000001600140a662f348af0985cc6dafa3aaeb70dda55002921c39e130000000000160014f06b21f8b5ea8ea86fda8a386dd91b8570fe8ff0f0010e0000000000225120a86ea4c1ad481a7aa672e91f3908c2a652a3c141cf2d831a79d99da6481a2bc000d606000000000017a914f4271a76675cdace36766295a78d4a55bdfdff8d87f00a4b000000000017a914acc75749ef4cacbbc35fb3a845f75844b29bc35f874941170000000000225120db9c5cc908474e24341816cf49806418c7cae6818b5f89c3d983745ef0ce9bf966412d000000000022512048e3c67b37187558a7779b99a3d5a054848f00ac57e9260e0fecf544f6d597ad8d57fd0500000000225120699e736b83bddd76c3395a8924371894dc739eb3fec5772b858f1675e8c33a12b97e15000000000022512061162f8814587e431f7999c10416221ff325ad2c218641b8044753dff10bd4c5041499000000000017a914e9ca361c83c46ada9a93c9e6a1b85c9ca66a662f8770862c0000000000225120d45f4cc697b09edb740965749aac06ad9b6e2d3a9d79886e6dfe6d7d57dcec06dc5e060000000000225120291493dc11642f5e84b220f0d80d07a6bf5c658efbaa75060dcccd3cc30d1c59e64b6b00000000002251209ad90b4b044e3a8fcedac0270df87494b742e437b2d39edca74ae62d7d13b7fc5b5e260000000000225120ee9be94d7c01a36a9b01bd4f1a1b7a5bd1d2816424ed68dba417ec72e7fb0f6250e52400000000001600140a7ea421458a2a0577d42990cf03a68c212e49c7042118000000000022512006fb1c6f45779f899cb5e88b4e032ff53f5c4942cbb01fb554a9cb5491847000a086010000000000225120adc382321d2e370db287205aad71d2b97a4533d5e13810e40bf976ad88f0d49d361d1400000000002251208a0ded07c10c1e9a245f2da655aba399f7bb224058907370282f7f7e024cce2fe0d14d000000000022512005921968d9c823ba9298f08a8d42d52d804bdff9375836f1aa01322f7a56902ff0010e0000000000225120a1461712396c803712a7fa2eecdf08dfe99e3f03cc38625f7fe03cb9372cd7f92c5b090000000000225120c7355433fa5e420b84d9c26386eb211e573f69955c93e418983523c422894736fdf23700000000002251209c8a5a263f3a7250390ce8918e14296d2ff53dfaeec51b40521e0a10036d205d40420f00000000001976a9141d08a974de30edcfcad5091b73a9dc944e85463e88ac6be70c0000000000225120326fafdb5f6e3f7d247dda63d82374f8c6dc6b61dcf8586b2fd911ef50ed71cf3bc808010000000022512087ea4c7415226da08724b284a5a9d07dba8a53751367dcf807ef2f0eb6ff1ffaa28a0300000000002251205c119514c9bba2920991ba3b21e3cd5a106838878319ad03c136c6af80facebe10b93b0000000000225120fd3a11d164d8dbaf2730177b35c84a83048dab306fbc5f0a6e0b2be0ac82f58358f90c00000000002251203164858dbf43d5b04e5fabaf5f25ddfb8382debf0e31af5acc0e1f0ddbda5d87816f0900000000002251208a8c9fe5a32e5be49f65115c0c8c087e1418014f4af60e097cd589677e90e36495cb3700000000001976a9149a601f879efd7e3e4c75c3ead807983ce78bd70288ac30441d000000000016001406f2512a90ead3f819d1a2cec6227e5974b837802010340000000000225120e1ca58c090c55fb4aebc10e40b514d0c041f8e295c645fa142f45c3499678f5ae4a60300000000002251204bb248e711a81edffa5088f170e1e6c4fc53a5d4118ab51b974efb264612c4db442d210000000000160014ac6382f7fb35f24ce8cdc841b1f730fd503c23e674a13f00000000002251205437ddffd90d3273e62155b9ce1bc45d294ee303daacd43c9f88befc274388a1cd9710000000000022512079cbb077e83f361ecb66777218b1e8ca4a81385c3c3b98453bd48c4d929b8d931e472a0000000000225120063d907d79279ee4ffae6a752f55600b0bb5782ece8c36a2bc71cb944d527fa260920100000000001976a9146a82de0217639b46e6a0a58ae379d5b9a227642188acf8682200000000001600143f76c9c3302be4da30e722f400a0784293b97901809698000000000022512083ad1fab826ebf0fdfd86d1cf91cf4824c1bbca2d670f21f32f374acbe6dbb934e8b08000000000022512037447bb2346dfdc0c2dd685cb9a783fa8c8a91d314780bf82b2e24ce5aceec8f96324c0000000000225120535205b265267c973529bf2009f5b98473fd8844a6e8d64ca2653ed399b9df34346e0800000000002251208e0c8a9fd4f3c72e970e0878da72cdf22216cb6cd58c8f2b4606deb27c1409833fc51d000000000022512047db5690f90eac2d8cc0d1addbee64b8b3bb45217896bf39533214de05f5bc3b90530300000000001600147b2c89ebb25b459d55d35989a05b3ef06ab57607049610000000000017a914e3b0f22fbaf6488ae811430d3374c6304bec68d9872c014b0000000000225120357a337f384949b590a1f3ab0cad3b84bae7cddad5fd38ec19384c83e2df20e4619510000000000016001430890e35976da7c0b40e0da55c302c24c3eba6eef0010e0000000000225120e435c30c6be6efc8ee7f2ab0879d3e2cba61308cafb642a5eaf46e9ff44085d670332500000000001600140a7ea421458a2a0577d42990cf03a68c212e49c7f0900b0000000000160014aefed35edcb73eeedb21724fe7c6cdf9a72f8c06d0fb0100000000001976a9146ce6f7ad56e66fcdbe21aeadb6307c1434d0215288ac44840f00000000001600140919eef26ba6449ffa215a405ab939370d5c872e191f04000000000017a914d270542327c4ce4fa1ac0852f6d55dd57b289e4a87bfb869000000000017a914f53a4273af8be7208d43e454aeaf203075491a9e87b0a0f40500000000160014438b8ffc1e62d3b967fa88b2ee1f985c1b0c64a570862c0000000000225120841488b47001df49d54cf5c397eda423a918544db7ac7c18a2dd79a9d9d70c08c2e6080000000000160014f29c0f34a4a3715907da86c1937d10dce5efa6b10bf5140300000000220020e5c7c00d174631d2d1e365d6347b016fb87b6a0c08902d8e443989cb771fa7ec",
            locktime: hex"00000000"
        });

        BitcoinTx.Proof memory proof2 = BitcoinTx.Proof({
            merkleProof: hex"b3ed2cbdb45beafc848cba83cdfec403de217702f93d2ef760ef947f4c25364a889f2744d13099fb75294ba3accea7f2f57b080c5c330c7cc4ef5e474f2f3ae43e30f9d1fa0b701a487100ee77eab74c9c22a6fc54e7e65a58d47606aa2c38cb359d4ab035a25ec029181ac67866d74b8f99ef0dba9325a907f21d14eae9de529523c85f4b49d1b69e351f70acfbc03a4f54108accaa85d52bfca065bbc1c2feb08c9dc1a2f87cdba5ad1017c70b4356ab86040538a60b8166afd59b185fa577a26a81b037de042f501d5a42a6efad1f2bf0d82208ebd3324b3923c9c358d7da5644be7757e3d5318ee584b01c31ae1f2f9c8edab8a87c0ebb6e3d7a4d7731138ef5f9f8d9de6ab54954f9f8eca8ad8b03a3d24b82ada0104944dfaf1752ac43abc2887f5fd3d6e03ce7fd819f6260d972adcbbdc35a41bffbc57c6647f1a054c220414f359dc95a81a3644d22f7b62b4f253b5b97a300f47a40a2a681cee1a3a1db92ab7b6c098bdc88b276695f5b52eff831c9678a6b20576814333943b0db",
            txIndexInBlock: 25,
            bitcoinHeaders: abi.encodePacked(
                hex"00200020b70169ce36e45282d9e32c6b3cf815c4580872c94d7801000000000000000000a9bb880a70ef5e045fa684064c5229fb1c540a799e2bdb71efde06304a26de4dc2c77a65952e0417d2418bf4"
            )
        });

        BitcoinTx.UTXO memory utxo2;
        utxo2.txHash = hex"de7a1f2a8a8a54357196fddb37c90d41cd48d48251b5517f3a454c21eb970cf0";
        utxo2.txOutputIndex = 0;
        utxo2.txOutputValue = 0;

        // https://btc-mainnet.gobob.xyz/tx/67b7dda5dbc6b6dff1a5487e0eb01db8a22f5cf9ed1c2427cdb1f4986e14e79a
        BitcoinAddress memory requester2 =
            BitcoinAddress({scriptPubKey: hex"5120d45f4cc697b09edb740965749aac06ad9b6e2d3a9d79886e6dfe6d7d57dcec06"});
        OrdinalId memory id2;

        ordinalsInfo[1] = Ordinal({info: info2, proof: proof2, requester: requester2, utxo: utxo2, id: id2});

        // data from testnet
        // get data from sdk txid: 591235b1a474ea29e29e2b3aaee45055b43e38cdf38c3700df65509f60ee2d8e
        BitcoinTx.Info memory info3 = BitcoinTx.Info({
            version: hex"02000000",
            inputVector: hex"022c73deced32f831ee0c7f9cda848b11ed1f284e6b3251814ca3b6c028d80216e0000000000ffffffffb07f925d2e91c4ff4bf802f38e60c390a8498f650e06ccc8eac32e6913aa3b450100000000ffffffff",
            outputVector: hex"0222020000000000001976a914344a0f48ca150ec2b903817660b9b68b13a6702688ac178e07000000000022512025f4ee2e73ace3ed0e5ec4472541db5755b52cb8823e8ce14b6052dedfe3b33d",
            locktime: hex"00000000"
        });

        BitcoinTx.Proof memory proof3 = BitcoinTx.Proof({
            merkleProof: hex"b1b47c8e4fcf75b9820ef433d7b5a6fa65d559af35173c600b44e736deb5ffd23a1cebfd570521a1153ef16e9fc4eeb8f80d699fdf113e96e7b5c9dfdaf0db8e166f972b6eff1e8d248d1bc3732dfb4e2aa7a78ca0e6d99c01644ce521c02b2f90c4848b9e60e8d44c764c58affefdc9dd40ce2f4f537b923551676a008f8adea365f60d22ae64f00aa2d981e640ee2f4f0781e4d347ea1d154bbf3c829ab2d02e69bdb26bef5f06781b439b6e65ac701cf458018bfb23304817efb66fabf3ba10317933e969f8bd6e6e3aa48f75b0d7c53adbe80dc9dca82d0e66aef5c592d66e6bdbd0630efb413b54c208c9465cc747701261978de3c8f3c6c33923242c67",
            txIndexInBlock: 72,
            bitcoinHeaders: abi.encodePacked(
                hex"0000002019ecad2d640319bad3cbe99dd2819fdba90023ccd6a479d31400000000000000f308f7de96bab7e11e64fdb06000e57806b7791b923cf0e8382547d3c609f2bf65438165ffff001d84cd32400000e0201a1e9727a2da2e112c761a0d3aa639c52e9b43c1e9d80bb0242632240000000056f577d01fbd2550679ced5da24291ec48bbb96a93f790349ac2f54fa075f90c64448165efdf2819529ec2d8"
            )
        });

        // https://mempool.space/testnet/tx/6e21808d026c3bca141825b3e684f2d11eb148a8cdf9c7e01e832fd3cede732c
        // ordinal inscription: https://ordinals-testnet.gamma.io/inscription/6e21808d026c3bca141825b3e684f2d11eb148a8cdf9c7e01e832fd3cede732ci0
        // ordinal tx: https://ordinals-testnet.gamma.io/tx/591235b1a474ea29e29e2b3aaee45055b43e38cdf38c3700df65509f60ee2d8e
        BitcoinTx.UTXO memory utxo3;
        utxo3.txHash = hex"6e21808d026c3bca141825b3e684f2d11eb148a8cdf9c7e01e832fd3cede732c";
        utxo3.txOutputIndex = 0;
        utxo3.txOutputValue = 546;

        // https://btc-testnet.gobob.xyz/tx/591235b1a474ea29e29e2b3aaee45055b43e38cdf38c3700df65509f60ee2d8e
        BitcoinAddress memory requester3 =
            BitcoinAddress({scriptPubKey: hex"76a914344a0f48ca150ec2b903817660b9b68b13a6702688ac"});
        OrdinalId memory id3;

        ordinalsInfo[2] = Ordinal({info: info3, proof: proof3, requester: requester3, utxo: utxo3, id: id3});
    }

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

    function test_OrdinalSellOrderFullFlow() public {
        uint256 nextOrdinalId;

        for (uint256 i = 0; i < ordinalsInfo.length; i++) {
            token1.sudoMint(bob, 100);

            uint256 expectedPlaceId = nextOrdinalId++;

            // placeOrdinalSellOrder by alice
            vm.startPrank(alice);
            vm.expectEmit();
            emit placeOrdinalSellOrderEvent(expectedPlaceId, ordinalsInfo[i].id, address(token1), 100);
            this.placeOrdinalSellOrder(ordinalsInfo[i].id, ordinalsInfo[i].utxo, address(token1), 100);

            uint256 expectedAcceptId = nextOrdinalId++;

            // acceptOrdinalSellOrder by bob
            vm.startPrank(bob);
            token1.approve(address(this), 100);

            vm.expectEmit();
            emit acceptOrdinalSellOrderEvent(
                expectedPlaceId, expectedAcceptId, ordinalsInfo[i].requester, address(token1), 100
            );
            this.acceptOrdinalSellOrder(expectedPlaceId, ordinalsInfo[i].requester);
            assertEq(expectedAcceptId, expectedAcceptId);

            // proofOrdinalSellOrder
            vm.startPrank(alice);
            vm.expectEmit();
            emit proofOrdinalSellOrderEvent(expectedAcceptId);
            this.proofOrdinalSellOrder(expectedAcceptId, ordinalsInfo[i].info, ordinalsInfo[i].proof);
            vm.stopPrank();
        }
    }

    function test_PlaceOrdinalSellOrderShouldRevert() public {
        token1.sudoMint(bob, 200);
        // placeOrdinalSellOrder by alice
        vm.startPrank(alice);
        vm.expectRevert("Invalid buying token");
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[0].utxo, address(0x0), 100);

        vm.expectRevert("Buying amount should be greater than 0");
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[0].utxo, address(token1), 0);

        vm.stopPrank();
    }

    function setUpForAcceptOrdinalSellOrder() public {
        token1.sudoMint(bob, 200);
        // placeOrdinalSellOrder by alice
        vm.startPrank(alice);
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[0].utxo, address(token1), 100);
    }

    function test_AcceptOrdinalSellOrderShouldRevert() public {
        setUpForAcceptOrdinalSellOrder();

        vm.startPrank(bob);

        // allow insufficient tokens
        token1.approve(address(this), 50);
        vm.expectRevert("ERC20: insufficient allowance");
        this.acceptOrdinalSellOrder(0, ordinalsInfo[0].requester);

        // call with wrong id
        token1.approve(address(this), 100);
        vm.expectRevert("Address: call to non-contract");
        this.acceptOrdinalSellOrder(1, ordinalsInfo[0].requester);
        vm.stopPrank();
    }

    function test_AcceptOrdinalSellOrderWhenOrderAlreadyAccepted() public {
        setUpForProofOrdinalSellOrder();

        // acceptOrdinalSellOrder by bob
        vm.startPrank(bob);
        token1.approve(address(this), 100);
        vm.expectRevert("Order Already Accepted");
        this.acceptOrdinalSellOrder(0, ordinalsInfo[0].requester);
    }

    function setUpForProofOrdinalSellOrder() public {
        token1.sudoMint(bob, 200);
        // placeOrdinalSellOrder by alice
        vm.startPrank(alice);
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[0].utxo, address(token1), 100);

        // acceptOrdinalSellOrder by bob
        vm.startPrank(bob);
        token1.approve(address(this), 100);
        this.acceptOrdinalSellOrder(0, ordinalsInfo[0].requester);
    }

    function test_ProofOrdinalSellOrderShouldRevert() public {
        setUpForProofOrdinalSellOrder();

        // when sender is not the requester
        vm.startPrank(bob);
        vm.expectRevert("Sender not the requester");
        this.proofOrdinalSellOrder(1, ordinalsInfo[0].info, ordinalsInfo[0].proof);

        // with invalid id
        vm.startPrank(alice);
        vm.expectRevert("Sender not the requester");
        this.proofOrdinalSellOrder(2, ordinalsInfo[0].info, ordinalsInfo[0].proof);
        vm.stopPrank();
    }

    function test_acceptProofOrdinalSellOrderWithInvalidMerkelProof() public {
        setUpForProofOrdinalSellOrder();
        // with invalid proof
        vm.startPrank(alice);
        vm.expectRevert("Tx merkle proof is not valid for provided header and tx hash");
        this.proofOrdinalSellOrder(1, ordinalsInfo[0].info, ordinalsInfo[1].proof);
        vm.stopPrank();
    }

    function test_AcceptProofOrdinalSellOrderWithUtxoSpentOnInCorrectAddress() public {
        setUpForAcceptOrdinalSellOrder();

        // acceptOrdinalSellOrder by bob
        vm.startPrank(bob);
        token1.approve(address(this), 100);
        this.acceptOrdinalSellOrder(0, ordinalsInfo[1].requester);

        vm.startPrank(alice);
        vm.expectRevert("No output found for scriptPubKey");
        this.proofOrdinalSellOrder(1, ordinalsInfo[0].info, ordinalsInfo[0].proof);
        vm.stopPrank();
    }

    function test_AcceptProofOrdinalSellOrderWithInvalidUtxoSpent() public {
        token1.sudoMint(bob, 200);
        // placeOrdinalSellOrder by alice
        vm.startPrank(alice);
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[1].utxo, address(token1), 100);

        // acceptOrdinalSellOrder by bob
        vm.startPrank(bob);
        token1.approve(address(this), 100);
        this.acceptOrdinalSellOrder(0, ordinalsInfo[0].requester);

        vm.startPrank(alice);
        vm.expectRevert("Transaction does not spend the required utxo");
        this.proofOrdinalSellOrder(1, ordinalsInfo[0].info, ordinalsInfo[0].proof);
    }

    function test_WithdrawOrdinalSellOrder() public {
        // placeOrdinalSellOrder by alice
        vm.startPrank(alice);
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[1].utxo, address(token1), 100);
        this.withdrawOrdinalSellOrder(0);

        (, uint256[] memory ids) = this.getOpenOrdinalSellOrders();

        // there should be no order  ids
        assertEq(ids.length, 0);
    }

    function test_AcceptOrdinalSellOrderWhenOrderIsAlreadyWithdrawnShouldRevert() public {
        token1.sudoMint(bob, 100);

        // placeOrdinalSellOrder by alice
        setUpForAcceptOrdinalSellOrder();

        // withdraw sell order
        this.withdrawOrdinalSellOrder(0);

        // try accepting withdraw order
        vm.startPrank(bob);
        token1.approve(address(this), 100);
        // since order is withdrawn it will try to send tokens to default address
        vm.expectRevert("Address: call to non-contract");
        this.acceptOrdinalSellOrder(0, ordinalsInfo[0].requester);
        vm.stopPrank();
    }

    function test_WithdrawOrdinalSellOrderShouldRevert() public {
        // placeOrdinalSellOrder by alice
        vm.startPrank(alice);
        this.placeOrdinalSellOrder(ordinalsInfo[0].id, ordinalsInfo[1].utxo, address(token1), 100);

        vm.startPrank(bob);
        vm.expectRevert("Sender not the requester");
        this.withdrawOrdinalSellOrder(0);
    }

    function test_CancelAcceptedOrdinalSellOrder() public {
        setUpForProofOrdinalSellOrder();
        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);
        vm.startPrank(bob);
        this.cancelAcceptedOrdinalSellOrder(1);
        (, uint256[] memory ids) = this.getOpenAcceptedOrdinalSellOrders();
        // there should be no order ids
        assertEq(ids.length, 0);
    }

    function test_ProofOrdinalSellOrderWhenOrderIsAlreadyCancelledShouldRevert() public {
        setUpForProofOrdinalSellOrder();

        vm.startPrank(bob);
        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);
        this.cancelAcceptedOrdinalSellOrder(1);

        // proofOrdinalSellOrder when order cancelled
        vm.startPrank(alice);
        // since order is cancelled it will give a default value for the address which is not match alice address
        vm.expectRevert("Sender not the requester");
        this.proofOrdinalSellOrder(1, ordinalsInfo[0].info, ordinalsInfo[0].proof);
        vm.stopPrank();
    }

    function test_CancelAcceptedOrdinalSellOrderShouldRevert() public {
        setUpForProofOrdinalSellOrder();

        vm.startPrank(bob);
        vm.expectRevert("Request still valid");
        this.cancelAcceptedOrdinalSellOrder(1);

        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);

        vm.startPrank(alice);
        vm.expectRevert("Sender not the acceptor");
        this.cancelAcceptedOrdinalSellOrder(1);
    }
}
