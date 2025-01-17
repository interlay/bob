// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {IFullRelay} from "../../src/relay/FullRelayInterfaces.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayAddHeaderTest is FullRelayTestUtils {
    using stdJson for string;

    uint256 constant CHAIN_LENGTH = 18;

    bytes headers;
    bytes genesisHex;
    bytes orphan562630Hex;
    bytes regularChainBadHeaderHex;

    constructor() FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".orphan_562630.digest_le") {
        headers = getHeaders("chain", 0, CHAIN_LENGTH);
        genesisHex = json.readBytes(".genesis.hex");
        orphan562630Hex = json.readBytes(".orphan_562630.hex");
        regularChainBadHeaderHex = json.readBytes(".badHeader.hex");
    }

    function testIncorrectAnchorLength() public {
        vm.expectRevert(bytes("Anchor must be 80 bytes"));
        relay.addHeaders("00", headers);
    }

    function testExternalRetarget() public {
        bytes memory badHeaders =
            hex"0000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d38782b0000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000005af53b865c27c6e9b5e5db4c3ea8e024f8329178a79ddb39f7727ea2fe6e6825d1349c5ba1192817e2d951590000002073bd2184edd9c4fc76642ea6754ee40136970efc10c419000000000000000000c63a8848a448a43c9e4402bd893f701cd11856e14cbbe026699e8fdc445b35a8d93c9c5ba1192817b945dc6c00000020f402c0b551b944665332466753f1eebb846a64ef24c71700000000000000000033fc68e070964e908d961cd11033896fa6c9b8b76f64a2db7ea928afa7e304257d3f9c5ba11928176164145d0000ff3f63d40efa46403afd71a254b54f2b495b7b0164991c2d22000000000000000000f046dc1b71560b7d0786cfbdb25ae320bd9644c98d5c7c77bf9df05cbe96212758419c5ba1192817a2bb2caa00000020e2d4f0edd5edd80bdcb880535443747c6b22b48fb6200d0000000000000000001d3799aa3eb8d18916f46bf2cf807cb89a9b1b4c56c3f2693711bf1064d9a32435429c5ba1192817752e49ae0000002022dba41dff28b337ee3463bf1ab1acf0e57443e0f7ab1d000000000000000000c3aadcc8def003ecbd1ba514592a18baddddcd3a287ccf74f584b04c5c10044e97479c5ba1192817c341f595";
        vm.expectRevert(bytes("Unexpected retarget on external call"));
        relay.addHeaders(genesisHex, badHeaders);
    }

    function testIncorrectHeaderChainLength() public {
        bytes memory badHeaders = bytes.concat(headers, hex"42");
        vm.expectRevert(bytes("Header array length must be divisible by 80"));
        relay.addHeaders(genesisHex, badHeaders);
    }

    function testIInsufficientWork() public {
        bytes memory badHeaders = bytes.concat(
            headers,
            hex"0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );
        vm.expectRevert(bytes("Header work is insufficient"));
        relay.addHeaders(genesisHex, badHeaders);
    }

    function testTargetCangesMidchain() public {
        bytes memory badHeaders = bytes.concat(genesisHex, regularChainBadHeaderHex);
        vm.expectRevert(bytes("Headers do not form a consistent chain"));
        relay.addHeaders(genesisHex, badHeaders);
    }

    function testExtensionEventFiring() public {
        bytes32 genesisDigestLe = json.readBytes32(".genesis.digest_le");
        bytes32 lastChainHeaderDigestLe =
            json.readBytes32(string.concat(".chain[", Strings.toString(CHAIN_LENGTH - 1), "].digest_le"));

        vm.expectEmit();
        emit IFullRelay.Extension(genesisDigestLe, lastChainHeaderDigestLe);

        relay.addHeaders(genesisHex, headers);
    }
}
