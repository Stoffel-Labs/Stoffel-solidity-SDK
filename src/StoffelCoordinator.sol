pragma solidity ^0.8.13;

import "./StoffelAccessControl.sol";
import "./StoffelInputManager.sol";
import "./IStoffelAccessControl.sol";
import "./IStoffelInputManager.sol";

abstract contract StoffelCoordinator is StoffelAccessControl, StoffelInputManager {
    enum Rounds {
        PreprocessingRound,
        ClientInputMaskReservationRound,
        CollectingClientInputRound,
        ClientInputsCollectionEndRound
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

    mapping (address => Inputs) clientInputs;


    event CoordinatorInitialized(address coordinator, uint timeofInitialization, address designatedParty);
    event PreprocessingRoundEnded();
    event MPCTaskExecuted(bytes32 stoffelProgramHash, address executor, uint timeOfExecution);

    bytes32 _stoffelLangProgramHash;
    uint256 public creationTime = block.timestamp;

    Rounds round = Rounds.PreprocessingRound;

    modifier atRound(Rounds _round) {
        require(_round == round);
        _;
    }

    function nextRound() internal {
        round = Rounds(uint(round) + 1);
    }

    function goToRound(Round _round) internal {
        round = _round;
    }

    modifier timedRoundTransition(Rounds transitionRound, uint whenToTransition) {
        if (round == transitionRound && (block.timestamp >= creationTime + whenToTransition)) {
            nextRound();
        }
        _;

    }

    modifier timedRoundTransitionGoto(Rounds transitionRound, Rounds gotoRound, uint whenToTransition) {
        if (round == transitionRound && (block.timestamp >= creationTime + whenToTransition)) {
            goToRound(gotoRound);
        }
        _;
    }

    /* Need to initiate the following
      - Set designated party
      - Set MPC program hash
      - grant party role to initial set of MPC nodes
    */
    function initCoordinator() {
        
    };

    /**
     * Preprocessing can be started by the designated party 
     * Depending on the trust model of the contract that instantiates this abstract contract
     * the designated party can set the input mask indices or each party can contribute to setting
     * the input mask indices.
     */
    function startPreprocessing() atRound(Rounds.PreprocessingRound);

    /**
     * Once the indices have been set, clients should now be able to reserve an index
     * for an input mask that they can request for off-chain from the MPC nodes. 
     * Once reserved, the clients can now post their public and masked inputs to the resulting app.
     */
    function gatherInputs();

    /**
     * Once enough inputs have been collected according to the application's logic, the MPC computation should be initiated
     */
    function initiateMPCComputation() atRound(Rounds.ClientInputsCollectionEndRound);

    /**
     * Once the computation has been completed by the MPC nodes off-chain
     * The resulting public outputs are posted on-chain and the shares are sent directly to the client for them to reconstruct.
     * The Outputs struct keeps track of whether a particular party has sent the final share back to a client
     */
    function publishOutputs();

    
    

}