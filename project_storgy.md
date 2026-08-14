# 🐜 Hisho 2.0: Project Story & Business Vision

## Team Narrative: How We Build with AI

At Hisho, AI is not a peripheral feature—it is our core pair programmer, architecture reviewer, and reasoning engine. In our daily operations, the human team focuses on **zero-trust cryptographic security, systems-level Rust performance, proprietary smart contract auditing, and low-level OS Keyring integrations**. Meanwhile, **Google Gemini AI acts as our high-speed reasoning orchestrator**, handling complex natural language intent parsing, multi-chain schema translation, and semantic query breakdown.

By delegating intent resolution to Gemini 2.5 Flash while keeping all key derivation, memory zeroization, and on-chain signing strictly within compiled Rust, we achieved an unshakeable division of labor: **Humans enforce absolute cryptographic safety; AI handles natural human interaction.**

Beyond our founding team, Hisho opens unprecedented economic opportunities for independent developers, DeFi quantitative analysts, and financial auditors. By providing a zero-friction, terminal-native AI agent, freelancers and asset managers can build automated trading workflows, security audit pipelines, and cross-chain execution desks without paying thousands of dollars for proprietary API gateways or bloated enterprise SaaS stacks.

---

## Inspiration

Decentralized Finance (DeFi) offers unmatched financial freedom, yet user interaction remains notoriously fragmented, risky, and friction-heavy. Everyday Web3 users face:
1. **Catastrophic Phishing & Blind-Signing**: Over $2 Billion was lost in recent years to browser extension exploits, malicious DOM injections, phishing link redirects, and unreadable bytecode approval prompts.
2. **Chain Fragmentation & UX Friction**: Switching between 25+ EVM blockchains requires manual RPC configuration, gas estimation headaches, and constant network toggling across web wallets.
3. **Accessibility Barriers**: Non-technical users struggle to query smart contract state, calculate real-time token valuations, or verify NFT token standards without writing custom code or navigating complex block explorers.

We built **Hisho 2.0** to eliminate browser-based attack surfaces entirely and deliver an **autonomous, lightning-fast, terminal-native AI assistant** that turns natural language into verified, on-chain financial execution.

---

## What it does

Hisho 2.0 is an autonomous, terminal-native AI agent and Web3 financial executive assistant written in Rust. It empowers users to query market data, inspect token/NFT state, and execute secure multi-chain transactions using simple English commands directly from their command line.

Key capabilities include:
* **⚡ Multi-Chain Auto-Provisioning**: Instant switching across 25+ EVM chains (Mantle, Ethereum, Base, Polygon, Arbitrum, Optimism, Sonic, etc.), automatically loading both Mainnet and Testnet RPC endpoints.
* **🔒 Zero-Trust Vault Security**: Cryptographically zeroized 24-word BIP-39 mnemonic handling with Argon2id key derivation, AES-256-GCM encryption, and native OS Keyring storage (GNOME Keyring, macOS Keychain, Windows Credential Manager).
* **🤖 Natural Language DeFi Execution**: Interacts with ERC-20 tokens, ERC-721/1155 NFTs, and major protocols like Aave V3 using natural language prompts parsed by Google Gemini.
* **🔐 System PIN Guard**: Demands mandatory System PIN authentication before performing signed transactions or deriving private wallet credentials.

---

## How we built it

Hisho 2.0 was engineered ground-up in Rust for near-zero latency, link-time optimization (LTO), and absolute memory safety.

```text
┌──────────────────────────────────────────────────────────────────┐
│                   Terminal REPL (inquire / colored)               │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│             Google Gemini 2.5 Flash AI Engine                    │
│      (Parses natural language intent -> JSON tool format)        │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│             Hisho Orchestrator & Rust Services                   │
│   (Getter/Setter modules for Tokens, NFTs, Prices & Protocols)   │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│             Hardware OS Keyring + Alloy Provider                 │
│      (Argon2id + AES-256-GCM + Direct RPC Node Execution)         │
└──────────────────────────────────────────────────────────────────┘
```

1. **AI Intent Parsing**: Integrated `genai` with Google Gemini 2.5 Flash to convert raw prompt text into structured JSON execution vectors (`[tool_name, parameters]`).
2. **Blockchain Execution**: Powered by `alloy-rs`, connecting directly to JSON-RPC nodes for fast, reliable transaction simulation and execution.
3. **Cryptographic Engine**: Used `argon2`, `aes-gcm`, `keyring-rs`, and `zeroize` to prevent secret exposure in memory dumps.
4. **Terminal UI**: Built with `inquire`, `crossterm`, and `colored` to deliver a rich, neon-accented, truecolor dashboard experience.

