// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.4;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import {Test, console2} from "forge-std/Test.sol";

import {BitcoinTx} from "../src/utils/BitcoinTx.sol";
import {WitnessTx} from "../src/utils/WitnessTx.sol";

contract WitnessTxTest is Test {
    using BTCUtils for bytes;
    using ValidateSPV for bytes32;

    function test_GetWitnessTxHash() public {
        bytes32 wTxId = hex"6b374690d8e2dbca4187f443cddd293536400d431f43a643b263ce59c4f9a3eb";

        WitnessTx.WitnessInfo memory txInfo = WitnessTx.WitnessInfo({
            info: BitcoinTx.Info({
                version: hex"01000000",
                inputVector: hex"01ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a30000000000fdffffff",
                outputVector: hex"01102700000000000022512037679ea62eab55ebfd442c53c4ad46b6b75e45d8a8fa9cb31a87d0df268b029a",
                locktime: hex"00000000"
            }),
            witnessVector: hex"03406c00eb3c4d35fedd257051333b4ca81d1a25a37a9af4891f1fec2869edd56b14180eafbda8851d63138a724c9b15384bc5f0536de658bd294d426a36212e6f08a5209e2849b90a2353691fccedd467215c88eec89a5d0dcf468e6cf37abed344d746ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d38004c5e7b200a20202270223a20226272632d3230222c0a2020226f70223a20226465706c6f79222c0a2020227469636b223a20226f726469222c0a2020226d6178223a20223231303030303030222c0a2020226c696d223a202231303030220a7d6821c19e2849b90a2353691fccedd467215c88eec89a5d0dcf468e6cf37abed344d746"
        });

        bytes32 wTxHash = abi.encodePacked(
            txInfo.info.version,
            WitnessTx.SEGWIT_MARKER,
            WitnessTx.SEGWIT_FLAG,
            txInfo.info.inputVector,
            txInfo.info.outputVector,
            txInfo.witnessVector,
            txInfo.info.locktime
        ).hash256View();

        assertEq(wTxId, wTxHash);
    }

    function test_ProveWitnessTx() public {
        bytes32 txId = hex"6b374690d8e2dbca4187f443cddd293536400d431f43a643b263ce59c4f9a3eb";

        bytes32 witnessMerkleRoot = hex"7cee5e99c8f0fc25fb115b7d7d00befca61f59a8544adaf3980f52132baf61ae";
        bytes memory witnessMerkleProof =
            hex"6034ddf453f5dd20de449b29b1221dede67ccae56f00528e0767e2ab506db31c4d2946e88f7efa3e94bb17bbd10f3f44172b59c48f2eb6bd7f67a88d149373ee4082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107aca93b7c3c4f91ddf49c7f74244336c5833377d40760ae09dd1fba83063ace480f94cca3920a489b23f9133fc84d7987d990acc7c2569a81b547a5f65385856d90100e84878b4f305a3909a9420293cdc741109864c9338ea326449a7a303b227f2b10490bc4343355e1a391f51c42918a894c2980012cca5ffd4b56a6702abd98497802de83f5889b2ad5bd157762a58505948f32f42b9fa886c93bf30fef6144a64666843a28ef13184f9e7ac3c34b5741f58c8895a0167f496e0157e7d0a97f4041f97b8df4d8aee81d20d0d062ed3ee0f9b0afb196bdf5373712883cacdfd8349b739c0e6e41d650d05727ea5faec197bfa563d19b0150fba718ba1981aea9ef90";

        uint256 txIndexInBlock = 407;
        require(
            txId.prove(witnessMerkleRoot, witnessMerkleProof, txIndexInBlock),
            "Tx witness merkle proof is not valid for provided header and tx hash"
        );

        // witnessCommitment = SHA256(witnessMerkleRoot || witnessNonce)
        bytes32 witnessNonce = hex"0000000000000000000000000000000000000000000000000000000000000000";
        bytes32 witnessCommitment = abi.encodePacked(witnessMerkleRoot, witnessNonce).hash256();

        bytes32 coinbaseWitnessCommitment = hex"af8dcb9588f94a3adb462e80f1306d96ef6ffad72160b33cd5e90045d81e0d77";
        assertEq(coinbaseWitnessCommitment, witnessCommitment);
    }

    function test_ValidateWitnessProof() public view {
        WitnessTx.validateWitnessProof(
            WitnessTx.WitnessInfo({
                info: BitcoinTx.Info({
                    version: hex"01000000",
                    inputVector: hex"01ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a30000000000fdffffff",
                    outputVector: hex"01102700000000000022512037679ea62eab55ebfd442c53c4ad46b6b75e45d8a8fa9cb31a87d0df268b029a",
                    locktime: hex"00000000"
                }),
                witnessVector: hex"03406c00eb3c4d35fedd257051333b4ca81d1a25a37a9af4891f1fec2869edd56b14180eafbda8851d63138a724c9b15384bc5f0536de658bd294d426a36212e6f08a5209e2849b90a2353691fccedd467215c88eec89a5d0dcf468e6cf37abed344d746ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d38004c5e7b200a20202270223a20226272632d3230222c0a2020226f70223a20226465706c6f79222c0a2020227469636b223a20226f726469222c0a2020226d6178223a20223231303030303030222c0a2020226c696d223a202231303030220a7d6821c19e2849b90a2353691fccedd467215c88eec89a5d0dcf468e6cf37abed344d746"
            }),
            WitnessTx.WitnessProof({
                witnessNonce: hex"0000000000000000000000000000000000000000000000000000000000000000",
                paymentMerkleRoot: hex"7cee5e99c8f0fc25fb115b7d7d00befca61f59a8544adaf3980f52132baf61ae",
                coinbaseProof: BitcoinTx.Proof({
                    bitcoinHeaders: hex"00805d2b0952221f3dcd1e4f9053ea87b238327d99813a8879f00300000000000000000001c6691023f17fd78f2dffc85d9db21b84eb6e77352f494f9437168820dbfb901f0c0864a38906173a54d852",
                    merkleProof: hex"44b53cd654829638a7e1e4681b5d05c2049d1e5f936d159e6c4fca704d3c035127b84e765049e0211073bb8c8aa3df37266cc299a196e78b1a6883c82d7f7f0df32ba0fe04ea9e33eb2bbb1c4fffa4cd02a1af9e4d5f634530704f10dc88bf900eed8b1fb030e0c49316facf0ba1d98fb8979ed899d045192419801453448eaff73bc4f56d8ab7eb379732e4db73a33f2ad9b2ca35aba5fd45ea76aa843edac88292b730b6c07a4396fcb2995e41db66f176f5de25a18f52aeb7c27706c6c7288300c854a83dbb34ae626c3e8a127367c12150c0eac8ee7c436dbc1e0673ba3c7e4a4a552cc6df73919d28e5144e2b8f03a8b53a59fba1136f7f3bb278b1e1c1f9c380b9a9b0278d88b10289a8d14aca3152883256923cf163fd20ac13dfcb858979a263152d6c234ad0f4b70f697168502d62ead0c0194bcf77321a85a1e127afc4477dcc3c3636a7818601d9ff43f837b15ef74d387c688fc0a45b79aec0b6",
                    txIndexInBlock: 0
                }),
                paymentProof: BitcoinTx.Proof({
                    // we don't pass the block headers here since we need the merkle root
                    // calculated using the witness tx ids, only the coinbase headers are needed
                    bitcoinHeaders: hex"",
                    merkleProof: hex"6034ddf453f5dd20de449b29b1221dede67ccae56f00528e0767e2ab506db31c4d2946e88f7efa3e94bb17bbd10f3f44172b59c48f2eb6bd7f67a88d149373ee4082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107aca93b7c3c4f91ddf49c7f74244336c5833377d40760ae09dd1fba83063ace480f94cca3920a489b23f9133fc84d7987d990acc7c2569a81b547a5f65385856d90100e84878b4f305a3909a9420293cdc741109864c9338ea326449a7a303b227f2b10490bc4343355e1a391f51c42918a894c2980012cca5ffd4b56a6702abd98497802de83f5889b2ad5bd157762a58505948f32f42b9fa886c93bf30fef6144a64666843a28ef13184f9e7ac3c34b5741f58c8895a0167f496e0157e7d0a97f4041f97b8df4d8aee81d20d0d062ed3ee0f9b0afb196bdf5373712883cacdfd8349b739c0e6e41d650d05727ea5faec197bfa563d19b0150fba718ba1981aea9ef90",
                    txIndexInBlock: 407
                }),
                coinbaseTx: BitcoinTx.Info({
                    version: hex"02000000",
                    inputVector: hex"010000000000000000000000000000000000000000000000000000000000000000ffffffff310338e60b04200c08642f466f756e6472792055534120506f6f6c202364726f70676f6c642f13367b361659010000000000ffffffff",
                    outputVector: hex"020593cb260000000016001435f6de260c9f3bdee47524c473a6016c0c055cb90000000000000000266a24aa21a9edaf8dcb9588f94a3adb462e80f1306d96ef6ffad72160b33cd5e90045d81e0d77",
                    locktime: hex"00000000"
                })
            })
        );
    }
}
