// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {TestRelay} from "../../src/relay/FullRelayWithVerify.sol";

import {stdJson} from "forge-std/StdJson.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayConstructionTest is FullRelayTestUtils {
    using stdJson for string;

    bytes32 genesisDigestLe;
    bytes32 orphanDigestLe;
    uint256 genesisDifficulty;
    uint256 genesisHeight;
    bytes genesisHex;

    constructor() FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".orphan_562630.digest_le") {
        genesisDigestLe = json.readBytes32(".genesis.digest_le");
        orphanDigestLe = json.readBytes32(".orphan_562630.digest");
        genesisDifficulty = json.readUint(".genesis.difficulty");
        genesisHeight = json.readUint(".genesis.height");
        genesisHex = json.readBytes(".genesis.hex");
    }

    function testBadGenesisBlock() external {
        vm.expectRevert(bytes("Bad genesis block"));
        relay = new TestRelay("", genesisHeight, orphanDigestLe);
    }

    function testPeriodStartWrongByteOrder() external {
        vm.expectRevert(bytes("Period start hash does not have work. Hint: wrong byte order?"));
        new TestRelay(genesisHex, genesisHeight, orphanDigestLe);
    }

    function testStoresGenesisBlockInfo() external {
        assertEq(relay.getRelayGenesis(), genesisDigestLe);

        assertEq(relay.getBestKnownDigest(), genesisDigestLe);

        assertEq(relay.getLastReorgCommonAncestor(), genesisDigestLe);

        assertEq(relay.findAncestor(genesisDigestLe, 0), genesisDigestLe);

        assertEq(relay.findHeight(genesisDigestLe), genesisHeight);

        assertEq(relay.getCurrentEpochDifficulty(), genesisDifficulty);

        assertEq(relay.getPrevEpochDifficulty(), 0);
    }
}
