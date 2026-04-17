// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, Vm} from "forge-std/Test.sol";
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

    function test_sendOutputShares_revertsIfNotParty() public {
        vm.prank(client1);
        vm.expectRevert();
        coordinator.sendOutputShares(client1, abi.encode("share"));
    }

    function test_sendOutputShares_revertsAlreadyReceivedOutputShares() public {
        vm.prank(party1);
        coordinator.sendOutputShares(client1, abi.encode("share1"));

        vm.prank(party1);
        vm.expectRevert(
            abi.encodeWithSelector(StoffelInputManager.AlreadyReceivedOutputShares.selector, client1, party1)
        );
        coordinator.sendOutputShares(client1, abi.encode("share1_dup"));
    }

    function test_sendOutputShares_noEventBeforeThreshold() public {
        // t=1, threshold=2t+1=3; two shares must not trigger EnoughOutputShares
        vm.recordLogs();

        vm.prank(party1);
        coordinator.sendOutputShares(client1, abi.encode("share1"));
        vm.prank(party2);
        coordinator.sendOutputShares(client1, abi.encode("share2"));

        bytes32 eventSig = StoffelInputManager.EnoughOutputShares.selector;
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; i++) {
            assertTrue(logs[i].topics[0] != eventSig, "EnoughOutputShares emitted before threshold");
        }
    }

    function test_sendOutputShares_emitsEnoughOutputSharesAtThreshold() public {
        bytes memory share1 = abi.encode("share1");
        bytes memory share2 = abi.encode("share2");
        bytes memory share3 = abi.encode("share3");

        vm.prank(party1);
        coordinator.sendOutputShares(client1, share1);
        vm.prank(party2);
        coordinator.sendOutputShares(client1, share2);

        bytes[] memory expectedShares = new bytes[](3);
        expectedShares[0] = share1;
        expectedShares[1] = share2;
        expectedShares[2] = share3;

        vm.expectEmit(true, false, false, true);
        emit StoffelInputManager.EnoughOutputShares(client1, expectedShares);

        vm.prank(party3);
        coordinator.sendOutputShares(client1, share3);
    }

    function test_sendOutputShares_publicOutputAtAddressZero() public {
        bytes memory share1 = abi.encode("pub1");
        bytes memory share2 = abi.encode("pub2");
        bytes memory share3 = abi.encode("pub3");

        vm.prank(party1);
        coordinator.sendOutputShares(address(0), share1);
        vm.prank(party2);
        coordinator.sendOutputShares(address(0), share2);

        bytes[] memory expectedShares = new bytes[](3);
        expectedShares[0] = share1;
        expectedShares[1] = share2;
        expectedShares[2] = share3;

        vm.expectEmit(true, false, false, true);
        emit StoffelInputManager.EnoughOutputShares(address(0), expectedShares);

        vm.prank(party3);
        coordinator.sendOutputShares(address(0), share3);
    }

    function test_sendOutputShares_revertsWhenTooManyOutputClients() public {
        // maxOutputs = N_INPUTS + 1 = 4; fill all slots
        address[] memory outputClients = new address[](4);
        outputClients[0] = makeAddr("OUT1");
        outputClients[1] = makeAddr("OUT2");
        outputClients[2] = makeAddr("OUT3");
        outputClients[3] = address(0);

        for (uint256 i = 0; i < 4; i++) {
            vm.prank(party1);
            coordinator.sendOutputShares(outputClients[i], abi.encode("share"));
        }

        vm.prank(party1);
        vm.expectRevert(StoffelInputManager.TooManyOutputClients.selector);
        coordinator.sendOutputShares(makeAddr("OUT5"), abi.encode("share"));
    }

}
