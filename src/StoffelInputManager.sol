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

    /// @notice Next available input mask index
    /// @dev Increments as clients reserve indices
    uint256 internal nNextIndex;

    uint256 internal nInputsSubmitted;

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

    error NotEnoughIndices(uint256 requested, uint256 available);

    error IndexNotReserved(address client, uint256 index);

    constructor (uint256 nIndicesToReserve) {
        _resetInputManager(nIndicesToReserve);
    }

    /// @notice Initializes the input mask buffer with a specified number of indices
    /// @param nIndicesToReserve Number of input mask indices to make available
    /// @dev Can only be called once by the designated party during preprocessing.
    ///      This determines how many clients can participate in the computation.
    function _resetInputManager(uint256 nIndicesToReserve) internal {
        nTotalIndices = nIndicesToReserve;
	nNextIndex = 0;
	nInputsSubmitted = 0;

        emit IndexBufferEvent(nTotalIndices, msg.sender);
    }

    function resetInputManager(uint256 nIndicesToReserve) external onlyRole(DESIGNATED_PARTY_ROLE) {
        _resetInputManager(nIndicesToReserve);
    }

    /// @notice Reserves input mask indices for the calling client
    /// @param nIndices The number of indices to reserve
    /// @dev Clients must reserve an index before they can request the corresponding
    ///      input mask from MPC nodes and submit their masked input.
    function obtainInputMasks(uint256 nIndices) external override returns (uint256[] memory) {
	uint256 nIndicesLeft = nTotalIndices - nNextIndex; 

        if (nIndices > nIndicesLeft) {
	    revert NotEnoughIndices(nIndices, nIndicesLeft);
	}

	uint256 nFinalIndex = nNextIndex + nIndices - 1;
        uint256[] memory indices = new uint256[](nIndices);
	uint256 firstIndex = nNextIndex;

	for ( ; nNextIndex <= nFinalIndex; nNextIndex++) {
            reservedInputIndices[nNextIndex] = msg.sender;
            indices[nNextIndex - firstIndex] = nNextIndex;
            emit ReservedInputEvent(msg.sender, nNextIndex);
	}

	nIndicesLeft -= nIndices;

	return indices;
    }

    /// @notice Returns the number of input mask indices still available
    /// @return Number of unreserved indices
    function availableInputMasks() external view override returns (uint256) {
        return nTotalIndices - nNextIndex;
    }

    /// @notice Submits a masked input using a previously reserved index
    /// @param maskedInput The masked input value (client's raw input + mask)
    /// @param reservedIndex The index that was previously reserved by this client
    /// @dev After submission, the index is unreserved to prevent mask reuse.
    ///      The mask must be obtained off-chain from MPC nodes before calling this.
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex) external override {
	if (reservedInputIndices[reservedIndex] != msg.sender) {
            revert IndexNotReserved(msg.sender, reservedIndex);
	}

        clientInputs[msg.sender] = MaskedInput(reservedIndex, maskedInput);

        emit MaskedInputEvent(msg.sender, maskedInput, reservedIndex);
	nInputsSubmitted++;
    }

    /// @notice Authenticates a client using ECDSA signature verification, proving ownership of an address
    /// @param nonce Unique identifier for the authentication request
    /// @param clientAddr Address whose ownership is to be proved
    /// @param signature ECDSA signature over the nonce
    /// @return True if the signature was created by the claimed client address
    /// @dev Called off-chain by MPC nodes to verify client identity before
    ///      providing input masks. Uses EIP-191 signed message format.
    function authenticateClient(uint256 nonce, address clientAddr, bytes calldata signature)
        external
        override
        onlyRole(PARTY_ROLE)
        returns (bool)
    {
        bytes32 hashedMsg = MessageHashUtils.toEthSignedMessageHash(keccak256(abi.encode(nonce)));
        address clientAddress = ECDSA.recover(hashedMsg, signature);

        return clientAddress == clientAddr;
    }
}
