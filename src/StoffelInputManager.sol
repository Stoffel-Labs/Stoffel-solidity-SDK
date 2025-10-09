pragma solidity ^0.8.13;
import "./interfaces/IStoffelInputManager.sol";
import "./StoffelAccessControl.sol";

abstract contract StoffelInputManager is StoffelAccessControl, IStoffelInputManager{
    mapping(uint256 => address) reservedInputIndices;
    uint256 nTotalIndices;
    uint256 nIndicesLeft;

    event IndexBufferEvent(uint totalIndices, address designatedParty);
    event ReservedInputEvent(address client, uint reservedIndex);
    
    function initialzeInputMaskBuffer(uint256 nIndicesToReserve) onlyDesignatedParty external {
        require(nTotalIndices == 0, "The index buffer has already been set");
        nTotalIndices = nIndicesToReserve;

        emit IndexBuffer(ntotalIndices, msg.sender);
    }

    function reserveInputMask(uint indexToReserve) external override {
        require(reservedInputIndices[indexToReserve] != address(0), "This index has already been reserved");
        require(nIndicesLeft > 0, "No more indices to reserve");

        reservedInputIndices[indexToReserve] = msg.sender;
        nIndicesLeft--;

        emit ReservedInputEvent(msg.sender, indexToReserve);

    }

    function currentlyAvailableInputMasks() external view override returns (uint256) {
        return nIndicesLeft;
    }
    function submitMaskedInput() external override {

    }

    function authenticateClient() external override {

    }

    
}