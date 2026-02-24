// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title IStoffelInputManager
/// @author Stoffel Labs
/// @notice Interface for privacy-preserving client input management
/// @dev Defines the functions for input mask reservation, submission,
///      and client authentication in MPC computations.
interface IStoffelInputManager {
    /// @notice Reserves input mask indices for the calling client
    /// @param nIndices The number of indices to reserve
    /// @dev Must be called before obtaining the mask from MPC nodes
    function obtainInputMasks(uint256 nIndices) external returns (uint256[] memory);

    /// @notice Returns the number of input mask indices still available
    /// @return Number of unreserved indices
    function availableInputMasks() external view returns (uint256);

    /// @notice Submits a masked input using a previously reserved index
    /// @param maskedInput The masked input value (raw input + mask)
    /// @param reservedIndexed The index that was previously reserved
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndexed) external;

    /// @notice Authenticates a client using ECDSA signature verification, proving ownership of an address
    /// @param clientAddr Address whose ownership is to be proved
    /// @param signature ECDSA signature over the nonce
    function authenticateClient(address clientAddr, bytes calldata signature)
        external;
}
