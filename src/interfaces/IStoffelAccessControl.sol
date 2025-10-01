pragma solidity ^0.8.13;
import "@openzeppelin/contracts/access/IAccessControl.sol";
import "@openzeppelin/contracts/interfaces/IERC5267.sol";
import "@openzeppelin/contracts/utils/introspection/IERC165.sol";

interface IStoffelAccessControl is IAccessControl, IERC165 {
    
    function supportsInterface(bytes4 interfaceId) external view virtual returns (bool);
    function grantRole(bytes32 role, address account) external virtual;
    function revokeRole(bytes32 role, address account) external virtual;

    function isParty(address account) external view virtual returns (bool);
    
}