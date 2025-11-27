pragma solidity ^0.8.13;

import "./StoffelAccessControl.sol";
import "./StoffelInputManager.sol";
import "./interfaces/IStoffelAccessControl.sol";
import "./interfaces/IStoffelInputManager.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

abstract contract StoffelCoordinator is StoffelAccessControl, StoffelInputManager, Ownable {
    enum Round {
        PreprocessingRound,
        ClientInputMaskReservationRound,
        CollectingClientInputRound,
        ClientInputsCollectionEndRound,
        MPCTaskExecutionRound,
        MPCTaskExecutionEndRound,
        ClientOutputCollectionRound
    }

    struct Inputs {
        bytes publicInputs;
        MaskedInput[] maskedInputs;
    }

    struct Outputs {
        bytes publicOutputs;
        mapping(address => mapping (address => bool)) sharesReceived;
    }

    event CoordinatorInitialized(address coordinator, uint timeofInitialization, address designatedParty);
    event PreprocessingRoundExecuted(address designatedParty, uint timeOfExecution);
    event ClientInputMaskReversationEvent(address executor, uint timeOfExecution);
    
    event MPCTaskExecuted(bytes32 stoffelProgramHash, address executor, uint timeOfExecution);

    bytes32 _stoffelProgramHash;
    uint256 public creationTime;

    Round round;

    modifier atRound(Round _round) {
        _atRound(_round);
        _;
    }

    modifier nextRound() {
        _nextRound();
        _;
    }

    modifier goToRound(Round _round) {
        _goToRound(_round);
        _;
    }

    function _atRound(Round _round) internal {
        require(round == _round);
    }

    function _nextRound() internal {
        round = Round(uint(round) + 1);
    }

    function _goToRound(Round _round) internal {
        round = _round;
    }

    function _timedRoundTransition(Round transitionRound, uint whenToTransition) internal {
        if (round == transitionRound && (block.timestamp >= creationTime + whenToTransition)) {
            _nextRound();
        }
    }

    function _timedRoundTransitionGoTo(Round transitionRound, Round gotoRound, uint whenToTransition) internal {
        if (round == transitionRound && block.timestamp >= whenToTransition) {
            _goToRound(gotoRound);
        }
    }

    modifier timedRoundTransition(Round transitionRound, uint whenToTransition) {
        _timedRoundTransition(transitionRound, whenToTransition);
        _;

    }

    modifier timedRoundTransitionGoto(Round transitionRound, Round gotoRound, uint whenToTransition) {
        _timedRoundTransitionGoTo(transitionRound, gotoRound, whenToTransition);
        _;
    }

    /* Need to initiate the following
      - Set designated party
      - Set MPC program hash
      - grant party role to initial set of MPC nodes
    */
    constructor(bytes32 stoffelProgramHash, uint256 n, uint256 t, address designatedParty, address[] memory initialMPCNodes) StoffelAccessControl(n, t) Ownable(msg.sender) {
        _stoffelProgramHash = stoffelProgramHash;
        _grantRole(DESIGNATED_PARTY_ROLE, designatedParty);
        for (uint i = 0; i < initialMPCNodes.length; i++) {
            _grantRole(PARTY_ROLE, initialMPCNodes[i]);
        }

        creationTime = block.timestamp;

        emit CoordinatorInitialized(address(this), creationTime, designatedParty);
    }

    /**
     * Preprocessing can be started by the designated party 
     * Depending on the trust model of the contract that instantiates this abstract contract
     * the designated party can set the input mask indices or each party can contribute to setting
     * the input mask indices.
     * 
     * Should be used in conjuction with the atRound modififer
     */
    function startPreprocessing() virtual external;

    /**
     * Once the indices have been set, clients should now be able to reserve an index
     * for an input mask that they can request for off-chain from the MPC nodes. 
     * Once reserved, the clients can now post their public and masked inputs to the resulting app.
     */
    function gatherInputs() virtual external;

    /**
     * Once enough inputs have been collected according to the application's logic, the MPC computation should be initiated
     * Should be used in conjunction with the atRound modifier
     */
    function initiateMPCComputation() virtual external;

    /**
     * Once the computation has been completed by the MPC nodes off-chain
     * The resulting public outputs are posted on-chain and the shares are sent directly to the client for them to reconstruct.
     * The Outputs struct keeps track of whether a particular party has sent the final share back to a client
     */
    function publishOutputs() virtual external;

    
    

}