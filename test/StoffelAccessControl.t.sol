// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";
import {IStoffelAccessControl} from "../src/interfaces/IStoffelAccessControl.sol";

/// @notice Tests for StoffelAccessControl: role granting/revoking, threshold enforcement, and interface support.
contract StoffelAccessControlTest is Test {
    StoffelAccessControl public ac;

    address designatedParty = makeAddr("DESIGNATED");
    address party1 = makeAddr("PARTY1");
    address party2 = makeAddr("PARTY2");
    address party3 = makeAddr("PARTY3");

    bytes32 partyRole;
    bytes32 designatedPartyRole;

    function setUp() public {
        address[] memory nodes = new address[](4);
        nodes[0] = designatedParty;
        nodes[1] = party1;
        nodes[2] = party2;
        nodes[3] = party3;

        ac = new StoffelAccessControl(1, nodes);

        partyRole = ac.PARTY_ROLE();
        designatedPartyRole = ac.DESIGNATED_PARTY_ROLE();
    }

    function test_constructor_emitsInitializeEvent() public {
        address[] memory nodes = new address[](4);
        nodes[0] = designatedParty;
        nodes[1] = party1;
        nodes[2] = party2;
        nodes[3] = party3;

        vm.expectEmit();
        emit StoffelAccessControl.InitializeStoffelAccessControl(4, 1, address(this));
        new StoffelAccessControl(1, nodes);
    }

    function test_constructor_grantRoles() public view {
        assertTrue(ac.hasRole(partyRole, designatedParty));
        assertTrue(ac.hasRole(partyRole, party1));
        assertTrue(ac.hasRole(partyRole, party2));
        assertTrue(ac.hasRole(partyRole, party3));
        assertTrue(ac.hasRole(designatedPartyRole, designatedParty));
        assertFalse(ac.hasRole(designatedPartyRole, party1));
        assertFalse(ac.hasRole(designatedPartyRole, party2));
        assertFalse(ac.hasRole(designatedPartyRole, party3));
    }

    function test_grantRole_designatedPartyCanGrantRoles() public {
        address newParty = makeAddr("NEW_PARTY");

        vm.prank(designatedParty);

        ac.grantRole(partyRole, newParty);
        assertTrue(ac.hasRole(partyRole, newParty));

        vm.prank(designatedParty);

        ac.grantRole(designatedPartyRole, party1);
        assertTrue(ac.hasRole(designatedPartyRole, party1));
    }

    function test_grantRole_revertsIfCallerLacksDesignatedPartyRole() public {
        address newParty = makeAddr("NEW_PARTY");
        vm.prank(party1);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, party1, designatedPartyRole
            )
        );
        ac.grantRole(partyRole, newParty);
    }

    function test_grantRoleAgain() public {
        uint256 countBefore = ac.getRoleMemberCount(partyRole);
        vm.prank(designatedParty);
        ac.grantRole(partyRole, party1); // party1 already has the role
        assertEq(ac.getRoleMemberCount(partyRole), countBefore);
    }

    function test_revokeRole_designatedPartyCanRevokePartyRole() public {
        vm.prank(designatedParty);
        ac.revokeRole(partyRole, party1);
        assertFalse(ac.hasRole(partyRole, party1));
    }

    function test_revokeRole_revertsIfCallerLacksDesignatedPartyRole() public {
        vm.prank(party1);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, party1, designatedPartyRole
            )
        );
        ac.revokeRole(partyRole, party2);
    }

    function test_revokeRole_partyCountDecreases() public {
        uint256 countBefore = ac.getRoleMemberCount(partyRole);
        vm.prank(designatedParty);
        ac.revokeRole(partyRole, party1);
        assertEq(ac.getRoleMemberCount(partyRole), countBefore - 1);
    }

    function test_renounceRole_partyCanRenounceOwnRole() public {
        vm.prank(party1);
        ac.renounceRole(partyRole, party1);
        assertFalse(ac.hasRole(partyRole, party1));
    }

    function test_renounceRole_revertsIfRenouncingForAnotherAccount() public {
        vm.prank(party1);
        vm.expectRevert();
        ac.renounceRole(partyRole, party2);
    }

    function test_isParty() public {
        assertTrue(ac.isParty(party1));
        assertFalse(ac.isParty(makeAddr("STRANGER")));
    }

    function test_isDesignatedParty() public {
        assertTrue(ac.isDesignatedParty(designatedParty));
        assertFalse(ac.isDesignatedParty(party1));
        assertFalse(ac.isDesignatedParty(makeAddr("STRANGER")));
    }

    function test_supportsInterface_IStoffelAccessControl() public view {
        assertTrue(ac.supportsInterface(type(IStoffelAccessControl).interfaceId));
        assertTrue(ac.supportsInterface(type(IAccessControl).interfaceId));
        assertFalse(ac.supportsInterface(0xdeadbeef));
    }
}
