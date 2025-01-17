// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayWithVerifyTest is FullRelayTestUtils {
    using stdJson for string;

    uint256 constant CHAIN_LENGTH = 18;
    bytes header =
        hex"0000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d38782b";
    bytes proof =
        hex"e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64";
    bytes32 txToVerify = hex"48e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6";
    uint256 txId = 281;

    constructor() FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".genesis.digest_le") {
        relay.setAncestorOverride(true, true);
    }

    function testMalformedProofSupplied() public {
        vm.expectRevert(bytes("Bad merkle array proof"));
        relay.verifyProof(header, hex"00", txToVerify, txId, 0);
    }

    function testIncorrectProofSupplied() public {
        vm.expectRevert(bytes("Bad inclusion proof"));
        relay.verifyProof(
            header, hex"0000000000000000000000000000000000000000000000000000000000000000", txToVerify, txId, 0
        );
    }

    function testIncorrectTxSupplied() public {
        vm.expectRevert(bytes("Bad inclusion proof"));
        relay.verifyProof(header, proof, hex"00", txId, 0);
    }

    function testIncorrectTxIdSupplied() public {
        vm.expectRevert(bytes("Bad inclusion proof"));
        relay.verifyProof(header, proof, txToVerify, txId + 1, 0);
    }

    function testGCDDoesntConfirmHeader() public {
        relay.setAncestorOverride(false, false);
        vm.expectRevert(bytes("GCD does not confirm header"));
        relay.verifyProof(header, proof, txToVerify, txId, 0);
    }

    function testInsufficientConfirmations() public {
        vm.expectRevert(bytes("Insufficient confirmations"));
        relay.verifyProof(header, proof, txToVerify, txId, 9);
    }

    function testSuccessfullyVerify() public view {
        relay.verifyProof(header, proof, txToVerify, txId, 0);
    }
}
