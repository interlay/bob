// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "../src/paymasters/OracleTokenPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";
// import {IERC2771} from "../lib/gsn/packages/contracts/src/interfaces/IERC2771Recipient.sol";
import {IRelayHub} from "../lib/gsn/packages/contracts/src/interfaces/IRelayHub.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract OracleTokenPaymasterScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        IRelayHub relay = IRelayHub(vm.envAddress("RELAY_ADDRESS"));
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");
        ERC20 token = ERC20(vm.envAddress("ERC_ADDRESS"));
        IOracle oracle = IOracle(vm.envAddress("ORACLE_ADDRESS"));
        // IERC2771 target = IERC2771(vm.envAddress("TARGET_ADDRESS"));

        vm.startBroadcast(deployerPrivateKey);
        DummyOracle nativeTokenOracle = new DummyOracle();
        OracleTokenPaymaster paymaster = new OracleTokenPaymaster(nativeTokenOracle);

        relay.depositFor{value: 1 ether}(address(paymaster));
        paymaster.setRelayHub(relay);
        paymaster.setTrustedForwarder(forwarder);

        nativeTokenOracle.setPrice(189100000000); // 1 eth = 1891usd

        paymaster.addOracle(token, token.decimals(), oracle);

        vm.stopBroadcast();
    }
}
