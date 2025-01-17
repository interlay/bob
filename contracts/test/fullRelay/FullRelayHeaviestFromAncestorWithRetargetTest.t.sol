// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {FullRelay} from "../../src/relay/FullRelay.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayHeaviestFromAncestorWithRetargetTest is FullRelayTestUtils {
    using stdJson for string;

    bytes[] preHeaderHexes;
    bytes[] postHeaderHexes;
    bytes[] postShortHeaderHexes;

    bytes32 genesisDigestLe;
    bytes orphanHex;
    bytes32 orphanDigestLe;

    uint256 constant preLength = 5;
    uint256 constant postLength = 8;

    constructor()
        FullRelayTestUtils("headersReorgAndRetarget.json", ".genesis.hex", ".genesis.height", ".oldPeriodStart.digest_le")
    {
        preHeaderHexes = getHeaderHexes("preRetargetChain", 0, preLength);
        postHeaderHexes = getHeaderHexes("postRetargetChain", 0, postLength);
        postShortHeaderHexes = getHeaderHexes("postRetargetChain", 0, postLength - 2);
        bytes memory preHeaders = getHeaders("preRetargetChain", 0, preLength);
        bytes memory postHeaders = getHeaders("postRetargetChain", 0, postLength);
        bytes memory postShortHeaders = getHeaders("postRetargetChain", 0, postLength - 2);

        orphanHex = json.readBytes(".orphan_437478.hex");
        orphanDigestLe = json.readBytes32(".orphan_437478.digest_le");
        bytes memory postWithOrphan = bytes.concat(postShortHeaders, orphanHex);

        bytes memory genesisHex = json.readBytes(".genesis.hex");
        genesisDigestLe = json.readBytes32(".genesis.digest_le");
        bytes memory oldPeriodStartHex = json.readBytes(".oldPeriodStart.hex");
        relay.addHeaders(genesisHex, preHeaders);
        relay.addHeadersWithRetarget(oldPeriodStartHex, preHeaderHexes[preLength - 1], postHeaders);
        relay.addHeadersWithRetarget(oldPeriodStartHex, preHeaderHexes[preLength - 1], postWithOrphan);
    }

    function testHandlingDescendantsAtDifferentDifficultyPeriod() public {
        assertEq(relay.heaviestFromAncestor(genesisDigestLe, orphanHex, preHeaderHexes[3]), orphanDigestLe);
        assertEq(relay.heaviestFromAncestor(genesisDigestLe, preHeaderHexes[3], orphanHex), orphanDigestLe);
    }

    function testHandlingDescendantsWhenBothAtDifferentNewDifficultyPeriod() public {
        assertEq(relay.heaviestFromAncestor(genesisDigestLe, orphanHex, postHeaderHexes[3]), orphanDigestLe);
        assertEq(relay.heaviestFromAncestor(genesisDigestLe, postHeaderHexes[3], orphanHex), orphanDigestLe);
    }
}
