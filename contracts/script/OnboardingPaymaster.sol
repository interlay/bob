// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "../src/paymasters/OnboardingPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";
// import {IERC2771} from "../lib/gsn/packages/contracts/src/interfaces/IERC2771Recipient.sol";
import {IRelayHub} from "../lib/gsn/packages/contracts/src/interfaces/IRelayHub.sol";

contract OnboardingPaymasterScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        IRelayHub relay = IRelayHub(vm.envAddress("RELAY_ADDRESS"));
        // bytes memory b = vm.envBytes("WHITELIST_SELECTOR");
        // (uint32 whitelistSelector) = abi.decode(b, (uint32));
        uint256 tmp = vm.envUint("WHITELIST_SELECTOR");
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");
        uint32 whitelistSelector = uint32(tmp);
        address whitelistAddress = vm.envAddress("WHITELIST_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);
        OnboardingPaymaster paymaster = new OnboardingPaymaster(whitelistAddress, whitelistSelector);

        relay.depositFor{value: 1 ether}(address(paymaster));
        paymaster.setRelayHub(relay);
        paymaster.setTrustedForwarder(forwarder);

        vm.stopBroadcast();
    }
}
