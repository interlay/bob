// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {FullRelay} from "../../src/relay/FullRelay.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayMarkHeaviestTest is FullRelayTestUtils {
    using stdJson for string;

    bytes[] preHeaderHexes;
    bytes[] postHeaderHexes;
    bytes32[] preDigestLes;
    bytes32[] postDigestLes;

    bytes genesisHex;
    bytes32 genesisDigestLe;
    bytes orphanHex;
    bytes32 orphanDigestLe;
    bytes oldPeriodStartHex;
    bytes32 oldPeriodStartDigestLe;

    uint256 constant preLength = 5;
    uint256 constant postLength = 8;

    constructor()
        FullRelayTestUtils("headersReorgAndRetarget.json", ".genesis.hex", ".genesis.height", ".oldPeriodStart.digest_le")
    {
        preHeaderHexes = getHeaderHexes("preRetargetChain", 0, preLength);
        postHeaderHexes = getHeaderHexes("postRetargetChain", 0, postLength);
        preDigestLes = getDigestLes("preRetargetChain", 0, preLength);
        postDigestLes = getDigestLes("postRetargetChain", 0, postLength);
        bytes memory preHeaders = getHeaders("preRetargetChain", 0, preLength);
        bytes memory postHeaders = getHeaders("postRetargetChain", 0, postLength);
        bytes memory postShortHeaders = getHeaders("postRetargetChain", 0, postLength - 2);

        orphanHex = json.readBytes(".orphan_437478.hex");
        orphanDigestLe = json.readBytes32(".orphan_437478.digest_le");
        bytes memory postWithOrphan = bytes.concat(postShortHeaders, orphanHex);

        oldPeriodStartHex = json.readBytes(".oldPeriodStart.hex");
        oldPeriodStartDigestLe = json.readBytes32(".oldPeriodStart.digest_le");

        genesisHex = json.readBytes(".genesis.hex");
        genesisDigestLe = json.readBytes32(".genesis.digest_le");
        relay.addHeaders(genesisHex, preHeaders);
        relay.addHeadersWithRetarget(oldPeriodStartHex, preHeaderHexes[preLength - 1], postHeaders);
        relay.addHeadersWithRetarget(oldPeriodStartHex, preHeaderHexes[preLength - 1], postWithOrphan);
    }

    function testPassedInNotTheBestKnown() public {
        vm.expectRevert(bytes("Passed in best is not best known"));
        relay.markNewHeaviest(oldPeriodStartDigestLe, oldPeriodStartHex, oldPeriodStartHex, 10);
    }

    function testPassedInBestKnowIsUnknown() public {
        vm.expectRevert(bytes("New best is unknown"));
        relay.markNewHeaviest(
            genesisDigestLe,
            genesisHex,
            hex"9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
            10
        );
    }

    function testPassedInAncestorNotTheHeaviestCommon() public {
        relay.markNewHeaviest(genesisDigestLe, genesisHex, preHeaderHexes[0], 10);

        vm.expectRevert(bytes("Ancestor must be heaviest common ancestor"));
        relay.markNewHeaviest(genesisDigestLe, preHeaderHexes[0], preHeaderHexes[1], 10);
    }

    function testSuccessfullyMarkHeaviest() public {
        relay.markNewHeaviest(genesisDigestLe, genesisHex, preHeaderHexes[0], 10);

        vm.expectEmit();
        emit IFullRelay.NewTip(preDigestLes[0], orphanDigestLe, preDigestLes[0]);
        relay.markNewHeaviest(preDigestLes[0], preHeaderHexes[0], orphanHex, 20);

        vm.expectRevert(bytes("New best hash does not have more work than previous"));
        relay.markNewHeaviest(postDigestLes[5], orphanHex, postHeaderHexes[6], 10);
    }
}
