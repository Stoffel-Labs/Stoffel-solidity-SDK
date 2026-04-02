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
    /// @notice Structure for storing computation inputs
    /// @dev Contains both public parameters and privacy-preserving masked inputs
    struct Inputs {
        /// @notice Public inputs visible to all parties
        bytes publicInputs;
        /// @notice Array of masked client inputs for privacy-preserving computation
        MaskedInput[] maskedInputs;
    }

    /// Structure for storing an output destined for a specific client.
    struct PerClientOutput {
        /// The output shares encrypted under the client's key.
        bytes[] shares;
	/// The number of shares so far received from nodes.
	uint256 nShares;
	/// Mapping to track which parties have sent their shares for this client.
	mapping (address => bool) sharesReceived;
    }

    /// Encrypted output shares per client for private per-client outputs.
    mapping (address => PerClientOutput) internal privateOutputs;
    /// Public outputs.
    bytes internal publicOutputs;

    /// @notice Mapping from input mask index to the client who reserved it
    /// @dev Address(0) indicates the index is available for reservation
    mapping(uint256 => address) internal reservedInputIndices;

    // first value is number negative auths, second is number of positive auths
    mapping(address => uint256[2]) internal clientAuths;

    /// @notice Total number of input mask indices available
    /// @dev Set once during preprocessing by the designated party
    uint256 internal nTotalIndices;

    /// @notice Next available input mask index
    /// @dev Increments as clients reserve indices
    uint256 internal nNextIndex;

    uint256 internal nInputsSubmitted;

    /// Currently, each mask index is authenticated separately with a signature.
    /// Each signature signs a nonce. The nonce signed by the signature for index i is
    /// `baseNonce + i`.
    /// After an instance has been executed, i.e., upon reset, the base nonce is incremented by the
    /// total number of indices to enforce the nonce's uniqueness.
    uint256 public baseNonce;

    /// The threshold value. Used for authentication.
    uint256 internal t;

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
    /// @param reservedIndices The indices that have been reserved
    event ReservedInputEvent(address client, uint256[] reservedIndices);

    /// @notice Emitted when a client submits their masked input
    /// @param client Address of the client submitting the input
    /// @param maskedInput The masked input value
    /// @param reservedIndex The index used for this input
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);

    event ClientAuthenticated(address indexed client, bool success);

    event EnoughPrivateOutputShares(address indexed client, bytes[] shares);

    error AlreadyReceivedOutputShares(address client, address sender);

    error AlreadySubmittedInputs(address client);

    error ZeroMaskedInput(address client);

    error ZeroIndices(address client);

    error NotEnoughIndices(uint256 requested, uint256 available);

    error IndexNotReserved(address client, uint256 index);

    error NoIndicesReserved(address client);

    error IndicesAlreadyReserved(address client);

    constructor (uint256 nIndicesToReserve, uint256 t) {
	baseNonce = 0;
        _resetInputManager(nIndicesToReserve, t);
    }

    /// @notice Initializes the input mask buffer with a specified number of indices
    /// @param nIndicesToReserve Number of input mask indices to make available
    /// @param _t The threshold value
    function _resetInputManager(uint256 nIndicesToReserve, uint256 _t) internal {
	baseNonce += nTotalIndices;
	nTotalIndices = nIndicesToReserve;
	nNextIndex = 0;
	nInputsSubmitted = 0;
	t = _t;

        address[] memory parties = getRoleMembers(PARTY_ROLE);
        uint256 nParties = getRoleMemberCount(PARTY_ROLE);

	for (uint256 i = 0; i < nTotalIndices; i++) {
	    delete clientInputs[reservedInputIndices[i]];
	    delete clientAuths[reservedInputIndices[i]];

       	    for (uint256 i = 0; i < nParties; i++) {
            	delete privateOutputs[reservedInputIndices[i]].sharesReceived[parties[i]];
       	    }
	    delete privateOutputs[reservedInputIndices[i]];
	    delete reservedInputIndices[i];
	}

	emit IndexBufferEvent(nTotalIndices, msg.sender);
    }

    function resetInputManager(uint256 nIndicesToReserve, uint256 t) external onlyRole(DESIGNATED_PARTY_ROLE) {
        _resetInputManager(nIndicesToReserve, t);
    }

    /// @notice Reserves input mask indices for the calling client
    /// @param nIndices The number of indices to reserve
    /// @dev Clients must reserve an index before they can request the corresponding
    ///      input mask from MPC nodes and submit their masked input.
    function obtainInputMasks(uint256 nIndices) external override returns (uint256[] memory) {
	require(nIndices == 1, "CURRENTLY ONLY ONE INDEX PER CLIENT ALLOWED");

	if (nIndices == 0) {
	    revert ZeroIndices(msg.sender);
	}

	uint256 nIndicesLeft = nTotalIndices - nNextIndex; 

        if (nIndices > nIndicesLeft) {
	    revert NotEnoughIndices(nIndices, nIndicesLeft);
	}

	// check if client already reserved indices
	for (uint256 i = 0; i < nTotalIndices; i++) {
	    if (reservedInputIndices[i] == msg.sender) {
		revert IndicesAlreadyReserved(msg.sender);
	    }
	}

	uint256 nFinalIndex = nNextIndex + nIndices - 1;
        uint256[] memory indices = new uint256[](nIndices);
	uint256 firstIndex = nNextIndex;

	for ( ; nNextIndex <= nFinalIndex; nNextIndex++) {
            reservedInputIndices[nNextIndex] = msg.sender;
            indices[nNextIndex - firstIndex] = nNextIndex;
	}

	_grantRole(CLIENT_ROLE, msg.sender);

	PerClientOutput storage output = privateOutputs[msg.sender];
	output.shares = new bytes[](3 * t + 1);
	output.nShares = 0;

	// TODO: not sure if this is the best way, consider storing the relevant list of nodes elsewhere perhaps?
        address[] memory parties = getRoleMembers(PARTY_ROLE);
        uint256 nParties = getRoleMemberCount(PARTY_ROLE);
        for (uint256 i = 0; i < nParties; i++) {
	    output.sharesReceived[parties[i]] = false;
        }

        emit ReservedInputEvent(msg.sender, indices);
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
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex) external override onlyRole(CLIENT_ROLE) {
	if (reservedInputIndices[reservedIndex] != msg.sender) {
            revert IndexNotReserved(msg.sender, reservedIndex);
	}

	if (maskedInput == 0) {
	    revert ZeroMaskedInput(msg.sender);
	}

	if (clientInputs[msg.sender].maskedInput != 0) {
	    revert AlreadySubmittedInputs(msg.sender);
	}

        clientInputs[msg.sender] = MaskedInput({ index: reservedIndex, maskedInput: maskedInput });

        emit MaskedInputEvent(msg.sender, maskedInput, reservedIndex);
	nInputsSubmitted++;
    }

    /// @notice Authenticates a client using ECDSA signature verification, proving ownership of an address
    /// @param clientAddr Address whose ownership is to be proved
    /// @param signature ECDSA signature over the nonce
    /// @dev Called off-chain by MPC nodes to verify client identity before
    ///      providing input masks. Uses EIP-191 signed message format.
    function authenticateClient(address clientAddr, bytes calldata signature)
        external
        override
        onlyRole(PARTY_ROLE)
    {
	if (!hasRole(CLIENT_ROLE, clientAddr)) {
	    revert NotAClient(clientAddr);
	}

	// the nonce is the current base value plus the lowest reserved index
	uint256 lowestReservedIndex = nTotalIndices;
	for (uint256 i = 0; i < nTotalIndices; i++) {
	    if (reservedInputIndices[i] == clientAddr) {
		lowestReservedIndex = i;
		break;
	    }
	}
	if (lowestReservedIndex == nTotalIndices) {
            revert NoIndicesReserved(reservedInputIndices[0]);
	}
	uint256 nonce = baseNonce + lowestReservedIndex;

        bytes32 hashedMsg = MessageHashUtils.toEthSignedMessageHash(keccak256(abi.encode(nonce)));
        address clientAddress = ECDSA.recover(hashedMsg, signature);

        if (clientAddress == clientAddr) {
	    ++clientAuths[clientAddr][1];
	} else {
	    ++clientAuths[clientAddr][0];
	}

	require(clientAuths[clientAddr][0] < t + 1 || clientAuths[clientAddr][1] < t + 1, "BUG: the authentication votes by honest clients are inconsistent");

	/// We wait for t+1 calls with the same verification result to be sure that at
	/// least one of them has been made by an honest party.
	if (clientAuths[clientAddr][0] >= t + 1) {
            emit ClientAuthenticated(clientAddr, false);
	} else if (clientAuths[clientAddr][1] >= t + 1) {
            emit ClientAuthenticated(clientAddr, true);
	}
    }

    function sendPublicOutputs(bytes calldata _publicOutputs) external onlyRole(DESIGNATED_PARTY_ROLE) {
	publicOutputs = _publicOutputs;
    }

    /// @notice Given encrypted shares and a client address, store the shares to be retrieved by the client at a later point.
    /// The coordinator waits until enough shares for reconstruction are available and decryption and reconstruction is up to the
    /// client.
    function sendPrivateOutputShares(address client, bytes calldata shares) external onlyRole(PARTY_ROLE) {
	if (!hasRole(CLIENT_ROLE, client)) {
            revert NotAClient(client);
	}

        uint256 nShares = privateOutputs[client].nShares;

	if (privateOutputs[client].sharesReceived[msg.sender]) {
            revert AlreadyReceivedOutputShares(client, msg.sender);
	}
	// more than n output share messages are never stored, since there are only n parties
	require(nShares < 3 * t + 1, "BUG: ALREADY RECEIVED SHARES FROM N PARTIES, TOO MANY CLIENTS");

	privateOutputs[client].sharesReceived[msg.sender] = true;
	privateOutputs[client].shares[nShares] = shares;
	privateOutputs[client].nShares += 1;
	nShares += 1;

	if (nShares >= 2 * threshold + 1) {
	    bytes[] memory shares = new bytes[](nShares);
	    for (uint256 i = 0; i < nShares; i++) {
		shares[i] = privateOutputs[client].shares[i];
	    }
	    emit EnoughPrivateOutputShares(client, shares);
	}
    }
}
