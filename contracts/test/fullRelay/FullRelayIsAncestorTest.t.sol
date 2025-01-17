// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {FullRelay} from "../../src/relay/FullRelay.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayIsAncestorTest is FullRelayTestUtils {
    using stdJson for string;

    bytes32 genesisDigestLe;
    bytes32[] digestLes;
    uint256 blockNumberAfterGenesis = 6;

    constructor() FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".genesis.digest_le") {
        digestLes = getDigestLes("chain", 0, blockNumberAfterGenesis);
        genesisDigestLe = json.readBytes32(".genesis.digest_le");
        relay.addHeaders(json.readBytes(".genesis.hex"), getHeaders("chain", 0, blockNumberAfterGenesis));
    }

    function testExceedSearchLimit() public {
        assertFalse(relay.isAncestor(genesisDigestLe, digestLes[3], 1));
    }

    function testAncestorFound() public {
        assertTrue(relay.isAncestor(genesisDigestLe, digestLes[3], 5));
    }
}
