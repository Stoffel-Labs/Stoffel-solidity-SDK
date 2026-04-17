// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {FakeCoordinator} from "./FakeCoordinator.sol";
import {StoffelCoordinator} from "../src/StoffelCoordinator.sol";

/// Tests for FakeCoordinator's concrete implementations of the abstract lifecycle methods.
contract FakeCoordinatorTest is Test {
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

        coordinator = new FakeCoordinator(keccak256("program hash"), 1, nodes, 3, new address[](0));
    }

    // ── startPreprocessing ───────────────────────────────────────────────────

    function test_startPreprocessing() public {
        coordinator.startPreprocessing();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Preprocessing));
    }

    function test_startPreprocessing_revertsIfNotDesignatedParty() public {
        vm.prank(party1);
        vm.expectRevert();
        coordinator.startPreprocessing();
    }

    function test_startPreprocessing_revertsIfNotIdle() public {
        coordinator.startPreprocessing();
        vm.expectRevert(
            abi.encodeWithSelector(
                StoffelCoordinator.NotAtRound.selector,
                StoffelCoordinator.Round.Idle,
                StoffelCoordinator.Round.Preprocessing
            )
        );
        coordinator.startPreprocessing();
    }

    // ── reserveInputMasks ────────────────────────────────────────────────────

    function test_reserveInputMasks() public {
        coordinator.startPreprocessing();
        coordinator.reserveInputMasks();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.InputMaskReservation));
    }

    function test_reserveInputMasks_revertsIfNotPreprocessing() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                StoffelCoordinator.NotAtRound.selector,
                StoffelCoordinator.Round.Preprocessing,
                StoffelCoordinator.Round.Idle
            )
        );
        coordinator.reserveInputMasks();
    }

    function test_reserveInputMasks_revertsIfNotDesignatedParty() public {
        coordinator.startPreprocessing();
        vm.prank(party1);
        vm.expectRevert();
        coordinator.reserveInputMasks();
    }

    // ── collectInputs ────────────────────────────────────────────────────────

    function test_collectInputs() public {
        coordinator.startPreprocessing();
        coordinator.reserveInputMasks();
        coordinator.collectInputs();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.InputCollection));
    }

    function test_collectInputs_revertsIfNotInputMaskReservation() public {
        coordinator.startPreprocessing();
        vm.expectRevert(
            abi.encodeWithSelector(
                StoffelCoordinator.NotAtRound.selector,
                StoffelCoordinator.Round.InputMaskReservation,
                StoffelCoordinator.Round.Preprocessing
            )
        );
        coordinator.collectInputs();
    }

    // ── startMpc ─────────────────────────────────────────────────────────────

    function test_startMpc() public {
        _advanceTo(StoffelCoordinator.Round.InputCollection);
        coordinator.startMpc();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.MPCExecution));
    }

    function test_startMpc_revertsIfNotInputCollection() public {
        _advanceTo(StoffelCoordinator.Round.Preprocessing);
        vm.expectRevert();
        coordinator.startMpc();
    }

    function test_startMpc_revertsIfNotDesignatedParty() public {
        _advanceTo(StoffelCoordinator.Round.InputCollection);
        vm.prank(party1);
        vm.expectRevert();
        coordinator.startMpc();
    }

    // ── sendOutputs ──────────────────────────────────────────────────────────

    function test_sendOutputs() public {
        _advanceTo(StoffelCoordinator.Round.MPCExecution);
        coordinator.sendOutputs();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.OutputDistribution));
    }

    function test_sendOutputs_revertsIfNotMpcExecution() public {
        _advanceTo(StoffelCoordinator.Round.InputCollection);
        vm.expectRevert();
        coordinator.sendOutputs();
    }

    // ── finalize ─────────────────────────────────────────────────────────────

    function test_finalize() public {
        _advanceTo(StoffelCoordinator.Round.OutputDistribution);
        coordinator.finalize();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.ProgramFinished));
    }

    function test_finalize_revertsIfNotOutputDistribution() public {
        _advanceTo(StoffelCoordinator.Round.MPCExecution);
        vm.expectRevert();
        coordinator.finalize();
    }

    // ── full progression ─────────────────────────────────────────────────────

    function test_fullRoundProgression() public {
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Idle));
        coordinator.startPreprocessing();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.Preprocessing));
        coordinator.reserveInputMasks();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.InputMaskReservation));
        coordinator.collectInputs();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.InputCollection));
        coordinator.startMpc();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.MPCExecution));
        coordinator.sendOutputs();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.OutputDistribution));
        coordinator.finalize();
        assertEq(uint256(coordinator.round()), uint256(StoffelCoordinator.Round.ProgramFinished));
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    function _advanceTo(StoffelCoordinator.Round target) internal {
        StoffelCoordinator.Round[] memory sequence = new StoffelCoordinator.Round[](6);
        sequence[0] = StoffelCoordinator.Round.Preprocessing;
        sequence[1] = StoffelCoordinator.Round.InputMaskReservation;
        sequence[2] = StoffelCoordinator.Round.InputCollection;
        sequence[3] = StoffelCoordinator.Round.MPCExecution;
        sequence[4] = StoffelCoordinator.Round.OutputDistribution;
        sequence[5] = StoffelCoordinator.Round.ProgramFinished;

        for (uint256 i = 0; i < sequence.length; i++) {
            if (coordinator.round() == target) break;
            StoffelCoordinator.Round next = sequence[i];
            if (next == StoffelCoordinator.Round.Preprocessing) coordinator.startPreprocessing();
            else if (next == StoffelCoordinator.Round.InputMaskReservation) coordinator.reserveInputMasks();
            else if (next == StoffelCoordinator.Round.InputCollection) coordinator.collectInputs();
            else if (next == StoffelCoordinator.Round.MPCExecution) coordinator.startMpc();
            else if (next == StoffelCoordinator.Round.OutputDistribution) coordinator.sendOutputs();
            else if (next == StoffelCoordinator.Round.ProgramFinished) coordinator.finalize();
        }
    }
}
