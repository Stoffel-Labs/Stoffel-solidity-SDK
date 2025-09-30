// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";

contract StoffelAccessControlScript is Script {
    StoffelAccessControl public stoffel_access_control;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        stoffel_access_control = new StoffelAccessControl(5, 3);

        vm.stopBroadcast();
    }
}
