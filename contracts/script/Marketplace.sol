// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {BtcMarketPlace} from "../src/swap/Btc_Marketplace.sol";
import {OrdMarketplace} from "../src/swap/Ord_Marketplace.sol";
import {MarketPlace} from "../src/swap/Marketplace.sol";
import {TestLightRelay} from "../src/relay/TestLightRelay.sol";

contract MarketplaceScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);

        TestLightRelay relay = new TestLightRelay();
        new BtcMarketPlace(relay, forwarder);
        new OrdMarketplace(relay);
        new MarketPlace(forwarder);

        vm.stopBroadcast();
    }
}
