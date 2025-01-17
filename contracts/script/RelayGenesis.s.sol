// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {LightRelay} from "../src/relay/LightRelay.sol";
import {TestLightRelay} from "../src/relay/TestLightRelay.sol";

contract RelayGenesisScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        bytes memory genesisHeader = vm.envBytes("GENESIS_HEADER");
        uint256 genesisHeight = vm.envUint("GENESIS_HEIGHT");
        uint256 genesisProofLength = vm.envUint("GENESIS_PROOF_LENGTH");
        bytes memory retargetHeaders = vm.envBytes("RETARGET_HEADERS");

        vm.startBroadcast(deployerPrivateKey);

        LightRelay relay;

        if (vm.envOr("TESTNET", false)) {
            console2.log("Deploying testnet relay");
            relay = new TestLightRelay();
        } else {
            console2.log("Deploying mainnet relay");
            relay = new LightRelay();
        }

        // initialize relay at the given block
        relay.genesis(genesisHeader, genesisHeight, uint64(genesisProofLength));
        console2.log("Current epoch %s", relay.currentEpoch());

        if (!vm.envOr("TESTNET", false)) {
            console2.log("Submitting retarget headers");
            relay.retarget(retargetHeaders);
            console2.log("Current epoch %s", relay.currentEpoch());
        }

        vm.stopBroadcast();
    }
}
