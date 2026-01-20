# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build the project
forge build

# Build with contract sizes displayed
forge build --sizes

# Run all tests
forge test

# Run tests with verbose output
forge test -vvv

# Run a single test file
forge test --match-path test/StoffelAccessControl.t.sol

# Run a single test function
forge test --match-test test_grantPartyRole

# Format code
forge fmt

# Check formatting (CI uses this)
forge fmt --check
```

## Architecture

This is a Solidity SDK for coordinating MPC (Multi-Party Computation) on-chain using the Foundry framework. The system manages MPC nodes, client inputs, and computation coordination through a round-based state machine.

### Core Components

**StoffelCoordinator** (`src/StoffelCoordinator.sol`)
- Abstract contract that orchestrates the MPC workflow
- Implements a 7-phase round-based state machine: PreprocessingRound → ClientInputMaskReservationRound → CollectingClientInputRound → ClientInputsCollectionEndRound → MPCTaskExecutionRound → MPCTaskExecutionEndRound → ClientOutputCollectionRound
- Uses `atRound` modifier for phase gating and `timedRoundTransition` modifiers for time-based transitions
- Constructor takes: stoffelProgramHash, n (parties), t (threshold), designatedParty, and initial MPC nodes

**StoffelAccessControl** (`src/StoffelAccessControl.sol`)
- Extends OpenZeppelin's AccessControl and AccessControlEnumerable
- Defines two roles: `PARTY_ROLE` (MPC nodes) and `DESIGNATED_PARTY_ROLE` (privileged party)
- Enforces n-of-t threshold constraints: cannot exceed n parties, cannot go below threshold t
- `onlyParty` and `onlyDesignatedParty` modifiers for access control

**StoffelInputManager** (`src/StoffelInputManager.sol`)
- Abstract contract managing client input submission for MPC
- Input mask reservation system: clients reserve indices, then submit masked inputs
- ECDSA signature verification via `authenticateClient` for off-chain authentication
- Emits events for index buffer initialization, reservations, and masked input submissions

### Dependencies

- OpenZeppelin Contracts (AccessControl, ECDSA, MessageHashUtils, Ownable)
- forge-std (testing)
- dstack (submodule - TEE/confidential computing infrastructure)

## Rust Bindings

The `bindings/` directory contains a Rust crate with type-safe bindings for all Stoffel contracts, generated using Foundry's `forge bind` with the Alloy framework.

### Building and Using Bindings

```bash
# Generate Rust bindings from contracts
make bindings

# Verify bindings compile
make check-bindings

# Generate Rust documentation
cd bindings && cargo doc --open
```

### Available Bindings

- `StoffelCoordinator` - Main MPC coordination contract
- `StoffelAccessControl` - Role-based access control
- `StoffelInputManager` - Client input management
- `IStoffelCoordinator`, `IStoffelAccessControl`, `IStoffelInputManager` - Interfaces

### Regenerating After Contract Changes

After modifying contracts in `src/`, regenerate bindings:

```bash
make bindings
cd bindings && cargo check
```

The bindings are committed to the repository so downstream consumers don't need Foundry installed.
