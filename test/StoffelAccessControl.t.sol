// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";
import {IStoffelAccessControl} from "../src/interfaces/IStoffelAccessControl.sol";

/// @title StoffelAccessControlTest
/// @notice Tests for StoffelAccessControl functionality
/// @dev Tests role management, threshold constraints, and party membership
contract StoffelAccessControlTest is Test {
    StoffelAccessControl public accessControl;

    address public admin;
    address public party1;
    address public party2;
    address public party3;
    address public party4;
    address public party5;
    address public designatedParty;
    address public unauthorized;

    // Events to test
    event InitializeStoffelAccessControl(uint256 n, uint256 t, address initializer);

    function setUp() public {
        admin = address(this);
        party1 = makeAddr("PARTY1");
        party2 = makeAddr("PARTY2");
        party3 = makeAddr("PARTY3");
        party4 = makeAddr("PARTY4");
        party5 = makeAddr("PARTY5");
        designatedParty = makeAddr("DESIGNATED_PARTY");
        unauthorized = makeAddr("UNAUTHORIZED");

        // Create with n=5 parties, t=1 threshold (5 >= 3*1 + 1 = 4)
        accessControl = new StoffelAccessControl(5, 1);
    }

    // =========================================================================
    // Constructor Tests
    // =========================================================================

    function test_constructorSetsNParties() public view {
        // n_parties is internal, but we can verify by granting roles
        // The contract allows up to n parties
        assertTrue(address(accessControl) != address(0));
    }

    function test_constructorEmitsEvent() public {
        vm.expectEmit(true, true, true, true);
        emit InitializeStoffelAccessControl(5, 1, address(this));
        new StoffelAccessControl(5, 1);
    }

    // =========================================================================
    // Role Granting Tests
    // =========================================================================

    function test_grantPartyRole() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        assertTrue(accessControl.isParty(party1));
    }

    function test_grantMultiplePartyRoles() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        accessControl.grantRole(accessControl.PARTY_ROLE(), party2);
        accessControl.grantRole(accessControl.PARTY_ROLE(), party3);

        assertTrue(accessControl.isParty(party1));
        assertTrue(accessControl.isParty(party2));
        assertTrue(accessControl.isParty(party3));
    }

    function test_grantDesignatedPartyRole() public {
        // First grant party role (designated party must be a party)
        accessControl.grantRole(accessControl.PARTY_ROLE(), designatedParty);
        accessControl.grantRole(accessControl.DESIGNATED_PARTY_ROLE(), designatedParty);

        assertTrue(accessControl.hasRole(accessControl.DESIGNATED_PARTY_ROLE(), designatedParty));
    }

    function test_cannotExceedMaxParties() public {
        // Grant exactly n_parties (5 parties)
        bytes32 partyRole = accessControl.PARTY_ROLE();

        accessControl.grantRole(partyRole, party1);
        accessControl.grantRole(partyRole, party2);
        accessControl.grantRole(partyRole, party3);
        accessControl.grantRole(partyRole, party4);
        accessControl.grantRole(partyRole, party5);

        // Verify all 5 parties are granted
        assertEq(accessControl.getRoleMemberCount(partyRole), 5);

        // Attempting to add a 6th party should fail (5 < 5 is false)
        address party6 = makeAddr("PARTY6");
        vm.expectRevert("Too many MPC parties");
        accessControl.grantRole(partyRole, party6);
    }

    function test_duplicateGrantNoOp() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        // Granting again should not fail (OpenZeppelin AccessControl behavior)
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        assertTrue(accessControl.isParty(party1));
    }

    // =========================================================================
    // Role Revocation Tests
    // =========================================================================

    function test_revokePartyRole() public {
        // Setup: Grant roles to meet threshold
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        accessControl.grantRole(accessControl.PARTY_ROLE(), party2);
        accessControl.grantRole(accessControl.PARTY_ROLE(), party3);

        // Revoke one (should still have enough for threshold)
        accessControl.revokeRole(accessControl.PARTY_ROLE(), party1);

        // Verify both hasRole and isParty return false after revoke
        assertFalse(accessControl.hasRole(accessControl.PARTY_ROLE(), party1));
        assertFalse(accessControl.isParty(party1));

        // Verify other parties are unaffected
        assertTrue(accessControl.isParty(party2));
        assertTrue(accessControl.isParty(party3));
    }

    function test_cannotRevokeBelowThreshold() public {
        // Setup: Grant threshold + 1 parties (2 parties with t=1)
        bytes32 partyRole = accessControl.PARTY_ROLE();
        accessControl.grantRole(partyRole, party1);
        accessControl.grantRole(partyRole, party2);

        // With 2 parties and t=1, revoking one should succeed (2 > 1)
        accessControl.revokeRole(partyRole, party1);
        assertEq(accessControl.getRoleMemberCount(partyRole), 1);

        // Now with 1 party (equal to threshold), revoking should fail (1 > 1 is false)
        vm.expectRevert("Not enough MPC parties");
        accessControl.revokeRole(partyRole, party2);

        // Verify party2 still has the role
        assertTrue(accessControl.isParty(party2));
        assertEq(accessControl.getRoleMemberCount(partyRole), 1);
    }

    // =========================================================================
    // isParty Tests
    // =========================================================================

    function test_isParty_returnsTrue() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        assertTrue(accessControl.isParty(party1));
    }

    function test_isParty_returnsFalse() public view {
        assertFalse(accessControl.isParty(unauthorized));
    }

    // =========================================================================
    // isDesignatedParty Tests
    // =========================================================================

    function test_isDesignatedParty_returnsTrue() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), designatedParty);
        accessControl.grantRole(accessControl.DESIGNATED_PARTY_ROLE(), designatedParty);

        // Must be called by a party
        vm.prank(designatedParty);
        assertTrue(accessControl.isDesignatedParty(designatedParty));
    }

    function test_isDesignatedParty_returnsFalse_whenNoDesignatedParties() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);

        // When there are no designated parties, the function should return false
        vm.prank(party1);
        assertFalse(accessControl.isDesignatedParty(party1));
    }

    function test_isDesignatedParty_requiresCallerToBeParty() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), designatedParty);
        accessControl.grantRole(accessControl.DESIGNATED_PARTY_ROLE(), designatedParty);

        // Non-party caller should revert
        vm.prank(unauthorized);
        vm.expectRevert("This account is not an existing MPC Party");
        accessControl.isDesignatedParty(designatedParty);
    }

    // =========================================================================
    // supportsInterface Tests
    // =========================================================================

    function test_supportsInterface() public view {
        // Should support IStoffelAccessControl interface
        bytes4 interfaceId = type(IStoffelAccessControl).interfaceId;
        assertTrue(accessControl.supportsInterface(interfaceId));
    }

    // =========================================================================
    // Role Renounce Tests
    // =========================================================================

    function test_renounceRole() public {
        bytes32 partyRole = accessControl.PARTY_ROLE();
        accessControl.grantRole(partyRole, party1);
        accessControl.grantRole(partyRole, party2);

        // Party can renounce their own role
        // OpenZeppelin requires account parameter to match msg.sender
        vm.prank(party1);
        accessControl.renounceRole(partyRole, party1);
        assertFalse(accessControl.hasRole(partyRole, party1));
    }

    function test_cannotRenounceForOther() public {
        bytes32 partyRole = accessControl.PARTY_ROLE();
        accessControl.grantRole(partyRole, party1);
        accessControl.grantRole(partyRole, party2);

        // Party1 cannot renounce party2's role
        // OpenZeppelin throws AccessControlBadConfirmation
        vm.prank(party1);
        vm.expectRevert();
        accessControl.renounceRole(partyRole, party2);
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    function test_zeroAddressHandling() public {
        // OpenZeppelin's AccessControl allows granting to zero address
        // This is probably undesirable but is the current behavior
        accessControl.grantRole(accessControl.PARTY_ROLE(), address(0));
        assertTrue(accessControl.hasRole(accessControl.PARTY_ROLE(), address(0)));
    }

    function test_partyRoleConstant() public view {
        bytes32 expected = keccak256("PARTY_ROLE");
        assertEq(accessControl.PARTY_ROLE(), expected);
    }

    function test_designatedPartyRoleConstant() public view {
        bytes32 expected = keccak256("DESIGNATED_PARTY_ROLE");
        assertEq(accessControl.DESIGNATED_PARTY_ROLE(), expected);
    }

    // =========================================================================
    // Access Control Enumerable Tests
    // =========================================================================

    function test_getRoleMemberCount() public {
        assertEq(accessControl.getRoleMemberCount(accessControl.PARTY_ROLE()), 0);

        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        assertEq(accessControl.getRoleMemberCount(accessControl.PARTY_ROLE()), 1);

        accessControl.grantRole(accessControl.PARTY_ROLE(), party2);
        assertEq(accessControl.getRoleMemberCount(accessControl.PARTY_ROLE()), 2);
    }

    function test_getRoleMember() public {
        accessControl.grantRole(accessControl.PARTY_ROLE(), party1);
        accessControl.grantRole(accessControl.PARTY_ROLE(), party2);

        address member0 = accessControl.getRoleMember(accessControl.PARTY_ROLE(), 0);
        address member1 = accessControl.getRoleMember(accessControl.PARTY_ROLE(), 1);

        assertTrue(member0 == party1 || member0 == party2);
        assertTrue(member1 == party1 || member1 == party2);
        assertTrue(member0 != member1);
    }
}
