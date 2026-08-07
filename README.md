<div align="center">
  <h1>🐜 Hisho 2.0</h1>
  <p><strong>A lightning-fast, terminal-native AI agent for multi-chain EVM networks.</strong></p>

  [![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue.svg)](LICENSE)
  [![Rust Version](https://img.shields.io/badge/rustc-1.75%2B-brightgreen.svg)](https://www.rust-lang.org/)
  [![Build Status](https://img.shields.io/badge/build-passing-success.svg)](#installation)
</div>

---

## 🌟 Overview

**Hisho 2.0** is an autonomous, terminal-native AI agent and on-chain finance assistant written in Rust. It enables developers and crypto users to query token prices, check contract state, and execute transactions using natural language—all directly from the terminal.

Hisho comes loaded with multi-chain support for **25+ EVM blockchains** (including **Mantle**, **Ethereum**, **Base**, **Polygon**, **Arbitrum**, **Optimism**, **Sonic**, **Celo**, and more), auto-loading both **Mainnet** and **Testnet** RPC endpoints upon network selection.

---

## ✨ Features

- **⚡ Multi-Chain Mainnet & Testnet**: Instantly switch between 25+ EVM networks. Selecting a chain automatically provisions both its Mainnet and Testnet RPC endpoints.
- **🔒 Zeroized 24-Word Wallet**: Cryptographically secure 24-word BIP-39 mnemonic generation with zeroized memory management (`zeroize`) to prevent secret exposure in RAM dumps.
- **🛡️ Argon2id + AES-256-GCM OS Keyring**: Encrypts your 24-word seed phrase with Argon2id key derivation and AES-256-GCM, storing it securely inside your native OS Keyring (GNOME Keyring, macOS Keychain, KWallet, or Windows Credential Manager).
- **🔐 System PIN Access**: Sensitive operations like displaying your 24-word seed phrase require entering your personal System PIN.
- **🤖 Gemini AI Engine**: Integrated with Google Gemini AI for natural language intent parsing and smart contract interactions.
- **🎨 Premium Terminal UI**: Built with `inquire`, `colored`, and `crossterm` featuring aligned badges, neon truecolor box frames, paginated selection menus, and status dashboard cards.
- **🚀 Near-Zero Latency**: Built completely in Rust with LTO (Link-Time Optimization) release profiles for instantaneous startup.

---

## 🛠️ Installation Guide

### Prerequisites

Ensure you have Rust and `cargo` installed on your machine:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Linux systems, make sure `libsecret` (or `gnome-keyring`) is installed for OS Keyring support:

```bash
# Debian / Ubuntu
sudo apt install libsecret-1-dev pkg-config

# Fedora
sudo dnf install libsecret-devel pkg-config

# Arch Linux
sudo pacman -S libsecret
```

---

### Method 1: Cargo Install System-Wide (Recommended)

To install `hisho` as a global command-line binary on your system (accessible from any terminal window):

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ukangaekom/hisho_2.0.git
   cd hisho_2.0
   ```

2. **Install globally:**
   ```bash
   cargo install --path .
   ```

3. **Verify installation:**
   ```bash
   hisho --help
   ```

---

### Method 2: Manual Release Build

To manually compile an optimized production binary:

1. **Build in release mode:**
   ```bash
   cargo build --release
   ```

2. **Move binary to your PATH (Linux / macOS):**
   ```bash
   sudo cp target/release/hisho /usr/local/bin/
   ```

---

## 🚀 Getting Started

### 1. Interactive Setup Wizard

Run the interactive setup wizard to configure your network, seed phrase, and AI key:

```bash
hisho settings
```

Or simply run `hisho start`—if Hisho is not yet configured, the wizard launches automatically!

```text
╭──────────────────────────────────────────────────────────────╮
│ ⚡ HISHO AGENT SYSTEM SETUP WIZARD                           │
│ Configure your network, zeroized wallet, and AI engine       │
╰──────────────────────────────────────────────────────────────╯

── STEP 1: BLOCKCHAIN SELECTION ────────────────────────────────
? Choose default blockchain (Mainnet & Testnet auto-loaded):
  ⚡ Mantle                 │ Mainnet ID: 5000   │ Testnet: Mantle Sepolia (5003)
> ⚡ Ethereum               │ Mainnet ID: 1      │ Testnet: Sepolia (11155111)
  ⚡ Base                   │ Mainnet ID: 8453   │ Testnet: Base Sepolia (84532)
```

During setup:
1. **Select Blockchain**: Choose from 25+ EVM chains (Mainnet & Testnet loaded automatically).
2. **System PIN & Seed Phrase**: Enter a System PIN (minimum 4 characters) to lock your quietly generated 24-word seed phrase directly into your OS Keyring.
3. **Gemini API Key**: Input your Gemini API key for AI engine capabilities.

---

### 2. Launch the AI Agent REPL

Once setup is complete, launch Hisho:

```bash
hisho start
```

**Example Natural Language Queries:**
- *"Check the native MNT balance of my wallet."*
- *"Switch active RPC to testnet."*
- *"Fetch the ERC-20 token info for contract address 0x..."*
- *"Query current gas prices on Mantle Mainnet."*

---

### 3. Settings & Wallet Dashboard

Access the interactive dashboard anytime:

```bash
hisho settings
```

Menu options include:
- `🌐 1. Switch Active Network (Mainnet & Testnet)`
- `🤖 2. Configure Gemini API Key`
- `🔒 3. View Wallet Seed Phrase` *(Requires System PIN)*
- `📊 4. Display System Status & Configuration`
- `🚪 5. Exit Settings Menu`

---

## 🔒 Security Architecture

```text
┌───────────────────────────────────────────────────────────────┐
│                      User System PIN                          │
└──────────────────────────────┬────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────┐
│                     Argon2id Key Derivation                   │
└──────────────────────────────┬────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────┐
│              AES-256-GCM Encrypted Mnemonic                   │
└──────────────────────────────┬────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────┐
│                Native OS Keyring Storage                      │
│      (GNOME Keyring / macOS Keychain / Credential Manager)    │
└──────────────────────────────┬────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────┐
│                 Zeroized In-Memory Runtime                    │
└───────────────────────────────────────────────────────────────┘
```

- **Seed Phrase Immutability**: Once generated, the 24-word seed phrase is locked inside the system and cannot be changed or overwritten.
- **Zeroized RAM**: Seed phrase strings use `zeroize::Zeroizing<String>` to wipe secret buffers from RAM as soon as they drop out of scope.
- **Argon2id + AES-256-GCM**: Key derivation prevents brute-force attacks on the System PIN, while AES-256-GCM provides authenticated encryption.

---

## 📁 Repository Structure

```text
src/
├── agents/             # Autonomous agent modules & tool handlers
├── chat/               # Terminal REPL interface
├── connection/         # Provider & RPC connection management
├── services/           # On-chain queries, EVM calls, and token getters
├── settings/
│   ├── chain.rs        # Blockchain & network models (chain.json integration)
│   ├── config.rs       # Setup wizard & interactive settings menu
│   ├── storage.rs      # Argon2 / AES-GCM / OS Keyring storage engine
│   └── wallet.rs       # 24-word BIP-39 seed phrase generation & zeroization
└── main.rs             # CLI entrypoint & command parser
```

---

## 📄 License

Distributed under the **MIT OR Apache-2.0** License. See `LICENSE` for details.

Developed with ❤️ by **Ekomabasi Ukanga** ([@ukangaekom](https://github.com/ukangaekom)).