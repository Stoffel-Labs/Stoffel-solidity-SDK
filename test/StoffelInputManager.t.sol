// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {FakeCoordinator} from "./FakeCoordinator.sol";
import {StoffelInputManager} from "../src/StoffelInputManager.sol";

/// @notice Tests for StoffelInputManager: index reservation, masked input submission, and nonce tracking.
contract StoffelInputManagerTest is Test {
    FakeCoordinator public coordinator;

    address party1 = makeAddr("PARTY1");
    address party2 = makeAddr("PARTY2");
    address party3 = makeAddr("PARTY3");
    address client1 = makeAddr("CLIENT1");
    address client2 = makeAddr("CLIENT2");
    address client3 = makeAddr("CLIENT3");

    uint256 constant N_INPUTS = 3;

    function setUp() public {
        address[] memory nodes = new address[](4);
        nodes[0] = address(this);
        nodes[1] = party1;
        nodes[2] = party2;
        nodes[3] = party3;

        coordinator = new FakeCoordinator(keccak256("program hash"), 1, nodes, N_INPUTS);
    }

    function test_availableInputMasksInitial() public view {
        assertEq(coordinator.availableInputMasks(), N_INPUTS);
    }

    function test_reserveMaskIndex() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        assertEq(coordinator.availableInputMasks(), N_INPUTS - 1);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);
        vm.prank(client3);
        coordinator.reserveMaskIndex(2);
        assertEq(coordinator.availableInputMasks(), 0);
    }

    function test_reserveMaskIndex_grantsInputClientRole() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        assertTrue(coordinator.hasRole(coordinator.INPUT_CLIENT_ROLE(), client1));
    }

    function test_reserveMaskIndex_revertsOutOfBounds() public {
        vm.prank(client1);
        vm.expectRevert(abi.encodeWithSelector(StoffelInputManager.IndexOutOfBounds.selector, client1, N_INPUTS));
        coordinator.reserveMaskIndex(N_INPUTS);
    }

    function test_reserveMaskIndex_revertsIndexAlreadyReserved() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);

        vm.prank(client2);
        vm.expectRevert(abi.encodeWithSelector(StoffelInputManager.IndexAlreadyReserved.selector, 0, client2, client1));
        coordinator.reserveMaskIndex(0);
    }

    function test_reserveMaskIndex_revertsClientAlreadyReservedIndex() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);

        vm.prank(client1);
        vm.expectRevert(abi.encodeWithSelector(StoffelInputManager.ClientAlreadyReservedIndex.selector, client1, 0));
        coordinator.reserveMaskIndex(1);
    }

    function test_submitMaskedInput_multipleClients() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);
        vm.prank(client3);
        coordinator.reserveMaskIndex(2);

        vm.prank(client1);
        coordinator.submitMaskedInput(11111, 0);
        vm.prank(client2);
        coordinator.submitMaskedInput(22222, 1);
        vm.prank(client3);
        coordinator.submitMaskedInput(33333, 2);
    }

    function test_submitMaskedInput_revertsWithoutReservation() public {
        vm.prank(client1);
        vm.expectRevert();
        coordinator.submitMaskedInput(12345, 0);
    }

    function test_submitMaskedInput_revertsZeroMaskedInput() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);

        vm.prank(client1);
        vm.expectRevert(abi.encodeWithSelector(StoffelInputManager.ZeroMaskedInput.selector, client1));
        coordinator.submitMaskedInput(0, 0);
    }

    function test_submitMaskedInput_revertsIndexNotReservedByCaller() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);
        vm.prank(client2);
        coordinator.reserveMaskIndex(1);

        // client2 tries to submit using client1's index
        vm.prank(client2);
        vm.expectRevert(abi.encodeWithSelector(StoffelInputManager.IndexNotReserved.selector, client2, 0));
        coordinator.submitMaskedInput(12345, 0);
    }

    function test_submitMaskedInput_revertsAlreadySubmitted() public {
        vm.prank(client1);
        coordinator.reserveMaskIndex(0);

        vm.prank(client1);
        coordinator.submitMaskedInput(12345, 0);

        vm.prank(client1);
        vm.expectRevert(abi.encodeWithSelector(StoffelInputManager.AlreadySubmittedInputs.selector, client1));
        coordinator.submitMaskedInput(31415, 0);
    }

    function test_baseNonceInitiallyZero() public view {
        assertEq(coordinator.baseNonce(), 0);
    }

    function test_baseNonceIncreasesEachReset() public {
        for (uint256 i = 0; i < 3; i++) {
            coordinator.startPreprocessing();
            coordinator.reserveInputMasks();
            coordinator.collectInputs();
            coordinator.startMpc();
            coordinator.sendOutputs();
            coordinator.finalize();
            coordinator.resetCoordinator();
        }
        assertEq(coordinator.baseNonce(), N_INPUTS * 3);
    }
}
