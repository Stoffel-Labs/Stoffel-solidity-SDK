// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "./StoffelAccessControl.sol";
import "./StoffelInputManager.sol";
import "./interfaces/IStoffelAccessControl.sol";
import "./interfaces/IStoffelInputManager.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/// @title StoffelCoordinator
/// @author Stoffel Labs
/// @notice Abstract contract for coordinating Multi-Party Computation (MPC) workflows on-chain
/// @dev Implements a 7-phase round-based state machine for MPC coordination.
///      Inherits access control for party management and input management for client submissions.
///      Concrete implementations must override the four abstract lifecycle methods.
abstract contract StoffelCoordinator is StoffelAccessControl, StoffelInputManager, Ownable {
    /// @notice Enum representing the phases of the MPC computation lifecycle
    /// @dev Rounds progress sequentially from PreprocessingRound to ClientOutputCollectionRound
    enum Round {
        Idle,
        /// @notice Initial phase where MPC nodes generate preprocessing material (input masks)
        Preprocessing,
        /// @notice Phase where clients can reserve input mask indices
        InputMaskReservation,
        /// @notice Phase where clients submit their masked inputs
        InputCollection,
        /// @notice Phase where off-chain MPC computation is executed by nodes
        MPC,
        /// @notice Final phase where clients can retrieve their computation outputs
        Output
    }
    /// @notice Emitted when the coordinator contract is initialized
    /// @param coordinator The address of this coordinator contract
    /// @param timeofInitialization The block timestamp when initialization occurred
    /// @param designatedParty The address granted the designated party role
    event CoordinatorInitialized(address coordinator, uint256 timeofInitialization, uint256 creationBlock, address designatedParty);

    /// @notice Emitted when the preprocessing round is executed
    /// @param designatedParty The address that executed the preprocessing
    /// @param timeOfExecution The block timestamp when preprocessing was executed
    event PreprocessingStarted(address designatedParty, uint256 timeOfExecution);

    /// @notice Emitted when input mask reservation phase begins
    /// @param executor The address that triggered the reservation phase
    /// @param timeOfExecution The block timestamp when the phase began
    event InputMaskReservationStarted(address executor, uint256 timeOfExecution);

    event InputCollectionStarted(address executor, uint256 timeOfExecution);

    /// @notice Emitted when the MPC task execution is initiated
    /// @param executor The address that initiated the execution
    /// @param timeOfExecution The block timestamp when execution was initiated
    event MPCStarted(address executor, uint256 timeOfExecution);

    event OutputSendingStarted(address executor, uint256 timeOfExecution);

    event ExecutionDone(address executor, uint256 timeOfExecution);

    error NotAtRound(Round required, Round current);

    /// @notice Hash of the StoffelLang program to be executed by MPC nodes
    /// @dev Used to verify the correct program is being run off-chain
    bytes32 internal _stoffelProgramHash;

    /// @notice Timestamp when the coordinator was created
    /// @dev Used for time-based round transitions
    uint256 public creationTime;

    uint256 public creationBlock;

    /// @notice Current round in the MPC lifecycle
    /// @dev Progresses through the Round enum values
    Round public round;

    /// @notice Modifier that enforces the contract is in a specific round
    /// @param _round The required round for the function to execute
    modifier atRound(Round _round) {
        _atRound(_round);
        _;
    }

    /// @notice Modifier that advances to the next round after function execution
    modifier nextRound() {
        _;
        _nextRound();
    }

    /// @notice Modifier that jumps to a specific round after function execution
    /// @param _round The target round to transition to
    modifier goToRound(Round _round) {
        _;
        _goToRound(_round);
    }

    /// @notice Internal function to verify the contract is in the expected round
    /// @param _round The expected current round
    /// @dev Reverts if the current round doesn't match the expected round
    function _atRound(Round _round) internal view {
        if (round != _round) {
	    revert NotAtRound(_round, round);
	}
    }

    /// @notice Internal function to advance to the next round
    /// @dev Increments the round enum value by 1
    function _nextRound() internal {
        round = Round(uint256(round) + 1);
    }

    /// @notice Internal function to jump to a specific round
    /// @param _round The target round to transition to
    /// @dev Use with caution as it bypasses sequential round progression
    function _goToRound(Round _round) internal {
        round = _round;
    }

    /// @notice Internal function for time-based round transitions
    /// @param transitionRound The round that should trigger the transition
    /// @param whenToTransition Duration in seconds after creation to trigger transition
    /// @dev Only transitions if currently in transitionRound and enough time has passed
    function _timedRoundTransition(Round transitionRound, uint256 whenToTransition) internal {
        if (round == transitionRound && (block.timestamp >= creationTime + whenToTransition)) {
            _nextRound();
        }
    }

    /// @notice Internal function for time-based transitions to a specific round
    /// @param transitionRound The round that should trigger the transition
    /// @param gotoRound The target round to transition to
    /// @param whenToTransition Absolute timestamp when transition should occur
    /// @dev Only transitions if currently in transitionRound and time threshold is met
    function _timedRoundTransitionGoTo(Round transitionRound, Round gotoRound, uint256 whenToTransition) internal {
        if (round == transitionRound && block.timestamp >= whenToTransition) {
            _goToRound(gotoRound);
        }
    }

    /// @notice Modifier for automatic time-based round advancement
    /// @param transitionRound The round that triggers the transition check
    /// @param whenToTransition Duration in seconds after creation to trigger transition
    modifier timedRoundTransition(Round transitionRound, uint256 whenToTransition) {
        _;
        _timedRoundTransition(transitionRound, whenToTransition);
    }

    /// @notice Modifier for automatic time-based transition to a specific round
    /// @param transitionRound The round that triggers the transition check
    /// @param gotoRound The target round to transition to
    /// @param whenToTransition Absolute timestamp when transition should occur
    modifier timedRoundTransitionGoto(Round transitionRound, Round gotoRound, uint256 whenToTransition) {
        _;
        _timedRoundTransitionGoTo(transitionRound, gotoRound, whenToTransition);
    }

    constructor (bytes32 stoffelProgramHash, uint256 t, address[] memory initialMPCNodes, uint256 nInputs) StoffelAccessControl(t, initialMPCNodes) StoffelInputManager(nInputs, t) Ownable(msg.sender) {
        _resetCoordinator(stoffelProgramHash, initialMPCNodes[0]);
    }

    /// @notice Initializes the MPC coordinator with parties and program configuration
    /// @param stoffelProgramHash Hash of the StoffelLang program to execute
    /// @param designatedParty The address assigned the designated party role
    /// @dev Sets up access control, stores program hash, and emits initialization event
    function _resetCoordinator(
        bytes32 stoffelProgramHash,
        address designatedParty
    ) internal {
        _stoffelProgramHash = stoffelProgramHash;

        creationTime = block.timestamp;
        creationBlock = block.number;
	round = Round.Idle;

        emit CoordinatorInitialized(address(this), creationTime, creationBlock, designatedParty);
    }

    function resetCoordinator(
        bytes32 stoffelProgramHash,
        uint256 t,
        address[] memory initialMPCNodes,
	uint256 nInputs
    ) external onlyRole(DESIGNATED_PARTY_ROLE) {
	super._resetAccessControl(t, initialMPCNodes);
	super._resetInputManager(nInputs, t);
	_resetCoordinator(stoffelProgramHash, initialMPCNodes[0]);
    }

    /// @notice Initiates the preprocessing phase of the MPC computation
    /// @dev Called by the designated party to start generating input masks.
    ///      Implementations should initialize the input mask buffer and transition
    ///      to the next round. Should use the atRound modifier for round enforcement.
    function startPreprocessing() external virtual;

    /// @notice Transitions to the input gathering phase
    /// @dev Called after preprocessing to allow clients to reserve input mask indices.
    ///      Clients can then request their masks off-chain from MPC nodes and submit
    ///      their masked inputs to the contract.
    function reserveInputMasks() external virtual;

    function collectInputs() external virtual;

    /// @notice Initiates the off-chain MPC computation
    /// @dev Called when sufficient inputs have been collected according to application logic.
    ///      MPC nodes listen for the associated event to begin computation.
    ///      Should use the atRound modifier for round enforcement.
    function startMPC() external virtual;

    /// @notice Publishes the results of the MPC computation
    /// @dev Called after MPC nodes complete the off-chain computation.
    ///      Public outputs are stored on-chain while private shares are sent
    ///      directly to clients. The Outputs struct tracks share delivery status.
    function sendOutputs() external virtual;

    function finalize() external virtual;
}
