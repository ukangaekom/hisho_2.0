<div align="center">
  <h1>🐜 Hisho 2.0</h1>
  <p><strong>Terminal-native AI agent for multi-chain on-chain finance</strong></p>
</div>

Hisho 2.0 is a Rust-powered blockchain copilot built for the terminal. It combines a native command-line interface, secure local wallet storage, and Google Gemini-driven intent parsing to help users interact with Web3 networks using natural language instead of custom scripts or browser wallets.

This project is intentionally EVM-first. The live chain registry is loaded from a centralized configuration file and supports a large set of mainnets and testnets across Ethereum-compatible ecosystems.

---

## Overview

Hisho gives you a direct way to:

- switch between supported blockchain networks
- inspect native balances and token balances
- query token and NFT metadata
- work with the configured wallet and secure vault
- use a conversational AI layer to translate natural-language prompts into structured operations

The current implementation is built around a terminal REPL, a settings wizard, an OS-keyring-backed wallet vault, and a dynamic chain registry that is refreshed through the project configuration rather than hard-coded runtime filters.

---

## Current implementation status

The project is not a generic multi-chain abstraction layer for every blockchain. It is currently focused on EVM and EVM-compatible chains, with the active network list managed in the chain registry.

### Verified project reality

- The app is a Rust CLI with commands for `start`, `status`, and `settings`.
- Default chain configuration is loaded from `chain.json`.
- Chain matching logic is implemented in `src/settings/chain.rs` for exact matches, normalized names, native token symbols, and fuzzy matching.
- Wallet storage and key protection are handled in `src/settings/storage.rs` and `src/settings/wallet.rs`.
- Token ingestion for the database is handled in `src/database/chain_data.rs`.
- The project uses Gemini for AI intent parsing and tool orchestration.

### Current network coverage

The project currently includes 68 mainnet/testnet records in the active chain registry, including:

- Ethereum
- BNB Smart Chain
- Polygon
- Avalanche
- Arbitrum One
- Optimism
- Base
- Sonic (formerly Fantom)
- Cronos
- Gnosis Chain
- Celo
- Moonbeam
- zkSync Era
- Linea
- Scroll
- Mantle
- Blast
- opBNB
- Metis
- Kava
- Core
- Polygon zkEVM
- Aurora
- Kaia
- Manta Pacific
- Mode
- Berachain
- X Layer
- Moonriver
- Arc
- Monad
- MegaETH
- HyperEVM
- Abstract
- Ronin
- ApeChain
- Plume
- Lens
- B3
- Zircuit
- Immutable zkEVM
- Morph
- TAC
- Chiliz Chain
- Flow EVM
- BOB
- Flare
- Ethereum Classic
- Hedera
- Filecoin EVM
- LUKSO
- Sei EVM
- Astar
- Canto
- Injective EVM
- Harmony
- IoTeX
- Telos EVM
- XDC Network
- Oasis Sapphire
- Unichain
- World Chain
- Ink
- Soneium
- Fraxtal
- Rootstock
- Zora
- Taiko

This registry is stored in `chain.json` and is the source of truth for available networks and RPC endpoints.

---

## Features

### Multi-chain EVM support

Hisho is designed to let the user move across a rich network set without manually rewriting RPC settings for every chain. The active chain is selected through the settings wizard and the app automatically keeps the chain metadata and RPC endpoints in sync with the selected network.

### Secure local wallet management

The wallet system is designed around:

- generated BIP-39 mnemonic support
- system PIN enforcement for sensitive wallet actions
- secure storage in the native OS keyring
- encrypted vault logic with Argon2id and AES-GCM-style protection patterns
- runtime zeroization strategies to reduce risk of wallet material lingering in memory

### AI-driven command interpretation

The project integrates with Google Gemini to convert natural-language requests into tool calls and structured operations. The assistant is meant to handle chat-style prompts such as:

- "Check my ETH balance on Ethereum"
- "Switch to Base Sepolia"
- "Show details for this token contract"
- "Fetch the current token balance for this wallet"

### Terminal-first UX

