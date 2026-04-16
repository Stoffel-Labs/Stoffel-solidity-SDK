// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";
import {FakeCoordinator} from "./FakeCoordinator.sol";
import {StoffelCoordinator} from "../src/StoffelCoordinator.sol";
import {StoffelAccessControl} from "../src/StoffelAccessControl.sol";

/// @notice Tests for StoffelCoordinator's base infrastructure: initial state, access control,
/// the enoughMpcParties guard, and the reset mechanism.
contract StoffelCoordinatorTest is Test {
    FakeCoordinator public coordinator;

    address party1 = makeAddr("PARTY1");
    address party2 = makeAddr("PARTY2");
    address party3 = makeAddr("PARTY3");

    function setUp() public {
        address[] memory nodes = new address[](4);
        nodes[0] = address(this);
        nodes[1] = party1;
        nodes[2] = party2;
        nodes[3] = party3;

        coordinator = new FakeCoordinator(keccak256("program hash"), 1, nodes, 3);
    }

    function test_initialRoundIsIdle() public view {
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Idle));
    }

    function test_deployerIsDesignatedParty() public view {
        assertTrue(coordinator.hasRole(coordinator.DESIGNATED_PARTY_ROLE(), address(this)));
    }

    function test_startPreprocessing_revertsIfNotEnoughParties() public {
        // Revoke one party so the count (3) falls below n=4
        coordinator.revokeRole(coordinator.PARTY_ROLE(), party3);
        vm.expectRevert(abi.encodeWithSelector(StoffelAccessControl.NotEnoughMPCParties.selector, 3, 4));
        coordinator.startPreprocessing();
    }

    function test_resetCoordinator() public {
        address client1 = makeAddr("CLIENT1");
        address client2 = makeAddr("CLIENT2");

        // Reserve indices and submit inputs so the reset has state to clear
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);
        vm.prank(client1);
        coordinator.submitMaskedInput(11111, 0);
        vm.prank(client2);
        coordinator.submitMaskedInput(22222, 1);

        _runFullCycle();

        uint256 newTimestamp = block.timestamp + 1000;
        uint256 newBlock = block.number + 50;
        vm.warp(newTimestamp);
        vm.roll(newBlock);

        vm.expectEmit();
        emit StoffelCoordinator.CoordinatorInitialized(address(coordinator), newTimestamp, newBlock, address(this));
        coordinator.resetCoordinator();

        // Round reset
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Idle));

        // All input masks freed
        assertEq(coordinator.availableInputMasks(), 3);

        // Base nonce incremented by nTotalIndices (3) to enforce uniqueness across runs
        assertEq(coordinator.baseNonce(), 3);

        // Creation time and block refreshed
        assertEq(coordinator.creationTime(), newTimestamp);
        assertEq(coordinator.creationBlock(), newBlock);

        // Previously reserved indices are available again
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);
        assertEq(coordinator.availableInputMasks(), 1);
    }

    function test_resetCoordinator_revertsIfNotDesignatedParty() public {
        vm.prank(party1);
        vm.expectRevert();
        coordinator.resetCoordinator();
    }

    function test_resetCoordinator_allowsAnotherFullRun() public {
        _runFullCycle();
        coordinator.resetCoordinator();

        coordinator.startPreprocessing();
        coordinator.reserveInputMasks();
        coordinator.collectInputs();
        coordinator.startMpc();
        coordinator.sendOutputs();
        coordinator.finalize();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.ProgramFinished));
    }

    function test_revokeRole_emitsEventAndComputationCompletesAfterRevocation() public {
        coordinator.startPreprocessing();

        // Revoke party1 during Preprocessing — not Idle
        vm.expectEmit(true, true, true, true);
        emit IAccessControl.RoleRevoked(coordinator.PARTY_ROLE(), party1, address(this));
        coordinator.revokeRole(coordinator.PARTY_ROLE(), party1);

        assertFalse(coordinator.hasRole(coordinator.PARTY_ROLE(), party1));

        // Computation continues to completion despite the revocation
        coordinator.reserveInputMasks();
        coordinator.collectInputs();
        coordinator.startMpc();
        coordinator.sendOutputs();
        coordinator.finalize();

        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.ProgramFinished));
    }

    /// @notice Runs all six lifecycle phases in order against the coordinator under test
    function _runFullCycle() internal {
        coordinator.startPreprocessing();
        coordinator.reserveInputMasks();
        coordinator.collectInputs();
        coordinator.startMpc();
        coordinator.sendOutputs();
        coordinator.finalize();
    }
}
