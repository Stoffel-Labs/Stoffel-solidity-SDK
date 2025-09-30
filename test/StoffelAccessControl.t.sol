// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";

contract StoffelAccessControlTest is Test {
    StoffelAccessControl public stoffel_access_control;

    function setUp() public {
        stoffel_access_control = new StoffelAccessControl(5, 3);
    }

    function test_grantPartyRole() public {
        address account_party = makeAddr("PARTY");
        stoffel_access_control.grantRole(stoffel_access_control.PARTY_ROLE(), account_party);
    }
}
