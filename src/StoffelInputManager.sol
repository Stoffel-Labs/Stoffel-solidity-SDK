// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStoffelInputManager} from "./interfaces/IStoffelInputManager.sol";
import {StoffelAccessControl} from "./StoffelAccessControl.sol";

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

    /// @notice Structure for storing an output destined for a specific client
    struct Output {
        /// @notice The output shares (encrypted for private outputs, unencrypted for public outputs) under the client's key
        bytes[] shares;
        /// @notice The number of shares so far received from nodes
        uint256 nShares;
        /// @notice Mapping to track which parties have sent their shares for this client
        mapping(address => bool) sharesReceived;
    }

    /// @notice Addresses of clients that may receive output shares
    address[] internal outputClients;

    /// @notice Private encrypted output shares for specific clients and public unencrypted output shares at `address(0)`.
    mapping(address => Output) internal outputs;

    /// @notice Mapping from input mask index to the client who reserved it
    /// @dev Address(0) indicates the index is available for reservation
    mapping(uint256 => address) internal reservedInputIndices;

    /// @notice Total number of input mask indices available
    /// @dev Set once during preprocessing by the designated party
    uint256 internal nTotalIndices;

    /// @notice The number of indices reserved so far
    uint256 internal nReservedIndices;

    /// @notice The number of masked inputs submitted by clients so far
    uint256 internal nInputsSubmitted;

    /// Currently, each mask index is authenticated separately with a signature.
    /// Each signature signs a nonce. The nonce signed by the signature for index i is
    /// `baseNonce + i`.
    /// After an instance has been executed, i.e., upon reset, the base nonce is incremented by the
    /// total number of indices to enforce the nonce's uniqueness.
    uint256 public baseNonce;

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
    /// @param reservedIndex The index that has been reserved
    event ReservedInputEvent(address client, uint256 reservedIndex);

    /// @notice Emitted when a client submits their masked input
    /// @param client Address of the client submitting the input
    /// @param maskedInput The masked input value
    /// @param reservedIndex The index used for this input
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);

    /// @notice Emitted when enough output shares for reconstruction have been received for a client
    /// @param client The client address the shares are for (`address(0)` for public outputs)
    /// @param shares All received shares, ready for client-side reconstruction
    event EnoughOutputShares(address indexed client, bytes[] shares);

    /// @notice Thrown when a party tries to send output shares for a client it already submitted shares for
    error AlreadyReceivedOutputShares(address client, address sender);

    /// @notice Thrown when a client tries to submit a masked input but has already done so
    error AlreadySubmittedInputs(address client);

    /// @notice Thrown when a client submits a masked input of zero, which is disallowed
    error ZeroMaskedInput(address client);

    /// @notice Thrown when a client submits a masked input for an index it did not reserve
    error IndexNotReserved(address client, uint256 index);

    /// @notice Thrown when an operation requires the client to have a reserved index but none exists
    error NoIndicesReserved(address client);

    /// @notice Thrown when a client attempts to reserve an index that is out of range
    error IndexOutOfBounds(address client, uint256 index);

    /// @notice Thrown when a client attempts to reserve a second index
    error ClientAlreadyReservedIndex(address client, uint256 i);

    /// @notice Thrown when a client attempts to reserve an index already held by another client
    error IndexAlreadyReserved(uint256 i, address reqClient, address resClient);

    /// @notice Thrown when a party tries to send output shares for an address not in the registered output client list
    error OutputClientNotRegistered(address client);

    /// @notice Initializes the input manager with the number of input slots and the set of output clients
    /// @param nIndicesToReserve Total number of input mask indices clients may reserve
    /// @param initialOutputClients Addresses that are allowed to receive output shares; include address(0) for public output
    constructor(uint256 nIndicesToReserve, address[] memory initialOutputClients) {
        baseNonce = 0;
        nTotalIndices = nIndicesToReserve;
        nReservedIndices = 0;
        nInputsSubmitted = 0;

        for (uint256 i = 0; i < initialOutputClients.length; i++) {
            _grantRole(OUTPUT_CLIENT_ROLE, initialOutputClients[i]);
        }
        outputClients = initialOutputClients;

        emit IndexBufferEvent(nTotalIndices, msg.sender);
    }

    /// @notice Initializes the input mask buffer with a specified number of indices
    function _resetInputManager() internal {
        address[] memory parties = getRoleMembers(PARTY_ROLE);
        uint256 nParties = getRoleMemberCount(PARTY_ROLE);
        address[] memory outputClients = getRoleMembers(OUTPUT_CLIENT_ROLE);
        uint256 nOutputClients = getRoleMemberCount(OUTPUT_CLIENT_ROLE);

        for (uint256 i = 0; i < nTotalIndices; i++) {
            address client = reservedInputIndices[i];

            delete clientInputs[client];
            delete reservedInputIndices[i];
        }

        // Properly clear state and revoke roles for all output clients
        for (uint256 i = 0; i < nOutputClients; i++) {
            address client = outputClients[i];
            for (uint256 j = 0; j < nParties; j++) {
                delete outputs[client].sharesReceived[parties[j]];
            }
            delete outputs[client];
            _revokeRole(OUTPUT_CLIENT_ROLE, client);
        }

        nReservedIndices = 0;
        nInputsSubmitted = 0;
        baseNonce += nTotalIndices;
    }

    /// @notice Returns the number of input mask indices still available
    /// @return Number of unreserved indices
    function availableInputMasks() external view override returns (uint256) {
        return nTotalIndices - nReservedIndices;
    }

    /// @notice Reserves an input mask index for the calling client
    /// @param i The index to reserve
    /// @dev Clients must reserve an index before they can request the corresponding
    ///      input mask from MPC nodes and submit their masked input.
    function reserveMaskIndex(uint256 i) external override {
        // check if index within bounds
        if (i >= nTotalIndices) {
            revert IndexOutOfBounds(msg.sender, i);
        }

        // check if client already reserved indices
        for (uint256 j = 0; j < nTotalIndices; j++) {
            if (reservedInputIndices[j] == msg.sender) {
                revert ClientAlreadyReservedIndex(msg.sender, j);
            }
        }

        // check if index available
        if (reservedInputIndices[i] != address(0)) {
            revert IndexAlreadyReserved(i, msg.sender, reservedInputIndices[i]);
        }

        reservedInputIndices[i] = msg.sender;
        _grantRole(INPUT_CLIENT_ROLE, msg.sender);

        nReservedIndices++;
        emit ReservedInputEvent(msg.sender, i);
    }

    /// @notice Submits a masked input using a previously reserved index
    /// @param maskedInput The masked input value (client's raw input + mask)
    /// @param reservedIndex The index that was previously reserved by this client
    /// @dev After submission, the index is unreserved to prevent mask reuse.
    ///      The mask must be obtained off-chain from MPC nodes before calling this.
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex)
        external
        override
        onlyRole(INPUT_CLIENT_ROLE)
    {
        if (reservedInputIndices[reservedIndex] != msg.sender) {
            revert IndexNotReserved(msg.sender, reservedIndex);
        }

        if (maskedInput == 0) {
            revert ZeroMaskedInput(msg.sender);
        }

        if (clientInputs[msg.sender].maskedInput != 0) {
            revert AlreadySubmittedInputs(msg.sender);
        }

        clientInputs[msg.sender] = MaskedInput({index: reservedIndex, maskedInput: maskedInput});

        emit MaskedInputEvent(msg.sender, maskedInput, reservedIndex);
        nInputsSubmitted++;
    }

    /// @notice Stores output shares sent by an MPC party for a given client
    /// @param client The client the shares are intended for; use `address(0)` for public (unencrypted) outputs
    /// @param shares The output shares — encrypted under the client's key for private outputs, plaintext for public
    /// @dev Emits EnoughOutputShares once the reconstruction threshold (2t+1) is reached.
    ///      New output clients are registered on first share receipt; the total is capped at maxOutputs
    ///      to prevent a malicious party from exhausting storage by submitting shares for arbitrary addresses.
    ///      Decryption and secret reconstruction are performed client-side.
    function _sendOutputShares(address client, bytes calldata shares) internal {
        if (!hasRole(OUTPUT_CLIENT_ROLE, client)) {
            revert OutputClientNotRegistered(client);
        }

        Output storage output = outputs[client];

        // Output storage is initialized lazily on first share receipt to avoid iterating over all
        // output clients at coordinator initialization.
        if (output.shares.length == 0) {
            output.shares = new bytes[](n);
            output.nShares = 0;

            address[] memory parties = getRoleMembers(PARTY_ROLE);
            uint256 nParties = getRoleMemberCount(PARTY_ROLE);
            for (uint256 j = 0; j < nParties; j++) {
                output.sharesReceived[parties[j]] = false;
            }
        }

        uint256 nShares = output.nShares;

        if (output.sharesReceived[msg.sender]) {
            revert AlreadyReceivedOutputShares(client, msg.sender);
        }
        // more than n output share messages are never stored, since there are only n parties
        require(nShares < n, "BUG: ALREADY RECEIVED SHARES FROM N PARTIES, TOO MANY CLIENTS");

        output.sharesReceived[msg.sender] = true;
        output.shares[nShares] = shares;
        output.nShares += 1;
        nShares += 1;

        if (nShares >= 2 * t + 1) {
            bytes[] memory sentShares = new bytes[](nShares);
            for (uint256 i = 0; i < nShares; i++) {
                sentShares[i] = output.shares[i];
            }
            emit EnoughOutputShares(client, sentShares);
        }
    }
}
