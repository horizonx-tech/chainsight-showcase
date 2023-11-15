// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.21;

import {Script, console2} from "forge-std/Script.sol";
import {Factory} from "../src/Factory.sol";

contract FactoryScript is Script {
    bytes32 constant byteText = "Chainsight v0.1";

    function setUp() public {}

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        Factory factory = new Factory{salt: byteText}();
        address oracle = factory.deploy(byteText);
        console2.log("Chainsight Oarcle Address: ", oracle);

        vm.stopBroadcast();
    }
}
