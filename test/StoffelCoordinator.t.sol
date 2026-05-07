// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";
import {FakeCoordinator} from "./FakeCoordinator.sol";
import {StoffelCoordinator} from "../src/StoffelCoordinator.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";
import {StoffelInputManager} from "../src/StoffelInputManager.sol";

/// @notice Tests for StoffelCoordinator's base infrastructure: initial state, access control,
/// the enoughMpcParties guard, and the reset mechanism.
contract StoffelCoordinatorTest is Test {
    FakeCoordinator public coordinator;

    address party1 = makeAddr("PARTY1");
    address party2 = makeAddr("PARTY2");
    address party3 = makeAddr("PARTY3");
    address client1 = makeAddr("CLIENT1");
    address client2 = makeAddr("CLIENT2");

    function setUp() public {
        address[] memory nodes = new address[](4);
        nodes[0] = address(this);
        nodes[1] = party1;
        nodes[2] = party2;
        nodes[3] = party3;

        address[] memory outClients = new address[](2);
        outClients[0] = client1;
        outClients[1] = client2;

        coordinator = new FakeCoordinator(keccak256("program hash"), 1, nodes, 3, outClients);
    }

    function test_initialRoundIsIdle() public view {
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Idle));
    }

    function test_deployerIsDesignatedParty() public view {
        assertTrue(coordinator.hasRole(coordinator.DESIGNATED_PARTY_ROLE(), address(this)));
    }

    function test_startPreprocessing_revertsIfNotEnoughParties() public {
        // Run to ProgramFinished so role changes are allowed, then revoke one party
        _advanceToFinalize();
        coordinator.finalize();
        coordinator.revokeRole(coordinator.PARTY_ROLE(), party3);
        coordinator.resetCoordinator();

        vm.expectRevert(abi.encodeWithSelector(StoffelAccessControl.NotEnoughMPCParties.selector, 3, 4));
        coordinator.startPreprocessing();
    }

    function test_resetCoordinator() public {
        // Reserve indices and submit inputs so the reset has state to clear
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);
        vm.prank(client1);
        coordinator.submitMaskedInput(hex"2B67", 0);
        vm.prank(client2);
        coordinator.submitMaskedInput(hex"56CE", 1);

        _advanceToFinalize();

        // Send output shares during OutputDistribution
        vm.prank(party1);
        coordinator.sendOutputShares(client1, abi.encode("share1"));
        vm.prank(party2);
        coordinator.sendOutputShares(client1, abi.encode("share2"));

        coordinator.finalize();

        uint256 newTimestamp = block.timestamp + 1000;
        uint256 newBlock = block.number + 50;
        vm.warp(newTimestamp);
        vm.roll(newBlock);

        vm.expectEmit();
        emit StoffelCoordinator.CoordinatorReset(address(coordinator), newBlock);
        coordinator.resetCoordinator();

        // Round reset
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Idle));

        // All input masks freed
        assertEq(coordinator.availableInputMasks(), 3);

        // Base nonce incremented by nTotalIndices (3) to enforce uniqueness across runs
        assertEq(coordinator.baseNonce(), 3);

        // lastResetBlock updated; creationTime and creationBlock stay fixed from construction
        assertEq(coordinator.lastResetBlock(), newBlock);

        // Output client roles persist across reset — sendOutputShares works with fresh state
        assertTrue(coordinator.hasRole(coordinator.OUTPUT_CLIENT_ROLE(), client1));
        _advanceToFinalize();

        vm.prank(party1);
        coordinator.sendOutputShares(client1, abi.encode("share_after_reset"));

        // Previously reserved indices are available again
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);
        assertEq(coordinator.availableInputMasks(), 1);
    }

    function test_resetCoordinator_updatesLastResetBlockAndEmitsEvent() public {
        uint256 initialCreationBlock = coordinator.creationBlock();

        _advanceToFinalize();
        coordinator.finalize();

        uint256 newBlock = block.number + 50;
        vm.roll(newBlock);

        vm.expectEmit(true, true, true, true);
        emit StoffelCoordinator.CoordinatorReset(address(coordinator), newBlock);
        coordinator.resetCoordinator();

        assertEq(coordinator.lastResetBlock(), newBlock);
        assertEq(coordinator.creationBlock(), initialCreationBlock);
    }

    function test_resetCoordinator_revertsIfNotDesignatedParty() public {
        vm.prank(party1);
        vm.expectRevert();
        coordinator.resetCoordinator();
    }

    function test_resetCoordinator_allowsAnotherFullRun() public {
        _advanceToFinalize();
        coordinator.finalize();
        coordinator.resetCoordinator();

        _advanceToFinalize();
        coordinator.finalize();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.ProgramFinished));
    }

    function test_revokeRole_revertsIfProgramIsExecuting() public {
        bytes32 partyRole = coordinator.PARTY_ROLE();
        coordinator.startPreprocessing();

        vm.expectRevert(
            abi.encodeWithSelector(StoffelCoordinator.RoleChangeNotAllowed.selector, StoffelCoordinator.Round.Preprocessing)
        );
        coordinator.revokeRole(partyRole, party1);
    }

    function test_revokeRole_succeedsWhenProgramFinished() public {
        bytes32 partyRole = coordinator.PARTY_ROLE();
        _advanceToFinalize();
        coordinator.finalize();

        vm.expectEmit(true, true, true, true);
        emit IAccessControl.RoleRevoked(partyRole, party1, address(this));
        coordinator.revokeRole(partyRole, party1);

        assertFalse(coordinator.hasRole(partyRole, party1));
    }

    /// @notice Advances the coordinator through all rounds up to OutputDistribution, ready to finalize
    function _advanceToFinalize() internal {
        coordinator.startPreprocessing();
        coordinator.reserveInputMasks();
        coordinator.collectInputs();
        coordinator.startMpc();
        coordinator.sendOutputs();
    }
}
