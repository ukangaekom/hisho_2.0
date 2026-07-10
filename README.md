<div align="center">
  <h1>Hisho 2.0</h1>
  <p><strong>A lightning-fast, terminal-native AI agent for the Mantle Network.</strong></p>
</div>

Welcome to the **Mantle Agent CLI**, your intelligent, on-chain assistant living directly in your terminal. Powered by Gemini AI and the Alloy blockchain toolkit, this CLI allows you to seamlessly generate wallets, query token prices, and interact with the Mantle Network using natural language.

---

## ✨ Features

- **🚀 Near-Zero Latency**: Written entirely in Rust, avoiding web-server overhead for instant terminal interactions.
- **🧠 Natural Language On-Chain**: Ask the agent to check balances, fetch token details, and execute smart contract functions directly.
- **🔐 Native Wallet Generation**: Spin up and secure new EVM wallets strictly within your local machine environment.
- **⚙️ Persistent Secure Storage**: Your API keys and private keys are encrypted and saved locally in your system's `~/.config` via the `directories` crate—configure once, run forever!

---

## 🛠️ Usage

### 1. Initial Configuration

Before chatting with the agent, you need to configure your environment. Run the interactive setup wizard to securely inject your Gemini API key, Mantle RPC endpoint, and EVM Wallet:

```bash
cargo run settings
```
*Tip: If you don't have a wallet, the CLI can securely generate a new one for you!*

### 2. Chat with the Agent

Once your credentials are locked in, start the AI REPL:

```bash
cargo run start
```

**Example Prompts to try:**
- *"What is the balance of my wallet?"*
- *"Get the current price of LINK."*
- *"Fetch the token details for [Contract Address]."*

---

## 📦 How to Compile & Install System-Wide

To use `mantle-agent-cli` from anywhere on your machine (just like `git` or `npm`), you can compile and publish it directly to your system's binary path.

### Method 1: Using Cargo Install (Recommended)

The easiest and safest way to install a Rust binary system-wide is by using `cargo install`. This compiles the binary with maximum optimizations and places it in your `~/.cargo/bin` directory (which should already be in your system `$PATH`).

1. **Navigate to the project root:**
   ```bash
   cd /path/to/mantle_agent_cli
   ```
2. **Install the CLI:**
   ```bash
   cargo install --path .
   ```
3. **Run it anywhere!**
   ```bash
   mantle-agent-cli start
   ```

### Method 2: Manual Compilation & Bin Linking

If you want to manually compile the release build and move it to a global UNIX bin directory (like `/usr/local/bin`):

1. **Build the Release Version:**
   ```bash
   cargo build --release
   ```
   *This compiles an optimized, stripped binary located at `target/release/mantle-agent-cli`.*

2. **Move the Binary to your Global Path:**
   ```bash
   sudo cp target/release/mantle-agent-cli /usr/local/bin/
   ```

3. **Verify the Installation:**
   ```bash
   mantle-agent-cli --help
   ```

Enjoy your lightning-fast Mantle AI Assistant! 🐜