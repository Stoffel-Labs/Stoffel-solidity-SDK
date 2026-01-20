// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title IStoffelInputManager
/// @author Stoffel Labs
/// @notice Interface for privacy-preserving client input management
/// @dev Defines the functions for input mask reservation, submission,
///      and client authentication in MPC computations.
interface IStoffelInputManager {
    /// @notice Reserves an input mask index for the calling client
    /// @param indexToReserve The index to reserve
    /// @dev Must be called before obtaining the mask from MPC nodes
    function reserveInputMask(uint256 indexToReserve) external virtual;

    /// @notice Returns the number of input mask indices still available
    /// @return Number of unreserved indices
    function currentlyAvailableInputMasks() external view virtual returns (uint256);

    /// @notice Submits a masked input using a previously reserved index
    /// @param maskedInput The masked input value (raw input + mask)
    /// @param reservedIndexed The index that was previously reserved
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndexed) external virtual;

    /// @notice Authenticates a client using ECDSA signature verification
    /// @param requestIndex Unique identifier for the authentication request
    /// @param clientAddr Expected client address
    /// @param signature ECDSA signature over the request index
    /// @return True if the signature was created by the claimed client address
    function authenticateClient(uint256 requestIndex, address clientAddr, bytes calldata signature)
        external
        virtual
        returns (bool);
}
