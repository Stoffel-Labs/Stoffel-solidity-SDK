# stoffel-solidity-bindings

Type-safe Rust bindings for Stoffel Solidity contracts, generated using Foundry's `forge bind` with the Alloy framework.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
stoffel-solidity-bindings = { git = "https://github.com/stoffel-labs/Stoffel-solidity-SDK", branch = "main" }
```

Or with on-chain interaction support:

```toml
[dependencies]
stoffel-solidity-bindings = { git = "https://github.com/stoffel-labs/Stoffel-solidity-SDK", branch = "main", features = ["providers"] }
```

## Usage

### Basic Type Usage

```rust
use stoffel_solidity_bindings::{StoffelCoordinator, Address, U256};

// Access contract types, events, and errors
let coordinator_address: Address = "0x...".parse().unwrap();
```

### Contract Interaction (with `providers` feature)

```rust
use stoffel_solidity_bindings::StoffelCoordinator;
use alloy_provider::ProviderBuilder;

// Create a provider
let provider = ProviderBuilder::new().on_http("http://localhost:8545".parse().unwrap());

// Interact with deployed contract
let contract = StoffelCoordinator::new(address, provider);
```

## Available Bindings

The following contracts have Rust bindings:

- `StoffelCoordinator` - Main MPC coordination contract
- `StoffelAccessControl` - Role-based access control
- `StoffelInputManager` - Client input management

### Interfaces

- `IStoffelCoordinator`
- `IStoffelAccessControl`
- `IStoffelInputManager`

## Regenerating Bindings

If you're developing on the Solidity contracts, regenerate bindings after changes:

```bash
# From the Stoffel-solidity-SDK root
make bindings

# Verify compilation
cd bindings && cargo check
```

## Features

| Feature | Description |
|---------|-------------|
| `default` | Core types only (no network dependencies) |
| `providers` | Enables `alloy-contract` and `alloy-provider` for on-chain interaction |

## Requirements

For regenerating bindings:
- Foundry (with `forge bind` support)
- Rust 1.70+

For using the crate:
- Rust 1.70+
- alloy 1.0+
