pragma solidity ^0.8.13;
import "@openzeppelin/contracts/access/IAccessControl.sol";
import "@openzeppelin/contracts/access/extensions/IAccessControlDefaultAdminRules.sol";
import "@openzeppelin/contracts/access/extensions/IAccessControlEnumerable.sol";

interface IStoffelAccessControl is IAccessControl, IAccessControlDefaultAdminRules, IAccessControlEnumerable {
    
    function supportsInterface(bytes4 interfaceId) external view virtual returns (bool);
    function grantRole(bytes32 role, address account) external virtual;
    function revokeRole(bytes32 role, address account) external virtual;
    

}