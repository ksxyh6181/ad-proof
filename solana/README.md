# AdProof Solana Smart Contracts

This directory contains the Solana smart contracts for the AdProof project, built with Anchor framework.

## Project Structure

```
credential-registry/
├── Anchor.toml          # Anchor configuration
├── programs/
│   └── credential-registry/ # Program code
│       ├── src/
│       │   └── lib.rs     # Main program code
│       └── Cargo.toml   
└── tests/                # Test files
    └── credential-registry.ts  # Tests for the program
```

## Smart Contract Overview

The Credential Registry program allows for:

1. Storing credential metadata on-chain
2. Verifying credential existence and validity
3. Managing credential lifecycle (issuance, revocation)
4. Querying credential history

## Development Setup

### Prerequisites

- Rust
- Solana CLI
- Anchor Framework

### Installation

1. Install Solana CLI: https://docs.solana.com/cli/install-solana-cli-tools
2. Install Anchor Framework: https://www.anchor-lang.com/docs/installation
3. Build the program: `anchor build`

### Testing

Run the tests with:

```
anchor test
```

## Deployment

To deploy to devnet:

```
anchor deploy --provider.cluster devnet
```

For mainnet:

```
anchor deploy --provider.cluster mainnet
```

## Integration with AdProof Server

The AdProof server interacts with this Solana program through RPC calls to:
- Register new credentials
- Verify existing credentials
- Query credential metadata

See the integration code in the server codebase for implementation details.
