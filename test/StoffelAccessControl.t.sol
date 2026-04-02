// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";

contract StoffelAccessControlTest is Test {
    StoffelAccessControl public stoffelAccessControl;

    function setUp() public {
	uint256 t = 1;
    	address[] memory initialMpcNodes = new address[](5);
	initialMpcNodes[0] = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
	initialMpcNodes[1] = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8;
	initialMpcNodes[2] = 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC;
	initialMpcNodes[3] = 0x90F79bf6EB2c4f870365E785982E1f101E93b906;
	initialMpcNodes[4] = 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65;

        stoffelAccessControl = new StoffelAccessControl(t, initialMpcNodes);
    }

    function test_grantPartyRole() public {
        address accountParty = makeAddr("PARTY");
        stoffelAccessControl.grantRole(stoffelAccessControl.PARTY_ROLE(), accountParty);
    }
}