---

## Security Architecture & Anti-Phishing Defense

Traditional Web3 wallets rely on browser extensions vulnerable to XSS attacks, clipboard hijacking, and malicious web page overrides. Hisho fundamentally redefines Web3 security:
* **Zero Browser Surface**: Running exclusively in the terminal removes browser extensions, HTML/JS injection vectors, and phishing popups.
* **Human-Readable Pre-Execution Proofs**: Every signed transaction displays full proof metrics (Tx Hash, Block Number, Gas Used, Recipient Address) before and after broadcast.
* **Zeroized RAM Buffers**: Secrets are wrapped in `zeroize::Zeroizing<String>`, ensuring key material is scrubbed from system memory immediately after use.

---

## Commercial Strategy: Proprietary Licensing & Version Upgrades

Hisho is built as a **closed-source, proprietary financial software suite**, protecting core intellectual property, prompt engineering heuristics, and security algorithms while combining a sustainable **Bring Your Own Key (BYOK)** model with tiered commercial licensing:

* **Free Starter Binary**:
  - Compiled proprietary CLI binary providing essential read-only market queries, token metadata, native balance checks, and manual single-wallet CLI execution using personal Gemini API keys.
  - Zero host infrastructure cost while driving user acquisition and product familiarity without exposing underlying source code.

* **Purchasable Pro License & Major Version Upgrades**:
  - **Unattended Daemon Execution**: Headless background execution for automated 24/7 trading and yield rebalancing.
  - **Hardware Wallet Integration**: Native Ledger, Trezor, and YubiKey signing support.
  - **Multi-Wallet Orchestration**: Manage and execute across multiple institutional wallets simultaneously.
  - **Perpetual Commercial Licenses & Version Passes**: Customers purchase a lifetime license key for major releases (e.g., Hisho Pro 2.0) with discounted upgrade passes for future major versions (e.g., Hisho Pro 3.0).

---

## Challenges we ran into

1. **Deterministic AI Tool Calling**: LLMs can produce varied text outputs. We engineered strict Chain-of-Thought (CoT) system prompts and custom regex extraction rules (`destructor_task`) to guarantee 100% deterministic JSON tuple output.
2. **Multi-Chain Provider Management**: Handling dynamic chain switches across 25+ EVM chains while preserving state required building a thread-safe, lazy-initialized connection registry (`Lazy<HashMap>`).
3. **Cross-Platform OS Keyring Support**: Ensuring seamless secret storage across Linux (libsecret/GNOME Keyring), macOS (Keychain), and Windows required extensive fallback logic and custom Argon2id key derivation parameters.

---

## Accomplishments that we're proud of

* **Uncompromised Security**: Zero plain-text key exposure in RAM or configuration files.
* **Blazing Fast Startup**: Instant binary launch under 5ms with near-zero latency execution.
* **Full EVM & Protocol Coverage**: 30+ service tools spanning market pricing, ERC-20, NFTs, Aave V3 lending, and multi-chain switching.
* **Developer-Grade UX**: Interactive CLI wizard that auto-configures RPCs, credentials, and settings on first launch.

---

## What we learned

* Combining Rust's strict type safety with Gemini's flexible reasoning creates an unmatched paradigm for financial AI agents.
* Eliminating browser dependencies is single-handedly the most effective anti-phishing defense in Web3 today.
* A proprietary BYOK model paired with purchasable Pro binary licenses protects core IP while securing high-margin commercial revenue.

---

## What's next for Hisho

* **Circle & Arc Blockchain Settlement Rails**: Deep integration with Circle USDC cross-chain transfer protocol (CCTP) and Arc blockchain native token standards for sub-second global settlement.
* **Automated Yield Strategy Agents**: Autonomous background agents that rebalance yield positions on Aave and DEX liquidity pools based on user-defined risk parameters.
* **Voice-Activated Terminal REPL**: Hands-free voice command integration for seamless desktop financial management.
* **Enterprise Trading Desk Plugins**: Multi-sig wallet integration and team permissioning controls for institutional treasuries.
