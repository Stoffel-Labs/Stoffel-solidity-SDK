pragma solidity ^0.8.13;
import "./IStoffelAccessControl.sol";
import "./IStoffelInputManager.sol";

interface IStoffelCoordinator {
    enum Rounds {
        PreprocessingRound,
        ClientInputMaskReservationRound,
        CollectingClientInputRound,
        MPCTaskExecutionRound,
        ClientOutputCollectionRound
    }

    Rounds round = Rounds.PreprocessingRound;

    modifier atRound(Rounds _round) {
        require(_round == round);
        _;
    }
}