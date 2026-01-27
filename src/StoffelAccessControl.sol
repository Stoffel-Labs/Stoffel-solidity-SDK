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

    /// @notice Maximum number of MPC parties allowed
    /// @dev Must satisfy n >= 3t + 1 for HoneyBadger Byzantine fault tolerance
    uint256 internal n_parties;

    /// @notice Fault tolerance threshold
    /// @dev Number of faulty/malicious parties the system can tolerate
    uint256 internal threshold;

    /// @notice Mapping to track party membership
    /// @dev True if the address has been granted PARTY_ROLE
    mapping(address => bool) public is_party;

    /// @notice Emitted when the access control is initialized
    /// @param n Maximum number of parties
    /// @param t Fault tolerance threshold
    /// @param initializer Address that deployed the contract
    event InitializeStoffelAccessControl(uint256 n, uint256 t, address initializer);

    /// @notice Modifier restricting access to addresses with PARTY_ROLE
    /// @dev Reverts if caller is not a registered MPC party
    modifier onlyParty() {
        _onlyParty();
        _;
    }

    /// @notice Internal function to verify caller has PARTY_ROLE
    /// @dev Called by onlyParty modifier
    function _onlyParty() internal view {
        require(this.isParty(msg.sender), "Only a Stoffel party can call this function.");
    }

    /// @notice Modifier restricting access to the designated party
    /// @dev Reverts if caller does not have DESIGNATED_PARTY_ROLE
    modifier onlyDesignatedParty() {
        _onlyDesignatedParty();
        _;
    }

    /// @notice Internal function to verify caller has DESIGNATED_PARTY_ROLE
    /// @dev Called by onlyDesignatedParty modifier
    function _onlyDesignatedParty() internal view {
        require(this.isDesignatedParty(msg.sender), "Only the designated Stofel party can call this function");
    }

    /// @notice Initializes the access control with party count and threshold
    /// @param n Maximum number of MPC parties (must satisfy n >= 3t + 1)
    /// @param t Fault tolerance threshold
    /// @dev Emits InitializeStoffelAccessControl event on deployment
    constructor(uint256 n, uint256 t) {
        n_parties = n;
        threshold = t;

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
        if (role == PARTY_ROLE) {
            require(getRoleMemberCount(role) < n_parties, "Too many MPC parties");
            is_party[account] = true;
        }
        return super._grantRole(role, account);
    }

    /// @notice Grants a role to an account
    /// @param role The role identifier to grant
    /// @param account The address to receive the role
    /// @dev Public wrapper for _grantRole with access control enforcement
    function grantRole(bytes32 role, address account)
        public
        override(AccessControl, IAccessControl, IStoffelAccessControl)
    {
        _grantRole(role, account);
    }

    /// @notice Internal function to revoke a role from an account
    /// @param role The role identifier to revoke
    /// @param account The address to lose the role
    /// @return True if the role was revoked
    /// @dev Enforces minimum party count (threshold) when revoking PARTY_ROLE
    function _revokeRole(bytes32 role, address account)
        internal
        override(AccessControl, AccessControlEnumerable)
        returns (bool)
    {
        if (role == PARTY_ROLE) {
            require(getRoleMemberCount(role) > threshold, "Not enough MPC parties");
            is_party[account] = false;
        }
        return super._revokeRole(role, account);
    }

    /// @notice Revokes a role from an account
    /// @param role The role identifier to revoke
    /// @param account The address to lose the role
    /// @dev Public wrapper for _revokeRole with threshold enforcement
    function revokeRole(bytes32 role, address account)
        public
        override(AccessControl, IAccessControl, IStoffelAccessControl)
    {
        _revokeRole(role, account);
    }

    /// @notice Allows an account to renounce their own role
    /// @param role The role identifier to renounce
    /// @param account The address renouncing the role (must be msg.sender)
    /// @dev Inherited from AccessControl, allows self-removal from roles
    function renounceRole(bytes32 role, address account) public override(AccessControl, IAccessControl) {
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
        return is_party[account];
    }

    /// @notice Checks if an address is the designated party
    /// @param account The address to check
    /// @return True if the account has DESIGNATED_PARTY_ROLE
    /// @dev Requires the caller to be an existing party
    function isDesignatedParty(address account) public view returns (bool) {
        require(isParty(msg.sender), "This account is not an existing MPC Party");
        address[] memory parties = getRoleMembers(DESIGNATED_PARTY_ROLE);
        uint256 n = getRoleMemberCount(DESIGNATED_PARTY_ROLE);
        for (uint256 i = 0; i < n; i++) {
            if (parties[i] == account) {
                return true;
            }
        }
        return false;
    }
}
