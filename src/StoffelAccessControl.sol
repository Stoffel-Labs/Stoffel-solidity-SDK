// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/access/extensions/AccessControlEnumerable.sol";
import "./interfaces/IStoffelAccessControl.sol";

/// @title StoffelAccessControl
/// @author Stoffel Labs
/// @notice Role-based access control for MPC party management
/// @dev Extends OpenZeppelin's AccessControl and AccessControlEnumerable to manage
///      MPC compute nodes (PARTY_ROLE) and orchestrators (DESIGNATED_PARTY_ROLE).
///      Enforces n/t threshold constraints for Byzantine fault tolerance.
contract StoffelAccessControl is AccessControl, AccessControlEnumerable, IStoffelAccessControl {
    /// @notice Role identifier for MPC compute nodes
    /// @dev Parties with this role can participate in MPC protocol execution
    bytes32 public constant PARTY_ROLE = keccak256("PARTY_ROLE");

    /// @notice Role identifier for the privileged orchestration party
    /// @dev The designated party can trigger round transitions and manage the MPC lifecycle
    bytes32 public constant DESIGNATED_PARTY_ROLE = keccak256("DESIGNATED_PARTY_ROLE");

    /// @notice Role identifier for clients
    bytes32 public constant CLIENT_ROLE = keccak256("CLIENT_ROLE");

    /// @notice Fault tolerance threshold
    /// @dev Number of faulty/malicious parties the system can tolerate
    uint256 internal threshold;

    /// @notice Emitted when the access control is initialized
    /// @param nParties Initial number of parties
    /// @param t Fault tolerance threshold
    /// @param initializer Address that deployed the contract
    event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);

    error NotAClient(address client);
    error NotEnoughMPCParties(uint256 current, uint256 required);
    error NotAnExistingParty(address account);

    constructor (uint256 t, address[] memory initialMPCNodes) {
        _resetAccessControl(t, initialMPCNodes);
    }

    /// @notice Initializes the access control with party count and threshold
    /// @param t Fault tolerance threshold
    /// @dev Emits InitializeStoffelAccessControl event on deployment
    function _resetAccessControl(uint256 t, address[] memory initialMPCNodes) internal {
        uint256 n = 3 * t + 1;

        if (initialMPCNodes.length < n) {
	    revert NotEnoughMPCParties(initialMPCNodes.length, n);
	}

        threshold = t;

	// revoke all existing roles
        address[] memory designated_parties = getRoleMembers(DESIGNATED_PARTY_ROLE);
        uint256 nDesignatedParties = getRoleMemberCount(DESIGNATED_PARTY_ROLE);
        for (uint256 i = 0; i < nDesignatedParties; i++) {
	    _revokeRole(DESIGNATED_PARTY_ROLE, designated_parties[i]);
        }
        address[] memory parties = getRoleMembers(PARTY_ROLE);
        uint256 nParties = getRoleMemberCount(PARTY_ROLE);
        for (uint256 i = 0; i < nParties; i++) {
	    _revokeRole(PARTY_ROLE, parties[i]);
        }
        address[] memory clients = getRoleMembers(CLIENT_ROLE);
        uint256 nClients = getRoleMemberCount(CLIENT_ROLE);
        for (uint256 i = 0; i < nClients; i++) {
	    _revokeRole(CLIENT_ROLE, clients[i]);
        }

	// grant new roles
        for (uint256 i = 0; i < initialMPCNodes.length; i++) {
            _grantRole(PARTY_ROLE, initialMPCNodes[i]);
        }
        _grantRole(DESIGNATED_PARTY_ROLE, initialMPCNodes[0]);

        emit InitializeStoffelAccessControl(n, t, msg.sender);
    }

    function resetAccessControl(uint256 t, address[] memory initialMPCNodes) external onlyRole(DESIGNATED_PARTY_ROLE) {
	_resetAccessControl(t, initialMPCNodes);
    }

    /// @notice Checks if the contract supports a given interface
    /// @param interfaceId The interface identifier to check
    /// @return True if the interface is supported
    /// @dev Overrides AccessControl and AccessControlEnumerable implementations
    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(AccessControl, AccessControlEnumerable, IStoffelAccessControl)
        returns (bool)
    {
        return interfaceId == type(IStoffelAccessControl).interfaceId || super.supportsInterface(interfaceId);
    }

    /// @notice Internal function to grant a role to an account
    /// @param role The role identifier to grant
    /// @param account The address to receive the role
    /// @return True if the role was newly granted
    /// @dev Enforces maximum party count when granting PARTY_ROLE
    function _grantRole(bytes32 role, address account)
        internal
        override(AccessControl, AccessControlEnumerable)
        returns (bool)
    {
        return super._grantRole(role, account);
    }

    /// @notice Grants a role to an account
    /// @param role The role identifier to grant
    /// @param account The address to receive the role
    /// @dev Public wrapper for _grantRole with access control enforcement
    function grantRole(bytes32 role, address account)
        public
        override(AccessControl, IAccessControl, IStoffelAccessControl)
	onlyRole(DESIGNATED_PARTY_ROLE)
    {
        _grantRole(role, account);
    }

    /// @notice Internal function to revoke a role from an account
    /// @param role The role identifier to revoke
    /// @param account The address to lose the role
    /// @return True if the role was revoked
    function _revokeRole(bytes32 role, address account)
        internal
        override(AccessControl, AccessControlEnumerable)
        returns (bool)
    {
        return super._revokeRole(role, account);
    }

    /// @notice Revokes a role from an account
    /// @param role The role identifier to revoke
    /// @param account The address to lose the role
    /// @dev Public wrapper for _revokeRole with threshold enforcement
    function revokeRole(bytes32 role, address account)
        public
        override(AccessControl, IAccessControl, IStoffelAccessControl)
	onlyRole(DESIGNATED_PARTY_ROLE)
    {
        if (role == PARTY_ROLE && hasRole(role, account)) {
            uint256 current = getRoleMemberCount(role);
	    uint256 n = 3 * threshold + 1;
            if (current == n) {
		revert NotEnoughMPCParties(n - 1, n);
	    }
        }
        _revokeRole(role, account);
    }

    /// @notice Allows an account to renounce their own role
    /// @param role The role identifier to renounce
    /// @param account The address renouncing the role (must be msg.sender)
    /// @dev Inherited from AccessControl, allows self-removal from roles
    function renounceRole(bytes32 role, address account) public override(AccessControl, IAccessControl) {
        if (role == PARTY_ROLE && hasRole(role, account)) {
            uint256 current = getRoleMemberCount(role);
	    uint256 n = 3 * threshold + 1;
            if (current == n) {
		revert NotEnoughMPCParties(n - 1, n);
	    }
        }
        super.renounceRole(role, account);
    }

    /// @notice Internal function to set the admin role for a given role
    /// @param role The role whose admin is being set
    /// @param adminRole The role that will be the admin
    /// @dev Inherited from AccessControl for role hierarchy management
    function _setRoleAdmin(bytes32 role, bytes32 adminRole) internal override(AccessControl) {
        super._setRoleAdmin(role, adminRole);
    }

    /// @notice Checks if an address is a registered MPC party
    /// @param account The address to check
    /// @return True if the account has PARTY_ROLE
    function isParty(address account) public view returns (bool) {
        return hasRole(PARTY_ROLE, account);
    }

    /// @notice Checks if an address is the designated party
    /// @param account The address to check
    /// @return True if the account has DESIGNATED_PARTY_ROLE
    /// @dev Requires the caller to be an existing party
    function isDesignatedParty(address account) public view returns (bool) {
	if (!isParty(account)) {
	    revert NotAnExistingParty(account);
	}
	return hasRole(DESIGNATED_PARTY_ROLE, account);
    }
}
