// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {StoffelCoordinator} from "../../src/StoffelCoordinator.sol";

/// @title MockCoordinator
/// @notice Concrete implementation of StoffelCoordinator for testing
/// @dev Implements all 4 abstract methods with minimal logic to enable testing.
///
/// KNOWN BUGS IN PARENT CONTRACT (StoffelAccessControl):
/// 1. isDesignatedParty() has a loop bug (i <= n instead of i < n) causing array out-of-bounds
/// 2. The is_party mapping is not cleared when PARTY_ROLE is revoked
/// 3. _grantRole uses <= instead of < for party count check (allows n+1 parties)
///
/// Due to bug #1, the onlyDesignatedParty modifier will always panic when there are
/// designated parties. Tests document this behavior and use alternative verification methods.
contract MockCoordinator is StoffelCoordinator {
    uint256 public inputBufferSize;
    uint256 public inputCount;
    bytes public storedOutputs;

    event PreprocessingStarted(address indexed executor, uint256 timestamp);
    event InputGatheringStarted(address indexed executor, uint256 timestamp);
    event MPCComputationInitiated(address indexed executor, uint256 inputCount, uint256 timestamp);
    event OutputsPublished(address indexed executor, bytes outputs, uint256 timestamp);

    constructor(
        bytes32 stoffelProgramHash,
        uint256 n,
        uint256 t,
        address designatedParty,
        address[] memory initialMPCNodes
    ) StoffelCoordinator(stoffelProgramHash, n, t, designatedParty, initialMPCNodes) {
        // Designated party must also have PARTY_ROLE to pass onlyDesignatedParty modifier
        _grantRole(PARTY_ROLE, designatedParty);
        // The contract itself needs PARTY_ROLE because isDesignatedParty() is called via this.
        // which makes msg.sender the contract address during the external call
        _grantRole(PARTY_ROLE, address(this));
    }

    /// @notice Starts the preprocessing phase
    /// @dev Initializes input mask buffer and advances to next round
    function startPreprocessing() external override onlyDesignatedParty atRound(Round.PreprocessingRound) {
        inputBufferSize = 10; // Default buffer size for testing
        this.initialzeInputMaskBuffer(inputBufferSize);
        emit PreprocessingStarted(msg.sender, block.timestamp);
        _nextRound();
    }

    /// @notice Starts the preprocessing phase with custom buffer size
    /// @param bufferSize Number of input mask indices to reserve
    function startPreprocessingWithSize(uint256 bufferSize)
        external
        onlyDesignatedParty
        atRound(Round.PreprocessingRound)
    {
        inputBufferSize = bufferSize;
        this.initialzeInputMaskBuffer(inputBufferSize);
        emit PreprocessingStarted(msg.sender, block.timestamp);
        _nextRound();
    }

    /// @notice Transitions to input gathering phase
    function gatherInputs() external override onlyDesignatedParty atRound(Round.ClientInputMaskReservationRound) {
        emit InputGatheringStarted(msg.sender, block.timestamp);
        _nextRound();
    }

    /// @notice Initiates MPC computation
    function initiateMPCComputation()
        external
        override
        onlyDesignatedParty
        atRound(Round.ClientInputsCollectionEndRound)
    {
        emit MPCComputationInitiated(msg.sender, inputCount, block.timestamp);
        _nextRound();
    }

    /// @notice Publishes computation outputs
    function publishOutputs() external override onlyDesignatedParty atRound(Round.MPCTaskExecutionEndRound) {
        emit OutputsPublished(msg.sender, storedOutputs, block.timestamp);
        _nextRound();
    }

    /// @notice Helper to set outputs for testing
    function setOutputs(bytes calldata outputs) external onlyDesignatedParty {
        storedOutputs = outputs;
    }

    /// @notice Helper to increment input count (simulates receiving inputs)
    function incrementInputCount() external {
        inputCount++;
    }

    /// @notice Helper to get current round for testing
    function getCurrentRound() external view returns (Round) {
        return round;
    }

    /// @notice Helper to manually advance round (for testing error cases)
    function forceNextRound() external onlyDesignatedParty {
        _nextRound();
    }

    /// @notice Helper to transition from CollectingClientInputRound to ClientInputsCollectionEndRound
    function endInputCollection() external onlyDesignatedParty atRound(Round.CollectingClientInputRound) {
        _nextRound();
    }

    /// @notice Helper to transition from MPCTaskExecutionRound to MPCTaskExecutionEndRound
    function endMPCExecution() external onlyDesignatedParty atRound(Round.MPCTaskExecutionRound) {
        _nextRound();
    }

    // =========================================================================
    // Testing Helpers (bypass buggy onlyDesignatedParty modifier)
    // =========================================================================
    // These functions use a manual check instead of the buggy isDesignatedParty()
    // to allow testing the coordinator's round progression logic.

    modifier onlyDesignatedPartyTest() {
        require(isParty(msg.sender), "This account is not an existing MPC Party");
        require(
            hasRole(DESIGNATED_PARTY_ROLE, msg.sender),
            "Only the designated Stofel party can call this function"
        );
        _;
    }

    /// @notice Test helper: Start preprocessing (bypasses buggy modifier)
    /// @dev Sets nTotalIndices directly since initialzeInputMaskBuffer uses buggy onlyDesignatedParty
    function startPreprocessingTest() external onlyDesignatedPartyTest atRound(Round.PreprocessingRound) {
        inputBufferSize = 10;
        _initializeBufferDirect(inputBufferSize);
        emit PreprocessingStarted(msg.sender, block.timestamp);
        _nextRound();
    }

    /// @notice Test helper: Start preprocessing with size (bypasses buggy modifier)
    /// @dev Sets nTotalIndices directly since initialzeInputMaskBuffer uses buggy onlyDesignatedParty
    function startPreprocessingWithSizeTest(uint256 bufferSize)
        external
        onlyDesignatedPartyTest
        atRound(Round.PreprocessingRound)
    {
        inputBufferSize = bufferSize;
        _initializeBufferDirect(bufferSize);
        emit PreprocessingStarted(msg.sender, block.timestamp);
        _nextRound();
    }

    /// @notice Internal helper to initialize buffer directly without external call
    /// @dev Bypasses the buggy onlyDesignatedParty modifier on initialzeInputMaskBuffer
    ///      Uses IndexBufferEvent inherited from StoffelInputManager
    function _initializeBufferDirect(uint256 nIndicesToReserve) internal {
        require(nTotalIndices == 0, "The index buffer has already been set");
        nTotalIndices = nIndicesToReserve;
        emit IndexBufferEvent(nTotalIndices, msg.sender);
    }

    /// @notice Test helper: Gather inputs (bypasses buggy modifier)
    function gatherInputsTest() external onlyDesignatedPartyTest atRound(Round.ClientInputMaskReservationRound) {
        emit InputGatheringStarted(msg.sender, block.timestamp);
        _nextRound();
    }

    /// @notice Test helper: End input collection (bypasses buggy modifier)
    function endInputCollectionTest() external onlyDesignatedPartyTest atRound(Round.CollectingClientInputRound) {
        _nextRound();
    }

    /// @notice Test helper: Initiate MPC computation (bypasses buggy modifier)
    function initiateMPCComputationTest()
        external
        onlyDesignatedPartyTest
        atRound(Round.ClientInputsCollectionEndRound)
    {
        emit MPCComputationInitiated(msg.sender, inputCount, block.timestamp);
        _nextRound();
    }

    /// @notice Test helper: End MPC execution (bypasses buggy modifier)
    function endMPCExecutionTest() external onlyDesignatedPartyTest atRound(Round.MPCTaskExecutionRound) {
        _nextRound();
    }

    /// @notice Test helper: Publish outputs (bypasses buggy modifier)
    function publishOutputsTest() external onlyDesignatedPartyTest atRound(Round.MPCTaskExecutionEndRound) {
        emit OutputsPublished(msg.sender, storedOutputs, block.timestamp);
        _nextRound();
    }

    /// @notice Test helper: Set outputs (bypasses buggy modifier)
    function setOutputsTest(bytes calldata outputs) external onlyDesignatedPartyTest {
        storedOutputs = outputs;
    }

    /// @notice Test helper: Force next round (bypasses buggy modifier)
    function forceNextRoundTest() external onlyDesignatedPartyTest {
        _nextRound();
    }
}
