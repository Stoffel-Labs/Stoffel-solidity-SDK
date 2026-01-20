// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/access/IAccessControl.sol";
import "@openzeppelin/contracts/interfaces/IERC5267.sol";
import "@openzeppelin/contracts/utils/introspection/IERC165.sol";

/// @title IStoffelAccessControl
/// @author Stoffel Labs
/// @notice Interface for MPC party access control management
/// @dev Extends OpenZeppelin's IAccessControl and IERC165 for role-based
///      access control of MPC compute nodes and designated parties.
interface IStoffelAccessControl is IAccessControl, IERC165 {
    /// @notice Checks if the contract supports a given interface
    /// @param interfaceId The interface identifier to check (ERC-165)
    /// @return True if the interface is supported
    function supportsInterface(bytes4 interfaceId) external view virtual returns (bool);

    /// @notice Grants a role to an account
    /// @param role The role identifier (PARTY_ROLE or DESIGNATED_PARTY_ROLE)
    /// @param account The address to receive the role
    function grantRole(bytes32 role, address account) external virtual;

    /// @notice Revokes a role from an account
    /// @param role The role identifier to revoke
    /// @param account The address to lose the role
    function revokeRole(bytes32 role, address account) external virtual;

    /// @notice Checks if an address is a registered MPC party
    /// @param account The address to check
    /// @return True if the account has the PARTY_ROLE
    function isParty(address account) external view virtual returns (bool);
}
