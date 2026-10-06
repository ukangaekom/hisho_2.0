<div align="center">
  <h1>🐜 Hisho 2.0</h1>
  <p><strong>Terminal-native AI agent for multi-chain on-chain finance</strong></p>
</div>

Hisho 2.0 is a Rust-powered blockchain copilot built for the terminal. It combines a native command-line interface, secure local wallet storage, and configurable AI model providers for intent parsing and tool orchestration, helping users interact with Web3 networks using natural language instead of custom scripts or browser wallets.

This project is intentionally EVM-first. The live chain registry is loaded from a centralized configuration file and supports a large set of mainnets and testnets across Ethereum-compatible ecosystems.

<div align="center">
  <p>
    <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
    <img alt="AI Agent" src="https://img.shields.io/badge/AI-Agentic-22c55e?style=for-the-badge" />
    <img alt="EVM" src="https://img.shields.io/badge/EVM-Multi-chain-3b82f6?style=for-the-badge" />
    <img alt="Terminal" src="https://img.shields.io/badge/Terminal-First-0ea5e9?style=for-the-badge" />
  </p>
</div>

---

## Overview

Hisho gives you a direct way to:

- switch between supported blockchain networks
- inspect native balances and token balances
- query token and NFT metadata
- work with the configured wallet and secure vault
- use a conversational AI layer to translate natural-language prompts into structured operations
- talk to blockchain infrastructure like a developer coworker, not a browser extension

The current implementation is built around a terminal REPL, a settings wizard, an OS-keyring-backed wallet vault, and a dynamic chain registry that is refreshed through the project configuration rather than hard-coded runtime filters.

---

## Quick install

### Bash / zsh

```bash
curl -fsSL https://raw.githubusercontent.com/ukangaekom/hisho_2.0/main/install.sh | bash
export PATH="$PATH:$HOME/.hisho/bin"
source ~/.bashrc
# or
source ~/.zshrc
```

### PowerShell

```powershell
irm https://raw.githubusercontent.com/ukangaekom/hisho_2.0/main/install.ps1 | iex
```

Then restart your terminal or add the install folder to your user PATH:

```powershell
$env:Path += ";$env:LOCALAPPDATA\Programs\hisho"
```

### SSH-based source install

```bash
git clone git@github.com:ukangaekom/hisho_2.0.git
cd hisho_2.0
cargo install --path .
```

### Verify installation

```bash
hisho --help
```

---

## Developer-facing product pitch

```text
╔══════════════════════════════════════════════════════════════════════╗
║  Hisho 2.0                                                    ║
║  Agentic multi-chain terminal copilot for EVM workflows         ║
║  Rust-native • Secure vault • AI-native • Terminal-first        ║
╚══════════════════════════════════════════════════════════════════════╝
```

Hisho is built for developers who want a fast, secure, and scriptable way to interact with blockchain state without relying on browser-heavy wallet UX. It gives you the feel of an AI coding assistant, but for on-chain systems: chain switching, wallet inspection, token queries, metadata retrieval, protocol-aware operations, and a natural-language interface for Web3 tasks.

---

## Agent ecosystem stickers

Hisho is designed to fit naturally into the modern AI coding stack. These are the agent and tooling communities that feel at home alongside Hisho:

- 🤖 Codex
- ⚡ OpenCode
- 🧠 Claude Code
- 🧩 Cursor
- 🌊 Windsurf
- 🔧 Cline
- 🦉 Roo Code
- 🧵 Continue
- 🛠️ Aider
- 🐚 OpenHands
- 🖱️ Goose
- 🚀 Warp
- ⚙️ Kiro
- ☕ JetBrains Junie
- ☁️ Amazon Q Developer
- 🧪 Tabnine
- ✨ Augment Code
- ✅ Qodo
- 🧭 Trae
- 🐚 PearAI
- 🕳️ Void
- 💎 Gemini CLI
- 🚀 Antigravity

These are the “stickers” people can use when describing Hisho as a terminal-native, AI-first companion for blockchain and engineering workflows.

---

## Installation guide

### Prerequisites

Before installing Hisho, make sure your machine has the Rust toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Linux, the OS keyring integration usually requires the system keyring development libraries:

```bash
# Debian / Ubuntu
sudo apt install libsecret-1-dev pkg-config

# Fedora
sudo dnf install libsecret-devel pkg-config

# Arch
sudo pacman -S libsecret
```

### Quick install via curl

Use the project installer script directly from the repository:

```bash
curl -fsSL https://raw.githubusercontent.com/ukangaekom/hisho_2.0/main/install.sh | bash
```

This installs the `hisho` binary into `$HOME/.hisho/bin` and prints the PATH export you need to add:

```bash
export PATH="$PATH:$HOME/.hisho/bin"
```

Then reload your shell:

```bash
source ~/.bashrc
# or
source ~/.zshrc
```

### SSH-based source install

If you prefer to clone via SSH instead of HTTPS:

