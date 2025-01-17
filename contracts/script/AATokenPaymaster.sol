// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {PimlicoERC20Paymaster, IERC20, IEntryPoint} from "../src/paymasters/AccountAbstraction/AATokenPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";

contract AATokenPaymasterScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        IERC20 token = IERC20(vm.envAddress("ERC_ADDRESS"));
        int256 tokenPrice = vm.envInt("ERC_PRICE");
        uint8 tokenDecimals = uint8(vm.envUint("ERC_DECIMALS"));
        IEntryPoint entrypoint = IEntryPoint(vm.envAddress("ENTRYPOINT_ADDRESS"));
        address owner = vm.envAddress("OWNER_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);

        DummyOracle ethOracle = new DummyOracle();
        ethOracle.setPrice(189100000000); // 1 eth = 1891usd
        DummyOracle tokenOracle = new DummyOracle();
        tokenOracle.setPrice(tokenPrice); // set usd price * 10^8

        PimlicoERC20Paymaster paymaster =
            new PimlicoERC20Paymaster(token, entrypoint, tokenOracle, ethOracle, owner, tokenDecimals);

        paymaster.updatePrice();
        paymaster.addStake{value: 0.1 ether}(1);
        entrypoint.depositTo{value: 1 ether}(address(paymaster));
        vm.stopBroadcast();
    }
}
