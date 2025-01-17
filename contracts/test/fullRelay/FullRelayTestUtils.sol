// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {TestRelay} from "../../src/relay/FullRelayWithVerify.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {Test, console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract FullRelayTestUtils is Test {
    using stdJson for string;

    TestRelay relay;
    string json;

    constructor(
        string memory testFileName,
        string memory genesisHexPath,
        string memory genesisHeightPath,
        string memory periodStartPath
    ) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/test/fullRelay/testData/", testFileName);
        json = vm.readFile(path);
        relay = new TestRelay(
            json.readBytes(genesisHexPath), json.readUint(genesisHeightPath), json.readBytes32(periodStartPath)
        );
    }

    function getHeaders(string memory chainName, uint256 from, uint256 to) public view returns (bytes memory headers) {
        bytes[] memory headerHexes = getHeaderHexes(chainName, from, to);
        for (uint256 i = 0; i < to - from; i++) {
            headers = bytes.concat(headers, headerHexes[i]);
        }
    }

    function getHeaderHexes(string memory chainName, uint256 from, uint256 to)
        public
        view
        returns (bytes[] memory elements)
    {
        return getBytes(chainName, from, to, "hex");
    }

    function getDigestLes(string memory chainName, uint256 from, uint256 to)
        public
        view
        returns (bytes32[] memory elements)
    {
        return getBytes32Array(chainName, from, to, "digest_le");
    }

    function getBlockHeights(string memory chainName, uint256 from, uint256 to)
        public
        view
        returns (uint256[] memory elements)
    {
        return getUint256Array(chainName, from, to, "height");
    }

    function getBytes(string memory chainName, uint256 from, uint256 to, string memory elementName)
        internal
        view
        returns (bytes[] memory elements)
    {
        elements = new bytes[](to - from);
        for (uint256 i = from; i < to; i++) {
            elements[i - from] =
                json.readBytes(string.concat(".", chainName, "[", Strings.toString(i), "].", elementName));
        }
    }

    function getBytes32Array(string memory chainName, uint256 from, uint256 to, string memory elementName)
        internal
        view
        returns (bytes32[] memory elements)
    {
        elements = new bytes32[](to - from);
        for (uint256 i = from; i < to; i++) {
            elements[i - from] =
                json.readBytes32(string.concat(".", chainName, "[", Strings.toString(i), "].", elementName));
        }
    }

    function getUint256Array(string memory chainName, uint256 from, uint256 to, string memory elementName)
        internal
        view
        returns (uint256[] memory elements)
    {
        elements = new uint256[](to - from);
        for (uint256 i = from; i < to; i++) {
            elements[i - from] =
                json.readUint(string.concat(".", chainName, "[", Strings.toString(i), "].", elementName));
        }
    }
}