```bash
git clone git@github.com:ukangaekom/hisho_2.0.git
cd hisho_2.0
cargo install --path .
```

### Manual build from source

```bash
cargo build --release
./target/release/hisho --help
```

After installation, verify the CLI is available:

```bash
hisho --help
```

### First run and setup flow

Run the setup wizard to configure the network, wallet, and AI key:

```bash
hisho settings
```

If the app has not yet been configured, a first-time setup flow will also launch when you run:

```bash
hisho start
```

During setup, select a default chain, create or restore a wallet with a secure system PIN, and choose an AI provider and exact model name. Enter that provider's API key when prompted, or use its existing environment variable.

### AI model selection

Use `hisho settings` to configure or change the AI provider, model name, and API key. The provider selector accepts these model-name patterns:

| Provider | Model name pattern | API-key environment variable |
| --- | --- | --- |
| OpenAI | `gpt-*`, `o1-*`, `o3-*`, `o4-*`, `chatgpt-*`, `codex-*` | `OPENAI_API_KEY` |
| OpenAI Responses | `gpt-5-*`, `gpt-6-*`, or `gpt-*` containing `codex` or `pro` | `OPENAI_API_KEY` |
| Anthropic | `claude-*` | `ANTHROPIC_API_KEY` |
| Gemini | `gemini-*` | `GEMINI_API_KEY` |
| xAI | `grok-*` | `XAI_API_KEY` |
| DeepSeek | `deepseek-*` | `DEEPSEEK_API_KEY` |
| Moonshot / Kimi | `moonshot-*` or `kimi*` | `MOONSHOT_API_KEY` |
| Zai | `glm-*` | `ZAI_API_KEY` |
| Cohere | `command-*` or `embed-*` | `COHERE_API_KEY` |
| Mimo | `mimo-*` | `MIMO_API_KEY` |
| OpenCode Go | `opencode_go::model-name` | `OPENCODE_GO_API_KEY` |
| Atlas Cloud | `atlascloud::model-name` | `ATLAS_CLOUD_API_KEY` |
| Qwen Cloud | `qwen_cloud::model-name` | `QWEN_CLOUD_API_KEY` |
| Fireworks | A model name containing `fireworks` | `FIREWORKS_API_KEY` |
| Ollama | Any other model name; routed to local Ollama | No key required |

The model ID is entered as text so you can use model names beyond a fixed in-app catalog. The API-key prompt is masked. Keys entered in Settings are stored in the application settings JSON file; they are separate from the wallet seed phrase, which is stored in the OS keyring. For environments where you do not want a key saved in that file, set the provider's environment variable instead.

Provider routing depends on the installed `genai` adapter. `gpt-6-*` is routed through the OpenAI Responses adapter; `kimi*` is routed through Moonshot; and `qwen_cloud::` names are translated to the Aliyun adapter's `aliyun::` model namespace. Atlas Cloud is listed in the selector, but the installed adapter does not support it yet, so requests using it return an unsupported-provider error. Confirm the endpoint and credentials expected by your Qwen Cloud account before relying on the Aliyun route.

---

## Current implementation status

The project is not a generic multi-chain abstraction layer for every blockchain. It is currently focused on EVM and EVM-compatible chains, with the active network list managed in the chain registry.

### Verified project reality

- The app is a Rust CLI with commands for `start`, `status`, and `settings`.
- Default chain configuration is loaded from `chain.json`.
- Chain matching logic is implemented in `src/settings/chain.rs` for exact matches, normalized names, native token symbols, and fuzzy matching.
- Wallet storage and key protection are handled in `src/settings/storage.rs` and `src/settings/wallet.rs`.
- Token ingestion for the database is handled in `src/database/chain_data.rs`.
- The project uses a configurable AI model for intent parsing and tool orchestration. See [AI model selection](#ai-model-selection) for supported model-name patterns and routing notes.

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

The project integrates with configurable AI providers to convert natural-language requests into tool calls and structured operations. The assistant is meant to handle chat-style prompts such as:

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
- choosing an AI provider and model name
- configuring its API key

### 2. Start the agent

```bash
hisho start
```

If configuration is incomplete, the app triggers the setup flow automatically.

### 3. Use the interactive dashboard

The project also exposes the settings menu directly from the CLI, where the user can configure or review:

- active network
- selected AI model and API-key status
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
- Hosted AI providers require a valid API key to unlock the full AI experience; local Ollama models do not require a cloud key
- Atlas Cloud model names are selectable but are not supported by the installed `genai` adapter yet
- the chain registry is actively evolving and is intentionally driven by the source configuration file rather than a fixed static list

---

## License

This project is distributed under a private licensing model. See the `LICENSE` file for the current terms.

---

## Project note

Hisho 2.0 is built as a practical, security-conscious terminal assistant for modern on-chain work: network switching, wallet protection, chain-aware queries, and AI-assisted decision support in a single command-line workflow.
