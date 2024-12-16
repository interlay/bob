// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {BtcMarketPlace} from "../src/swap/Btc_Marketplace.sol";
import {OrdMarketplace} from "../src/swap/Ord_Marketplace.sol";
import {MarketPlace} from "../src/swap/Marketplace.sol";
import {TestFullRelay} from "../src/relay/TestFullRelay.sol";

contract MarketplaceScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);

        // initialize with some dummy values
        TestFullRelay relay = new TestFullRelay(
            hex"00000020db62962b5989325f30f357762ae456b2ec340432278e14000000000000000000d1dd4e30908c361dfeabfb1e560281c1a270bde3c8719dbda7c848005317594440bf615c886f2e17bd6b082d",
            0,
            bytes32(0)
        );
        new BtcMarketPlace(relay, forwarder);
        new OrdMarketplace(relay);
        new MarketPlace(forwarder);

        vm.stopBroadcast();
    }
}
