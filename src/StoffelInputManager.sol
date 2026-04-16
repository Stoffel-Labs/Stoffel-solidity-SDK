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

    /// Structure for storing an output destined for a specific client.
    struct Output {
        /// The output shares (encrypted for private outputs, unencrypted for public outputs) under the client's key.
        bytes[] shares;
        /// The number of shares so far received from nodes.
        uint256 nShares;
        /// Mapping to track which parties have sent their shares for this client.
        mapping(address => bool) sharesReceived;
    }

    /// @notice Maximum number of clients that can receive outputs
    uint256 internal maxOutputs;

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

    event EnoughOutputShares(address indexed client, bytes[] shares);

    error AlreadyReceivedOutputShares(address client, address sender);

    error AlreadySubmittedInputs(address client);

    error ZeroMaskedInput(address client);

    error IndexNotReserved(address client, uint256 index);

    error NoIndicesReserved(address client);

    error IndexOutOfBounds(address client, uint256 index);

    error ClientAlreadyReservedIndex(address client, uint256 i);

    error IndexAlreadyReserved(uint256 i, address reqClient, address resClient);

    error TooManyOutputClients();

    constructor(uint256 nIndicesToReserve) {
        baseNonce = 0;
        nTotalIndices = nIndicesToReserve;
        nReservedIndices = 0;
        nInputsSubmitted = 0;
        maxOutputs = nIndicesToReserve + 1; // one public output + number of inputs

        emit IndexBufferEvent(nTotalIndices, msg.sender);
    }

    /// @notice Initializes the input mask buffer with a specified number of indices
    function _resetInputManager() internal {
        address[] memory parties = getRoleMembers(PARTY_ROLE);
        uint256 nParties = getRoleMemberCount(PARTY_ROLE);

        for (uint256 i = 0; i < nTotalIndices; i++) {
            address client = reservedInputIndices[i];

            delete clientInputs[client];

            for (uint256 j = 0; j < nParties; j++) {
                delete outputs[client].sharesReceived[parties[j]];
            }
            delete outputs[client];
            delete reservedInputIndices[i];
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

    /// @notice Given shares and a client address, store the shares to be retrieved by the client at a later point.
    /// The coordinator waits until enough shares for reconstruction are available and decryption and reconstruction is up to the
    /// client.
    /// The address `address(0)` is reserved for a public output, which are not encrypted.
    /// It is at this method where new output clients are added: if a node sends output shares for a client, this client becomes an output client if it is not already.
    /// There is an upper limit to the number of output clients. A malicious party can cause a DoS by filling up the storage for outputs by sending shares for many different clients.
    /// @param client The client for which the shares are intended (`address(0)` for public outputs)
    /// @param shares The output shares (encrypted for private outputs, unencrypted for public outputs)
    function sendOutputShares(address client, bytes calldata shares) external onlyRole(PARTY_ROLE) {
        if (!hasRole(OUTPUT_CLIENT_ROLE, client)) {
            // prevent malicious parties from endlessly filling up storage for outputs
            uint256 nOutputClients = getRoleMemberCount(OUTPUT_CLIENT_ROLE);
            if (nOutputClients == maxOutputs) {
                revert TooManyOutputClients();
            }

            _grantRole(OUTPUT_CLIENT_ROLE, client);

            Output storage output = outputs[msg.sender];
            output.shares = new bytes[](n);
            output.nShares = 0;

            address[] memory parties = getRoleMembers(PARTY_ROLE);
            uint256 nParties = getRoleMemberCount(PARTY_ROLE);
            for (uint256 j = 0; j < nParties; j++) {
                output.sharesReceived[parties[j]] = false;
            }
        }

        uint256 nShares = outputs[client].nShares;

        if (outputs[client].sharesReceived[msg.sender]) {
            revert AlreadyReceivedOutputShares(client, msg.sender);
        }
        // more than n output share messages are never stored, since there are only n parties
        require(nShares < n, "BUG: ALREADY RECEIVED SHARES FROM N PARTIES, TOO MANY CLIENTS");

        outputs[client].sharesReceived[msg.sender] = true;
        outputs[client].shares[nShares] = shares;
        outputs[client].nShares += 1;
        nShares += 1;

        if (nShares >= 2 * t + 1) {
            bytes[] memory sentShares = new bytes[](nShares);
            for (uint256 i = 0; i < nShares; i++) {
                sentShares[i] = outputs[client].shares[i];
            }
            emit EnoughOutputShares(client, sentShares);
        }
    }
}
