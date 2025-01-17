// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {FullRelay} from "../../src/relay/FullRelay.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayFindHeightTest is FullRelayTestUtils {
    using stdJson for string;

    uint256 constant blockNumberAfterGenesis = 6;

    constructor()
        FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".fakePeriodStartHeader.digest_le")
    {
        relay.addHeaders(json.readBytes(".genesis.hex"), getHeaders("chain", 0, blockNumberAfterGenesis));
    }

    function testFindUnknownBlock() public {
        vm.expectRevert(bytes("Unknown block"));
        relay.findHeight(hex"00000000000000000000000000000000");
    }

    function testFindHeightOfExistingBlocks() public {
        bytes32[] memory digestLes = getDigestLes("chain", 0, blockNumberAfterGenesis);
        uint256[] memory blockHeights = getBlockHeights("chain", 0, blockNumberAfterGenesis);
        for (uint256 i; i < blockNumberAfterGenesis; ++i) {
            assertEq(relay.findHeight(digestLes[i]), blockHeights[i]);
        }
    }
}
