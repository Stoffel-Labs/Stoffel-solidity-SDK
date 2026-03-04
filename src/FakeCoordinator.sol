pragma solidity ^0.8.13;

import "./StoffelAccessControl.sol";
import "./StoffelInputManager.sol";
import "./StoffelCoordinator.sol";
import "./interfaces/IStoffelAccessControl.sol";
import "./interfaces/IStoffelInputManager.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

// TODO: perhaps better to make nextRound switch to the next round after the body of the function, not before

// Expects two clients to get an input.

contract FakeCoordinator is StoffelCoordinator {
    constructor(bytes32 stoffelProgramHash, uint256 n, uint256 t, address designatedParty, address[] memory initialMPCNodes, uint256 nInputs) StoffelCoordinator(stoffelProgramHash, t, initialMPCNodes, nInputs) {
	creationTime = block.timestamp;
	creationBlock = block.number;
	emit CoordinatorInitialized(address(this), creationTime, creationBlock, msg.sender);
    }

    // make nodes do the preprocessing
    function startPreprocessing() external override onlyRole(DESIGNATED_PARTY_ROLE) atRound(Round.Idle) nextRound {
        emit PreprocessingStarted(msg.sender, block.timestamp);
    }

    function reserveInputMasks() external override onlyRole(DESIGNATED_PARTY_ROLE) atRound(Round.Preprocessing) nextRound {
        emit InputMaskReservationStarted(msg.sender, block.timestamp);
    }

    function collectInputs() external override onlyRole(DESIGNATED_PARTY_ROLE) atRound(Round.InputMaskReservation) nextRound {
        emit InputCollectionStarted(msg.sender, block.timestamp);
    }

    function startMPC() external override onlyRole(DESIGNATED_PARTY_ROLE) atRound(Round.InputCollection) nextRound {
        emit MPCStarted(msg.sender, block.timestamp);
    }

    function sendOutputs() external override onlyRole(DESIGNATED_PARTY_ROLE) atRound(Round.MPC) nextRound {
	emit OutputSendingStarted(msg.sender, block.timestamp);
    }

    function finalize() external override onlyRole(DESIGNATED_PARTY_ROLE) atRound(Round.Output) goToRound(Round.Idle) {
        emit ExecutionDone(msg.sender, block.timestamp);
    }
}
