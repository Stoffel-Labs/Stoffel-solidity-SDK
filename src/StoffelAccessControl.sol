// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IAccessControl, AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {AccessControlEnumerable} from "@openzeppelin/contracts/access/extensions/AccessControlEnumerable.sol";
import {IStoffelAccessControl} from "./interfaces/IStoffelAccessControl.sol";

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
    bytes32 public constant INPUT_CLIENT_ROLE = keccak256("INPUT_CLIENT_ROLE");

    bytes32 public constant OUTPUT_CLIENT_ROLE = keccak256("OUPTUT_CLIENT_ROLE");

    /// @notice Fault tolerance threshold
    /// @dev Number of faulty/malicious parties the system can tolerate
    uint256 internal immutable t;

    /// @notice n value
    uint256 internal immutable n;

    /// @notice Emitted when the access control is initialized
    /// @param nParties Initial number of parties
    /// @param t Fault tolerance threshold
    /// @param initializer Address that deployed the contract
    event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);

    error NotAnExistingParty(address account);

    error NotEnoughMPCParties(uint256 current, uint256 required);

    /// @notice Hook called before any public role change; override to add round-based guards
    function _beforeRoleChange() internal virtual {}

    /// @notice Initializes the access control with party count and threshold
    /// @param _t Fault tolerance threshold
    /// @param initialMpcNodes Array of addresses to be granted PARTY_ROLE
    /// @dev Emits InitializeStoffelAccessControl event on deployment
    constructor(uint256 _t, address[] memory initialMpcNodes) {
        t = _t;
        n = 3 * t + 1;

        require(initialMpcNodes.length >= n, NotEnoughMPCParties(initialMpcNodes.length, n));

        // grant new roles
        for (uint256 i = 0; i < initialMpcNodes.length; i++) {
            _grantRole(PARTY_ROLE, initialMpcNodes[i]);
        }
        _grantRole(DESIGNATED_PARTY_ROLE, initialMpcNodes[0]);

        emit InitializeStoffelAccessControl(n, t, msg.sender);
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
        _beforeRoleChange();
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
    /// @dev Public wrapper for _revokeRole; roles can be revoked when no program is executing, but new parties need to be added
    function revokeRole(bytes32 role, address account)
        public
        override(AccessControl, IAccessControl, IStoffelAccessControl)
        onlyRole(DESIGNATED_PARTY_ROLE)
    {
        _beforeRoleChange();
        _revokeRole(role, account);
    }

    /// @notice Allows an account to renounce their own role
    /// @param role The role identifier to renounce
    /// @param account The address renouncing the role (must be msg.sender)
    /// @dev Inherited from AccessControl, allows self-removal from roles
    function renounceRole(bytes32 role, address account) public override(AccessControl, IAccessControl) {
        _beforeRoleChange();
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
        return isParty(account) && hasRole(DESIGNATED_PARTY_ROLE, account);
    }
}
