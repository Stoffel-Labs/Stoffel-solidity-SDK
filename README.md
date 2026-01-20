# Stoffel Solidity SDK

A Solidity SDK for coordinating Multi-Party Computation (MPC) applications on-chain. This SDK provides the smart contract infrastructure for managing MPC node registration, client input submission, and computation coordination.

## Overview

The Stoffel Solidity SDK enables developers to build on-chain MPC applications by providing:

- **Role-based access control** for MPC parties and designated orchestrators
- **Input masking system** for privacy-preserving client input submission
- **Round-based state machine** for coordinating the MPC lifecycle
- **Byzantine fault tolerance** through configurable n/t threshold settings

## Installation

### Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation)

### Install via Forge

```bash
forge install Stoffel-Labs/Stoffel-solidity-SDK
```

### Add to remappings

```toml
# foundry.toml
[profile.default]
remappings = [
    "@stoffel/=lib/Stoffel-solidity-SDK/src/"
]
```

## Architecture

The SDK consists of three core contracts:

```
┌─────────────────────────────────────────────────────────────┐
│                    StoffelCoordinator                       │
│         (Abstract - Extend for your application)            │
├─────────────────────────────────────────────────────────────┤
│  - Round state machine (7 phases)                           │
│  - 4 abstract methods to implement                          │
│  - Event emission for MPC node coordination                 │
└───────────────────────┬─────────────────────────────────────┘
                        │ inherits
        ┌───────────────┴───────────────┐
        ▼                               ▼
┌───────────────────┐         ┌───────────────────┐
│ StoffelAccessControl│       │ StoffelInputManager│
├───────────────────┤         ├───────────────────┤
│ - PARTY_ROLE      │         │ - Input mask      │
│ - DESIGNATED_     │         │   reservation     │
│   PARTY_ROLE      │         │ - Masked input    │
│ - n/t threshold   │         │   submission      │
│   enforcement     │         │ - ECDSA auth      │
└───────────────────┘         └───────────────────┘
```

### StoffelCoordinator

Abstract contract that orchestrates the MPC workflow. Provides:
- 7-phase round state machine
- Timed round transitions
- 4 abstract methods for customization

### StoffelAccessControl

Manages MPC party roles:
- `PARTY_ROLE` - MPC compute nodes
- `DESIGNATED_PARTY_ROLE` - Orchestration privileges
- Enforces `n >= 3t + 1` for Byzantine fault tolerance

### StoffelInputManager

Handles client input submission:
- Input mask index reservation
- Masked input storage
- ECDSA signature verification for off-chain authentication

## Quick Start

### 1. Create Your Contract

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {StoffelCoordinator} from "@stoffel/StoffelCoordinator.sol";

contract MyMPCApp is StoffelCoordinator {
    constructor(
        bytes32 programHash,
        uint256 n,
        uint256 t,
        address designatedParty,
        address[] memory mpcNodes
    ) StoffelCoordinator(programHash, n, t, designatedParty, mpcNodes) {}

    function startPreprocessing() external override onlyDesignatedParty atRound(Round.PreprocessingRound) {
        // Initialize input mask buffer
        _nextRound();
    }

    function gatherInputs() external override onlyDesignatedParty atRound(Round.ClientInputMaskReservationRound) {
        // Transition to input collection
        _nextRound();
    }

    function initiateMPCComputation() external override onlyDesignatedParty atRound(Round.ClientInputsCollectionEndRound) {
        // Trigger off-chain MPC execution
        _nextRound();
    }

    function publishOutputs() external override onlyDesignatedParty atRound(Round.MPCTaskExecutionEndRound) {
        // Store computation results
        _nextRound();
    }
}
```

### 2. Deploy

```solidity
address[] memory nodes = new address[](5);
nodes[0] = 0x1111111111111111111111111111111111111111;
// ... add remaining nodes

MyMPCApp app = new MyMPCApp(
    keccak256("my-stoffel-program"),  // Program hash
    5,                                 // n parties
    1,                                 // t threshold
    msg.sender,                        // Designated party
    nodes                              // MPC nodes
);
```

## Round State Machine

The coordinator follows a 7-phase lifecycle:

```
PreprocessingRound                    ← startPreprocessing()
        │
        ▼
ClientInputMaskReservationRound       ← gatherInputs()
        │
        ▼
CollectingClientInputRound            ← Clients submit masked inputs
        │
        ▼
ClientInputsCollectionEndRound        ← initiateMPCComputation()
        │
        ▼
MPCTaskExecutionRound                 ← Off-chain MPC runs
        │
        ▼
MPCTaskExecutionEndRound              ← publishOutputs()
        │
        ▼
ClientOutputCollectionRound           ← Clients retrieve results
```

### Round Modifiers

```solidity
// Ensure correct round
modifier atRound(Round _round);

// Advance to next round
modifier nextRound();

// Jump to specific round
modifier goToRound(Round _round);

// Time-based transitions
modifier timedRoundTransition(Round transitionRound, uint whenToTransition);
```

## Client Input Flow

### 1. Reserve Input Mask

```solidity
// Client reserves index 5
coordinator.reserveInputMask(5);
```

### 2. Get Mask Off-Chain

```javascript
// Client requests mask from MPC nodes (off-chain)
const mask = await mpcNode.getInputMask(5);
```

### 3. Submit Masked Input

```solidity
// Client submits: maskedInput = secretInput + mask
coordinator.submitMaskedInput(maskedValue, 5);
```

## Development

### Build

```bash
forge build
```

### Test

```bash
# Run all tests
forge test

# Verbose output
forge test -vvv

# Run specific test
forge test --match-test test_startPreprocessing
```

### Format

```bash
forge fmt
```

### Contract Sizes

```bash
forge build --sizes
```

## Configuration

### Threshold Requirements

HoneyBadger MPC requires `n >= 3t + 1`:

| Parties (n) | Threshold (t) | Tolerates |
|-------------|---------------|-----------|
| 4           | 1             | 1 faulty  |
| 5           | 1             | 1 faulty  |
| 7           | 2             | 2 faulty  |
| 10          | 3             | 3 faulty  |

### Constructor Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `stoffelProgramHash` | `bytes32` | Hash of the StoffelLang program |
| `n` | `uint256` | Total number of MPC parties |
| `t` | `uint256` | Fault tolerance threshold |
| `designatedParty` | `address` | Address with orchestration privileges |
| `initialMPCNodes` | `address[]` | Initial MPC node addresses |

## Security Considerations

1. **Access Control**: Always use `onlyDesignatedParty` and `onlyParty` modifiers
2. **Round Enforcement**: Use `atRound` modifier to prevent state manipulation
3. **Threshold Maintenance**: Never allow party count below `t + 1`
4. **Input Validation**: Validate reserved indices before accepting inputs
5. **Mask Security**: Each mask index can only be used once

See [Security Best Practices](https://docs.stoffel.dev/solidity-sdk/security) for comprehensive guidelines.

## Dependencies

- [OpenZeppelin Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts) - Access control, ECDSA, Ownable
- [forge-std](https://github.com/foundry-rs/forge-std) - Testing utilities

## Documentation

- [Template Guide](https://docs.stoffel.dev/solidity-sdk/template-guide) - Step-by-step implementation
- [StoffelCoordinator Reference](https://docs.stoffel.dev/solidity-sdk/coordinator) - Full API documentation
- [Access Control](https://docs.stoffel.dev/solidity-sdk/access-control) - Role management
- [Input Manager](https://docs.stoffel.dev/solidity-sdk/input-manager) - Client input handling

## License

MIT
