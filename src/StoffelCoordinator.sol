pragma solidity ^0.8.13;

import "./StoffelAccessControl.sol";
import "./StoffelInputManager.sol";
import "./IStoffelAccessControl.sol";
import "./IStoffelInputManager.sol";

contract StoffelCoordinator is StoffelAccessControl, StoffelInputManager {
    enum Rounds {
        PreprocessingRound,
        ClientInputMaskReservationRound,
        CollectingClientInputRound,
        ClientInputsCollectionEndRound
        MPCTaskExecutionRound,
        MPCTaskExecutionEndRound,
        ClientOutputCollectionRound
    }

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

    function initCoordinator();

    function initiateMPCComputation();

    function publishPublicOutput();

    
    

}