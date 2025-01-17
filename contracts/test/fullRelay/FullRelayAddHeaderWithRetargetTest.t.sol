// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";
import {TestRelay} from "../../src/relay/FullRelayWithVerify.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayAddHeaderWithRetargetTest is FullRelayTestUtils {
    using stdJson for string;

    bytes headers;
    bytes preChange;
    bytes firstHeaderHex;
    bytes32 firstHeaderDigestLe;
    bytes lastHeaderHex;
    uint256 lastHeaderHeight;
    bytes genesisHex;

    constructor()
        FullRelayTestUtils("headersWithRetarget.json", ".chain[1].hex", ".chain[1].height", ".oldPeriodStart.digest_le")
    {
        preChange = getHeaders("chain", 2, 9);
        headers = getHeaders("chain", 9, 15);
        genesisHex = json.readBytes(".chain[1].hex");
        firstHeaderHex = json.readBytes(".oldPeriodStart.hex");
        firstHeaderDigestLe = json.readBytes32(".oldPeriodStart.digest_le");
        lastHeaderHex = json.readBytes(".chain[8].hex");
        lastHeaderHeight = json.readUint(".chain[8].height");
        relay.addHeaders(genesisHex, preChange);
    }

    function testOldPeriodStartHeaderUnknown() public {
        vm.expectRevert(bytes("Bad args. Check header and array byte lengths."));
        relay.addHeadersWithRetarget(hex"00", lastHeaderHex, headers);
    }

    function testOldPeriodEndHeaderUnknown() public {
        vm.expectRevert(bytes("Unknown block"));
        relay.addHeadersWithRetarget(firstHeaderHex, json.readBytes(".chain[15].hex"), headers);
    }

    function testOldPeriodEndMismatch() public {
        vm.expectRevert(bytes("Must provide the last header of the closing difficulty period"));
        relay.addHeadersWithRetarget(firstHeaderHex, firstHeaderHex, headers);
    }

    function testOldPeriodStartToEndNot2015Blocks() public {
        vm.expectRevert(bytes("Must provide exactly 1 difficulty period"));
        relay.addHeadersWithRetarget(lastHeaderHex, lastHeaderHex, headers);
    }

    function testRetargetPerformedIncorrectly() public {
        relay = new TestRelay(genesisHex, lastHeaderHeight, firstHeaderDigestLe);

        vm.expectRevert(bytes("Invalid retarget provided"));
        relay.addHeadersWithRetarget(firstHeaderHex, genesisHex, headers);
    }

    function testAppendsNewLinksToTheChain() public {
        relay.addHeadersWithRetarget(firstHeaderHex, lastHeaderHex, headers);
        assert(relay.findHeight(json.readBytes32(".chain[10].digest_le")) == lastHeaderHeight + 2);
    }
}
