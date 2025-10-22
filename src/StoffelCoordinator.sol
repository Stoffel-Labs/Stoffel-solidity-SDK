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

    modifier timedRoundTransition(Rounds transitionRound, uint whenToTransition) {
        if (round == transitionRound && (block.timestamp >= creationTime + whenToTransition)) {
            nextRound();
        }

    }

    function initCoordinator() {}

    

}