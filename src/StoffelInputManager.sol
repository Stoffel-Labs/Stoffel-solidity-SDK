pragma solidity ^0.8.13;
import "./interfaces/IStoffelInputManager.sol";
import "./StoffelAccessControl.sol";
import "openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
import "openzeppelin-contracts/contracts/utils/cryptography/MessageHashUtils.sol";


abstract contract StoffelInputManager is StoffelAccessControl, IStoffelInputManager{
    mapping(uint256 => address) reservedInputIndices;
    uint256 nTotalIndices;
    uint256 nIndicesLeft;

    struct MaskedInput {
        uint256 index;
        uint256 maskedInput;
    }

    mapping(address => MaskedInput) clientInputs;

    event IndexBufferEvent(uint totalIndices, address designatedParty);
    event ReservedInputEvent(address client, uint reservedIndex);
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);
    
    function initialzeInputMaskBuffer(uint256 nIndicesToReserve) onlyDesignatedParty external {
        require(nTotalIndices == 0, "The index buffer has already been set");
        nTotalIndices = nIndicesToReserve;

        emit IndexBufferEvent(nTotalIndices, msg.sender);
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
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex) external override {
        require(reservedInputIndices[reservedIndex] == msg.sender, "This client did not reserve the input mask at this index"); 
        clientInputs[msg.sender] = MaskedInput(reservedIndex, maskedInput);


        emit MaskedInputEvent(msg.sender, maskedInput, reservedIndex);

        // Unreserve index once the input mask has been used
        reservedInputIndices[reservedIndex] = address(0);
    }

    // This function is meant to be called off-chain by the nodes in addition to off-chain authentication
    function authenticateClient(uint256 requestIndex, address clientAddr, bytes calldata signature) external override onlyParty returns (bool) {
        bytes32 hashedMsg = MessageHashUtils.toEthSignedMessageHash(keccak256(abi.encode(requestIndex)));
        address clientAddress = ECDSA.recover(hashedMsg, signature);

        return clientAddress == clientAddr;
    }

    
}