pragma solidity ^0.8.13;

interface IStoffelInputManager {
    function reserveInputMask(uint indexToReserve) external virtual;
    function currentlyAvailableInputMasks() external view virtual returns (uint256);
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndexed) external virtual;
    function authenticateClient(uint256 requestIndex, address clientAddr, bytes calldata signature) external virtual returns (bool);
}