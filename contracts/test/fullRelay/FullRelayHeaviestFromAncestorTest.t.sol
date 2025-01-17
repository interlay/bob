// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {FullRelay} from "../../src/relay/FullRelay.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayHeaviestFromAncestorTest is FullRelayTestUtils {
    using stdJson for string;

    bytes32 genesisDigestLe;
    bytes32[] digestLes;
    bytes[] headersHexes;
    bytes headersWithMain;
    bytes headersWithOrphan;
    bytes32 unknownDigestLe;
    bytes unknownHex;
    uint256 constant blockNumberAfterGenesis = 8;

    constructor() FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".genesis.digest_le") {
        digestLes = getDigestLes("chain", 0, blockNumberAfterGenesis);
        genesisDigestLe = json.readBytes32(".genesis.digest_le");

        headersHexes = getHeaderHexes("chain", 0, blockNumberAfterGenesis);
        bytes memory headers = getHeaders("chain", 0, blockNumberAfterGenesis);
        headersWithMain =
            bytes.concat(headers, getHeaders("chain", blockNumberAfterGenesis, blockNumberAfterGenesis + 1));
        headersWithOrphan = bytes.concat(headers, json.readBytes(".orphan_562630.hex"));
        unknownDigestLe =
            json.readBytes32(string.concat(".chain[", Strings.toString(blockNumberAfterGenesis + 2), "].digest_le"));
        unknownHex = json.readBytes(string.concat(".chain[", Strings.toString(blockNumberAfterGenesis + 2), "].hex"));

        bytes memory genesisHex = json.readBytes(".genesis.hex");
        relay.addHeaders(genesisHex, headersWithMain);
        relay.addHeaders(genesisHex, headersWithOrphan);
    }

    function testUnknownAncestor() public {
        vm.expectRevert(bytes("Unknown block"));
        relay.heaviestFromAncestor(unknownDigestLe, headersHexes[3], headersHexes[4]);
    }

    function testUnknownLeft() public {
        vm.expectRevert(bytes("Unknown block"));
        relay.heaviestFromAncestor(digestLes[3], unknownHex, headersHexes[4]);
    }

    function testUnknownRight() public {
        vm.expectRevert(bytes("Unknown block"));
        relay.heaviestFromAncestor(digestLes[3], headersHexes[4], unknownHex);
    }

    function testDescendantLeftBelowAncestor() public {
        vm.expectRevert(bytes("A descendant height is below the ancestor height"));
        relay.heaviestFromAncestor(digestLes[3], headersHexes[2], headersHexes[4]);
    }

    function testDescendantRightBelowAncestor() public {
        vm.expectRevert(bytes("A descendant height is below the ancestor height"));
        relay.heaviestFromAncestor(digestLes[3], headersHexes[4], headersHexes[2]);
    }

    function testLeftIsHeavier() public {
        assertEq(relay.heaviestFromAncestor(digestLes[3], headersHexes[5], headersHexes[4]), digestLes[5]);
    }

    function testRightIsHeavier() public {
        assertEq(relay.heaviestFromAncestor(digestLes[3], headersHexes[4], headersHexes[5]), digestLes[5]);
    }

    function testEqualWeightsReturnsLeft() public {
        bytes32 orpan562640DigestLe = json.readBytes32(".orphan_562630.digest_le");
        bytes memory orpan562640HeaderHex = json.readBytes(".orphan_562630.hex");
        bytes32 headerDigestLeMain = getDigestLes("chain", blockNumberAfterGenesis, blockNumberAfterGenesis + 1)[0];
        bytes memory headerHexMain = getHeaderHexes("chain", blockNumberAfterGenesis, blockNumberAfterGenesis + 1)[0];

        assertEq(relay.heaviestFromAncestor(digestLes[3], headerHexMain, orpan562640HeaderHex), headerDigestLeMain);
        assertEq(relay.heaviestFromAncestor(digestLes[3], orpan562640HeaderHex, headerHexMain), orpan562640DigestLe);
    }
}
