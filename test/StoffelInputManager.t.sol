// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {MockCoordinator} from "./mocks/MockCoordinator.sol";
import {ECDSA} from "openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "openzeppelin-contracts/contracts/utils/cryptography/MessageHashUtils.sol";

/// @title StoffelInputManagerTest
/// @notice Tests for StoffelInputManager functionality
/// @dev CRITICAL BUG DISCOVERED: reserveInputMask() has backwards require condition.
///      The code is `require(reservedInputIndices[indexToReserve] != address(0), ...)`
///      but should be `require(reservedInputIndices[indexToReserve] == address(0), ...)`.
///      This means NO indices can ever be reserved - the function always reverts.
///      Tests document this bug and use workarounds where possible.
contract StoffelInputManagerTest is Test {
    MockCoordinator public coordinator;

    address public designatedParty;
    uint256 public designatedPartyPrivateKey;
    address public party1;
    address public party2;
    address public client1;
    uint256 public client1PrivateKey;
    address public client2;
    uint256 public client2PrivateKey;
    address public unauthorized;

    bytes32 public constant PROGRAM_HASH = keccak256("test-program");
    uint256 public constant N_PARTIES = 5;
    uint256 public constant THRESHOLD = 1;
    uint256 public constant BUFFER_SIZE = 10;

    // Events to test
    event IndexBufferEvent(uint256 totalIndices, address designatedParty);
    event ReservedInputEvent(address client, uint256 reservedIndex);
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);

    function setUp() public {
        // Generate keys for accounts that need to sign
        designatedPartyPrivateKey = 0x1;
        designatedParty = vm.addr(designatedPartyPrivateKey);

        client1PrivateKey = 0x2;
        client1 = vm.addr(client1PrivateKey);

        client2PrivateKey = 0x3;
        client2 = vm.addr(client2PrivateKey);

        party1 = makeAddr("PARTY1");
        party2 = makeAddr("PARTY2");
        unauthorized = makeAddr("UNAUTHORIZED");

        // Create initial MPC nodes array
        address[] memory initialNodes = new address[](2);
        initialNodes[0] = party1;
        initialNodes[1] = party2;

        // Deploy coordinator (designatedParty gets DESIGNATED_PARTY_ROLE in constructor)
        coordinator = new MockCoordinator(PROGRAM_HASH, N_PARTIES, THRESHOLD, designatedParty, initialNodes);

        // Progress to CollectingClientInputRound for input tests
        // Use Test helpers that bypass the buggy onlyDesignatedParty modifier
        vm.startPrank(designatedParty);
        coordinator.startPreprocessingWithSizeTest(BUFFER_SIZE);
        coordinator.gatherInputsTest();
        vm.stopPrank();
    }

    // =========================================================================
    // Buffer Initialization Tests
    // =========================================================================

    function test_initialzeInputMaskBuffer() public {
        // Create fresh coordinator for this test
        address[] memory nodes = new address[](1);
        nodes[0] = party1;
        MockCoordinator freshCoordinator = new MockCoordinator(PROGRAM_HASH, N_PARTIES, THRESHOLD, designatedParty, nodes);

        vm.prank(designatedParty);
        vm.expectEmit(true, true, true, true);
        emit IndexBufferEvent(5, designatedParty);
        freshCoordinator.startPreprocessingWithSizeTest(5);
    }

    function test_cannotReinitializeBuffer() public {
        // Buffer was already initialized in setUp
        // Create a fresh coordinator and try to initialize twice
        address[] memory nodes = new address[](1);
        nodes[0] = party1;
        MockCoordinator freshCoordinator = new MockCoordinator(PROGRAM_HASH, N_PARTIES, THRESHOLD, designatedParty, nodes);

        vm.startPrank(designatedParty);
        freshCoordinator.startPreprocessingWithSizeTest(5);
        freshCoordinator.gatherInputsTest();

        // Try to initialize again - should revert
        vm.expectRevert("The index buffer has already been set");
        freshCoordinator.initialzeInputMaskBuffer(10);
        vm.stopPrank();
    }

    function test_onlyDesignatedPartyCanInitializeBuffer() public {
        address[] memory nodes = new address[](1);
        nodes[0] = party1;
        MockCoordinator freshCoordinator = new MockCoordinator(PROGRAM_HASH, N_PARTIES, THRESHOLD, designatedParty, nodes);

        // Due to isDesignatedParty bug, this panics with array out-of-bounds instead
        // of giving the expected "Only the designated Stofel party can call this function" error
        vm.prank(unauthorized);
        vm.expectRevert(); // Will panic with array out-of-bounds (0x32)
        freshCoordinator.initialzeInputMaskBuffer(5);
    }

    // =========================================================================
    // Bug Documentation: reserveInputMask backwards require condition
    // =========================================================================

    function test_bug_reserveInputMaskAlwaysReverts() public {
        // BUG: StoffelInputManager.reserveInputMask() has backwards require:
        // `require(reservedInputIndices[indexToReserve] != address(0), ...)`
        // should be:
        // `require(reservedInputIndices[indexToReserve] == address(0), ...)`
        //
        // This means ANY attempt to reserve ANY index will ALWAYS revert
        // because initially all indices map to address(0).
        vm.prank(client1);
        vm.expectRevert("This index has already been reserved");
        coordinator.reserveInputMask(0);
    }

    // =========================================================================
    // Index Reservation Tests (all document the backwards require bug)
    // =========================================================================

    function test_reserveInputMask_revertsWithBug() public {
        // Due to backwards require, this always reverts
        vm.prank(client1);
        vm.expectRevert("This index has already been reserved");
        coordinator.reserveInputMask(0);
    }

    function test_reserveMultipleIndices_revertsWithBug() public {
        vm.prank(client1);
        vm.expectRevert("This index has already been reserved");
        coordinator.reserveInputMask(0);
    }

    function test_currentlyAvailableInputMasks() public view {
        // nIndicesLeft is not properly initialized in the contract
        // This test documents the current behavior
        uint256 available = coordinator.currentlyAvailableInputMasks();
        // Due to a bug in the contract, nIndicesLeft starts at 0 and is never set
        assertEq(available, 0);
    }

    // =========================================================================
    // Input Submission Tests (cannot test properly due to reservation bug)
    // =========================================================================

    function test_submitMaskedInput_cannotTestDueToReservationBug() public {
        // Cannot test submitMaskedInput because reserveInputMask always reverts
        vm.prank(client1);
        vm.expectRevert("This index has already been reserved");
        coordinator.reserveInputMask(0);
    }

    // =========================================================================
    // Client Authentication Tests
    // =========================================================================

    function test_authenticateClient() public {
        // Create a signature for client1
        uint256 requestIndex = 123;
        bytes32 messageHash = keccak256(abi.encode(requestIndex));
        bytes32 ethSignedMessageHash = MessageHashUtils.toEthSignedMessageHash(messageHash);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(client1PrivateKey, ethSignedMessageHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Call authenticateClient as a party
        vm.prank(party1);
        bool isValid = coordinator.authenticateClient(requestIndex, client1, signature);
        assertTrue(isValid);
    }

    function test_authenticateClient_invalidSignature() public {
        uint256 requestIndex = 123;
        bytes32 messageHash = keccak256(abi.encode(requestIndex));
        bytes32 ethSignedMessageHash = MessageHashUtils.toEthSignedMessageHash(messageHash);

        // Sign with client2's key but claim it's client1
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(client2PrivateKey, ethSignedMessageHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(party1);
        bool isValid = coordinator.authenticateClient(requestIndex, client1, signature);
        assertFalse(isValid);
    }

    function test_authenticateClient_wrongRequestIndex() public {
        uint256 requestIndex = 123;
        bytes32 messageHash = keccak256(abi.encode(requestIndex));
        bytes32 ethSignedMessageHash = MessageHashUtils.toEthSignedMessageHash(messageHash);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(client1PrivateKey, ethSignedMessageHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Verify with different request index
        vm.prank(party1);
        bool isValid = coordinator.authenticateClient(456, client1, signature);
        assertFalse(isValid);
    }

    function test_authenticateClient_onlyParty() public {
        uint256 requestIndex = 123;
        bytes32 messageHash = keccak256(abi.encode(requestIndex));
        bytes32 ethSignedMessageHash = MessageHashUtils.toEthSignedMessageHash(messageHash);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(client1PrivateKey, ethSignedMessageHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Non-party caller should revert
        vm.prank(unauthorized);
        vm.expectRevert("Only a Stoffel party can call this function.");
        coordinator.authenticateClient(requestIndex, client1, signature);
    }
}
