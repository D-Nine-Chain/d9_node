# D9 Chain Node 🔗⛓️

<div align="center">

![D9 Chain Logo](https://github.com/D-Nine-Chain/resources/blob/main/d9-logo.png?raw=true)

[![Release](https://img.shields.io/github/v/release/D-Nine-Chain/d9_node)](https://github.com/D-Nine-Chain/d9_node/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/D-Nine-Chain/d9_node/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/D-Nine-Chain/d9_node/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.75.0-orange.svg)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/substrate-polkadot--sdk-brightgreen)](https://substrate.io)

**The Future of Decentralized Community Governance**

[Documentation](./docs) | [Website](https://d9.network) | [Twitter](https://twitter.com/d9chain) | [Discord](https://discord.gg/d9chain)

</div>

## Table of Contents

- [About D9 Chain](#about-d9-chain)
  - [Key Features](#key-features)
  - [D9 Custom Pallets](#d9-custom-pallets)
- [Installation](#installation)
  - [Quick Install](#quick-install)
  - [Build from Source](#build-from-source)
- [Running a Node](#running-a-node)
  - [Development Chain](#development-chain)
  - [Mainnet Node](#mainnet-node)
  - [Validator Node](#validator-node)
- [Architecture](#architecture)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Support](#support)

## About D9 Chain

D9 Chain is a next-generation blockchain platform built with [Substrate](https://substrate.io) that revolutionizes community governance through innovative economic mechanisms and social coordination tools. By combining cutting-edge blockchain technology with human-centric design, D9 Chain enables communities to organize, collaborate, and thrive in the decentralized economy.

### Key Features

🏛️ **Community Governance** - Democratic decision-making with the D9 Council system  
💰 **Treasury Management** - Transparent fund allocation for community projects  
🔗 **Referral Network** - Built-in social graph tracking and rewards  
🗳️ **Node Voting** - Decentralized validator selection and rewards  
🔐 **Multi-Signature Support** - Secure shared control of assets  


### D9 Custom Pallets

D9 Chain features several custom pallets that provide unique functionality:

| Pallet | Description |
|--------|-------------|
| `pallet_d9_balances` | Enhanced balance management with referral support |
| `pallet_d9_referral` | Manages referral relationships and social graphs |
| `pallet_d9_treasury` | Community-controlled treasury with proposal system |
| `pallet_d9_node_voting` | Validator selection through community voting |
| `pallet_d9_node_rewards` | Distributes rewards to validators and voters |
| `pallet_d9_council_lock` | Manages council membership and voting |
| `pallet_d9_multi_sig` | Multi-signature wallet functionality |

[Learn more about our pallets →](./docs/pallets)

## Installation

### Quick Install

For Ubuntu 22.04 users, use our automated installation script:

```bash
curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/install-d9-node.sh | bash
```

This script will:
- Check system requirements
- Download the latest pre-built binary
- Configure the node as a systemd service
- Set up validator keys (optional)
- Start the node automatically

### Build from Source

#### Prerequisites

- Ubuntu 22.04 (recommended)
- At least 60GB free disk space
- 8GB RAM minimum
- Rust 1.75.0 exactly

#### Easy Build

Use our build script for automated compilation:

```bash
curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/build-node.sh | bash
```

#### Manual Build

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

2. **Install Dependencies**
```bash
sudo apt update
sudo apt install build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
```

3. **Clone and Build**
```bash
git clone https://github.com/D-Nine-Chain/d9_node.git
cd d9_node
cargo build --release
```

## Running a Node

### Development Chain

Start a single-node development chain:

```bash
./target/release/d9-node --dev
```

### Mainnet Node

Run a full node connected to the D9 mainnet:

```bash
./target/release/d9-node \
  --base-path /home/ubuntu/node-data \
  --chain ./new-main-spec.json \
  --name "MyD9Node" \
  --port 30333 \
  --rpc-port 9944 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

### Validator Node

To run a validator node, you'll need to:

1. Set up your node with validator keys
2. Bond D9 tokens
3. Set session keys
4. Validate

[Detailed validator guide →](./docs/running-as-a-validator.md)

## Architecture

D9 Chain is built using the Substrate framework and consists of:

- **Runtime**: Core blockchain logic including custom pallets
- **Node**: Network layer handling peer connections and consensus
- **RPC**: APIs for interacting with the blockchain

[Architecture documentation →](./docs/architecture)

## Documentation

- [Building a Node](./docs/building-a-node.md)
- [Running as a Validator](./docs/running-as-a-validator.md)
- [Staking Guide](./docs/staking.md)
- [Runtime Upgrades](./docs/runtime-upgrade.md)
- [Custom Pallets](./docs/pallets)
- [RPC Extensions](./docs/extending_rpc.md)

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support


- 📚 [Documentation](./docs)
- 🐛 [Issue Tracker](https://github.com/D-Nine-Chain/d9_node/issues)

---

<div align="center">
Built with ❤️ by the D9 Community
</div>