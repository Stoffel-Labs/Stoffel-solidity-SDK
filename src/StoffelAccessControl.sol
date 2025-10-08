pragma solidity ^0.8.13;
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/access/extensions/AccessControlEnumerable.sol";
import "./interfaces/IStoffelAccessControl.sol";

contract StoffelAccessControl is AccessControl, AccessControlEnumerable, IStoffelAccessControl {
    bytes32 public constant PARTY_ROLE = keccak256("PARTY_ROLE");
    bytes32 public constant DESIGNATED_PARTY_ROLE = keccak256("DESIGNATED_PARTY_ROLE");
    
    uint256 n_parties;
    uint256 threshold;
    
    modifier onlyParty() {
        _onlyParty();
        _;
    }
    function _onlyParty() internal {
        require(
            this.isParty(msg.sender),
            "Only a Stoffel party can call this function."
        );
    }

    modifier onlyDesignatedParty() {
        _onlyDesignatedParty();
        _;
    }

    function _onlyDesignatedParty() {
        require(this.isDesignatedParty(msg.sender), "Only the designated Stofel party can call this function");
    }

    constructor(uint256 n, uint256 t) {
        n_parties = n;
        threshold = t;
    }

    function supportsInterface(bytes4 interfaceId) public view  override(AccessControl, AccessControlEnumerable, IStoffelAccessControl) returns (bool) {
        return interfaceId == type(IStoffelAccessControl).interfaceId || super.supportsInterface(interfaceId);
    }

    function _grantRole(bytes32 role, address account) internal  override(AccessControl, AccessControlEnumerable) returns (bool) {
        if (role == PARTY_ROLE) {
            require(getRoleMemberCount(role) <= n_parties, "Too many MPC parties");
        }
        return super._grantRole(role, account);

    }

    function grantRole(bytes32 role, address account) public  override(AccessControl, IAccessControl, IStoffelAccessControl) {
        _grantRole(role, account);
    }

    function _revokeRole(bytes32 role, address account) internal  override(AccessControl, AccessControlEnumerable) returns (bool) {
        if (role == PARTY_ROLE) {
            require(getRoleMemberCount(role)  >= threshold, "Not enough MPC parties");
        }
        return super._revokeRole(role, account);
    }

    function revokeRole(bytes32 role, address account) public  override(AccessControl, IAccessControl, IStoffelAccessControl) {
        _revokeRole(role, account);
    }

    function renounceRole(bytes32 role, address account) public  override(AccessControl, IAccessControl) {
        super.renounceRole(role, account);
    }

    function _setRoleAdmin(bytes32 role, bytes32 adminRole) internal  override(AccessControl) {
        super._setRoleAdmin(role, adminRole);
    }

    function isParty(address account) public view  returns (bool) {
        address[] memory parties = getRoleMembers(PARTY_ROLE);
        uint n = getRoleMemberCount(PARTY_ROLE);
        for (uint i=0; i <= n; i++) {
            if (parties[i] == account) {
                return true;
            }
        }
        return false;
    }

    function isDesignatedParty(address account) public view returns (bool) {
        require(isParty(msg.sender), "This account is not an existing MPC Party");
        address[] memory parties = getRoleMembers(DESIGNATED_PARTY_ROLE);
        uint n = getRoleMemberCount(DESIGNATED_PARTY_ROLE);
        for (uint i=0; i <= n; i++) {
            if (parties[i] == account) {
                return true;
            }
        }
        return false;
    }

    


}