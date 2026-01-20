// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "./interfaces/IStoffelInputManager.sol";
import "./StoffelAccessControl.sol";
import "openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
import "openzeppelin-contracts/contracts/utils/cryptography/MessageHashUtils.sol";

/// @title StoffelInputManager
/// @author Stoffel Labs
/// @notice Abstract contract for managing privacy-preserving client input submission
/// @dev Implements input mask reservation and masked input submission for MPC computations.
///      Clients reserve mask indices, obtain masks off-chain from MPC nodes, then submit
///      masked inputs on-chain. This ensures raw inputs are never exposed on the blockchain.
abstract contract StoffelInputManager is StoffelAccessControl, IStoffelInputManager {
    /// @notice Mapping from input mask index to the client who reserved it
    /// @dev Address(0) indicates the index is available for reservation
    mapping(uint256 => address) internal reservedInputIndices;

    /// @notice Total number of input mask indices available
    /// @dev Set once during preprocessing by the designated party
    uint256 internal nTotalIndices;

    /// @notice Number of input mask indices still available for reservation
    /// @dev Decrements as clients reserve indices
    uint256 internal nIndicesLeft;

    /// @notice Structure representing a client's masked input
    /// @dev Contains the reserved index and the masked value submitted by the client
    struct MaskedInput {
        /// @notice The input mask index reserved by the client
        uint256 index;
        /// @notice The masked input value (raw input + mask)
        uint256 maskedInput;
    }

    /// @notice Mapping from client address to their submitted masked input
    mapping(address => MaskedInput) internal clientInputs;

    /// @notice Emitted when the input mask buffer is initialized
    /// @param totalIndices Total number of input mask indices available
    /// @param designatedParty Address that initialized the buffer
    event IndexBufferEvent(uint256 totalIndices, address designatedParty);

    /// @notice Emitted when a client reserves an input mask index
    /// @param client Address of the client reserving the index
    /// @param reservedIndex The index that was reserved
    event ReservedInputEvent(address client, uint256 reservedIndex);

    /// @notice Emitted when a client submits their masked input
    /// @param client Address of the client submitting the input
    /// @param maskedInput The masked input value
    /// @param reservedIndex The index used for this input
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);

    /// @notice Initializes the input mask buffer with a specified number of indices
    /// @param nIndicesToReserve Number of input mask indices to make available
    /// @dev Can only be called once by the designated party during preprocessing.
    ///      This determines how many clients can participate in the computation.
    function initialzeInputMaskBuffer(uint256 nIndicesToReserve) external onlyDesignatedParty {
        require(nTotalIndices == 0, "The index buffer has already been set");
        nTotalIndices = nIndicesToReserve;

        emit IndexBufferEvent(nTotalIndices, msg.sender);
    }

    /// @notice Reserves an input mask index for the calling client
    /// @param indexToReserve The index to reserve (must be less than nTotalIndices)
    /// @dev Clients must reserve an index before they can request the corresponding
    ///      input mask from MPC nodes and submit their masked input.
    function reserveInputMask(uint256 indexToReserve) external override {
        require(reservedInputIndices[indexToReserve] != address(0), "This index has already been reserved");
        require(nIndicesLeft > 0, "No more indices to reserve");

        reservedInputIndices[indexToReserve] = msg.sender;
        nIndicesLeft--;

        emit ReservedInputEvent(msg.sender, indexToReserve);
    }

    /// @notice Returns the number of input mask indices still available
    /// @return Number of unreserved indices
    function currentlyAvailableInputMasks() external view override returns (uint256) {
        return nIndicesLeft;
    }

    /// @notice Submits a masked input using a previously reserved index
    /// @param maskedInput The masked input value (client's raw input + mask)
    /// @param reservedIndex The index that was previously reserved by this client
    /// @dev After submission, the index is unreserved to prevent mask reuse.
    ///      The mask must be obtained off-chain from MPC nodes before calling this.
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex) external override {
        require(
            reservedInputIndices[reservedIndex] == msg.sender,
            "This client did not reserve the input mask at this index"
        );
        clientInputs[msg.sender] = MaskedInput(reservedIndex, maskedInput);

        emit MaskedInputEvent(msg.sender, maskedInput, reservedIndex);

        // Unreserve index once the input mask has been used
        reservedInputIndices[reservedIndex] = address(0);
    }

    /// @notice Authenticates a client using ECDSA signature verification
    /// @param requestIndex Unique identifier for the authentication request
    /// @param clientAddr Expected client address
    /// @param signature ECDSA signature over the request index
    /// @return True if the signature was created by the claimed client address
    /// @dev Called off-chain by MPC nodes to verify client identity before
    ///      providing input masks. Uses EIP-191 signed message format.
    function authenticateClient(uint256 requestIndex, address clientAddr, bytes calldata signature)
        external
        override
        onlyParty
        returns (bool)
    {
        bytes32 hashedMsg = MessageHashUtils.toEthSignedMessageHash(keccak256(abi.encode(requestIndex)));
        address clientAddress = ECDSA.recover(hashedMsg, signature);

        return clientAddress == clientAddr;
    }
}
