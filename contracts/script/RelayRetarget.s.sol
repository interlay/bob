// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {ILightRelay} from "../src/relay/LightRelay.sol";

contract RelayRetargetScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        address relayAddress = vm.envAddress("RELAY_ADDRESS");
        bytes memory retargetHeaders = vm.envBytes("RETARGET_HEADERS");

        vm.startBroadcast(deployerPrivateKey);
        ILightRelay relay = ILightRelay(relayAddress);

        // submit retarget
        relay.retarget(retargetHeaders);

        vm.stopBroadcast();
    }
}