The user experience is centered around a Rust TUI/CLI flow using `inquire`, `colored`, and `crossterm`. The app presents a guided configuration flow, multi-step prompts, and a clean terminal UI instead of a browser-based wallet experience.

---

## Installation

### Prerequisites

Make sure Rust and Cargo are installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Linux, the OS keyring integration may require the `libsecret` development package:

```bash
# Debian / Ubuntu
sudo apt install libsecret-1-dev pkg-config

# Fedora
sudo dnf install libsecret-devel pkg-config

# Arch
sudo pacman -S libsecret
```

### Install from the local repository

```bash
git clone https://github.com/ukangaekom/hisho_2.0.git
cd hisho_2.0
cargo install --path .
```

### Or build locally

```bash
cargo build --release
```

Then run the binary directly:

```bash
./target/release/hisho --help
```

---

## Getting started

### 1. Launch the setup wizard

```bash
hisho settings
```

This will walk the user through:

- choosing a default blockchain
- setting a system PIN
- creating or loading the secure wallet
- configuring the Gemini API key

### 2. Start the agent

```bash
hisho start
```

If configuration is incomplete, the app triggers the setup flow automatically.

### 3. Use the interactive dashboard

The project also exposes the settings menu directly from the CLI, where the user can configure or review:

- active network
- Gemini API key
- wallet visibility and public address
- runtime configuration state

---

## Example workflows

These are representative of how the current toolchain is intended to be used:

- Query the native token balance on the selected network
- Switch between mainnet and testnet for a supported chain
- Inspect ERC-20 metadata and supply information
- Check NFT collection data and NFT balances
- Ask the AI layer for a natural-language Web3 action and have it map to the configured tools

Example prompts:

```text
Check my wallet balance on Base.
Switch the active chain to Polygon.
Show token details for the contract 0x...
Get my NFT balance on Arbitrum.
What is the current native balance for this address on Mantle?
```

---

## Architecture

The current project structure reflects a clear separation of responsibilities:

```text
src/
├── main.rs               # CLI entrypoint and command parsing
├── chat/                 # terminal interaction and REPL behavior
├── agents/               # agent orchestration and tool routing
├── connection/           # provider and RPC management
├── services/             # getter/setter logic for blockchain queries
├── settings/
│   ├── chain.rs          # live chain matching and registry logic
│   ├── config.rs         # setup wizard and settings flow
│   ├── storage.rs        # secure app settings and vault persistence
│   └── wallet.rs         # wallet creation and EVM wallet derivation
├── database/
│   └── chain_data.rs     # token database ingestion and lookup
├── tools/                # tool map and wrappers for agent execution
├── logs/                 # runtime logging area
└── lib.rs                # library entrypoint
```

The design emphasizes a direct terminal-first workflow, GPT-like reasoning, EVM-specific blockchain operations, and local security boundaries rather than browser-extending wallet UX.

---

## Security model

The current security design is centered on protecting private wallet material while maintaining a seamless user experience:

- system PIN required for sensitive actions
- HD wallet generation and public address derivation
- encrypted secret storage via native OS keyring integration
- runtime memory protections intended to reduce secret exposure
- no browser extension dependency or web wallet surface

This keeps the wallet and private material local to the machine and outside the browser ecosystem.

> Note: The project is currently EVM-focused. Solana-specific wallet derivation is not fully implemented in the active codebase and should not be described as production-ready support.

---

## Limitations and current scope

Hisho is an advanced terminal-based Web3 assistant, but the active implementation still has a few scope boundaries:

- primary focus is EVM-compatible networks and tokens
- wallet operations are designed around the local system keyring and active user configuration
- Gemini integration requires a valid API key to unlock the full AI experience
- the chain registry is actively evolving and is intentionally driven by the source configuration file rather than a fixed static list

---

## License

This project is distributed under a private licensing model. See the `LICENSE` file for the current terms.

---

## Project note

Hisho 2.0 is built as a practical, security-conscious terminal assistant for modern on-chain work: network switching, wallet protection, chain-aware queries, and AI-assisted decision support in a single command-line workflow.
