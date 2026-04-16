// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStoffelAccessControl} from "./IStoffelAccessControl.sol";
import {IStoffelInputManager} from "./IStoffelInputManager.sol";

/// @title IStoffelCoordinator
/// @author Stoffel Labs
/// @notice Combined interface for the MPC coordinator contract
/// @dev Inherits from IStoffelAccessControl for party management and
///      IStoffelInputManager for client input handling. Implementations
///      should also define the four lifecycle methods: startPreprocessing,
///      gatherInputs, initiateMPCComputation, and publishOutputs.
interface IStoffelCoordinator is IStoffelAccessControl, IStoffelInputManager {}
