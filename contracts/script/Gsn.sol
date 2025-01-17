// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "../src/paymasters/OracleTokenPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";
import {IRelayHub} from "../lib/gsn/packages/contracts/src/interfaces/IRelayHub.sol";
import {TestingErc20} from "../src/TestingErc20.sol";

contract TestingWbtcScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        address forwarder = vm.envAddress("FORWARDER_ADDRESS");
        int256 tokenPrice = vm.envInt("ERC_PRICE");
        // address payable mtPaymaster = payable(vm.envAddress("MT_PAYMASTER_ADDRESS"));

        vm.startBroadcast(deployerPrivateKey);

        TestingErc20 token = new TestingErc20("Wrapped BTC", "WBTC", 8);
        token.setTrustedForwarder(forwarder);

        DummyOracle oracle = new DummyOracle();
        oracle.setPrice(tokenPrice);

        // OracleTokenPaymaster paymaster = OracleTokenPaymaster(mtPaymaster);
        // mtPaymaster.addOracle(token, token.decimals(), oracle);

        vm.stopBroadcast();
    }
}
