pragma solidity ^0.8.13;
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/access/extensions/AccessControlEnumerable.sol";
import "@openzeppelin/contracts/access/extensions/AccessControlDefaultAdminRules.sol";
import "./interfaces/IStoffelAccessControl.sol";
contract StoffelAccessControl is AccessControl, AccessControlDefaultAdminRules, AccessControlEnumerable, IStoffelAccessControl {
    bytes32 public constant PARTY_ROLE = keccak256("PARTY_ROLE");

    
    uint256 n_parties;
    uint256 threshold;
    

    constructor(uint256 n, uint256 t, uint48 initialDelay) AccessControlDefaultAdminRules(initialDelay, msg.sender){
        n_parties = n;
        threshold = t;
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override(AccessControl, AccessControlDefaultAdminRules, AccessControlEnumerable, IStoffelAccessControl) returns (bool) {
        return interfaceId == type(IStoffelAccessControl).interfaceId || super.supportsInterface(interfaceId);
    }

    function _grantRole(bytes32 role, address account) internal virtual override(AccessControl, AccessControlDefaultAdminRules, AccessControlEnumerable) returns (bool) {
        require(getRoleMemberCount(role) <= n_parties, "Too many MPC parties");
        return super._grantRole(role, account);

    }

    function grantRole(bytes32 role, address account) public virtual override(AccessControl, AccessControlDefaultAdminRules, IAccessControl, IStoffelAccessControl) {
        _grantRole(role, account);
    }

    function _revokeRole(bytes32 role, address account) internal virtual override(AccessControl, AccessControlDefaultAdminRules, AccessControlEnumerable) returns (bool) {
        require(getRoleMemberCount(role)  >= threshold, "Not enough MPC parties");
        return super._revokeRole(role, account);
    }

    function revokeRole(bytes32 role, address account) public virtual override(AccessControl, AccessControlDefaultAdminRules, IAccessControl, IStoffelAccessControl) {
        _revokeRole(role, account);
    }

    function renounceRole(bytes32 role, address account) public virtual override(AccessControl, AccessControlDefaultAdminRules, IAccessControl) {
        super.renounceRole(role, account);
    }

    function _setRoleAdmin(bytes32 role, bytes32 adminRole) internal virtual override(AccessControl, AccessControlDefaultAdminRules) {
        super._setRoleAdmin(role, adminRole);
    }



}