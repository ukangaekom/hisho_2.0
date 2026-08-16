# Hisho 2.0 Orchestrator & Services Tool Registry

## Who am I

**Hisho (秘書)** is an autonomous, high-performance **Agentic Web3 Copilot & Intelligent Financial Assistant** built in Rust, engineered to simplify, orchestrate, and secure multi-chain EVM operations through natural language conversation.

### 🌟 Vision & Core Pillars

* **🤖 Tactical Multi-Agent Orchestration**: Hisho acts as your dedicated digital secretary, translating natural language intent into precise, execution-ready blockchain operations, smart contract interactions, and real-time financial telemetry.
* **🛡️ Zeroized Financial Vault Security**: Built with security-first architecture utilizing Argon2id key derivation, AES-256-GCM encryption, and `ZeroizeOnDrop` heap protection—ensuring private key material remains strictly isolated and zeroized in memory.
* **⚡ Multi-Chain Fluidity**: Seamlessly transitions across diverse EVM networks (Ethereum, Avalanche, Mantle, Arbitrum, Base, Sepolia, Arc) with dynamic RPC endpoint calibration.
* **📊 Comprehensive DeFi & Asset Operations**: Native support for token pricing, ERC-20 transfers & approvals, NFT collection management, and direct integration with leading DeFi protocols like Aave.

---

This document serves as the official specification and registry for all **services functions** and **agent tools** available in the Hisho 2.0 ecosystem. 

---

## Tools Section

```json
[
  {
    "name": "get_price",
    "description": "Fetches the real-time USD price of a specified cryptocurrency token or multiple tokens using CryptoCompare API.",
    "parameters": [
      {
        "name": "_price",
        "type": "string",
        "description": "Token symbol or ticker abbreviation (e.g., 'AVAX', 'LINK', 'BTC', 'ETH')"
      }
    ],
    "example_output": "[get_price, AVAX]"
  },
  {
    "name": "get_marketcap",
    "description": "Fetches the total market capitalization / valuation in USD for a specified cryptocurrency token.",
    "parameters": [
      {
        "name": "coin",
        "type": "string",
        "description": "Token symbol or ticker abbreviation (e.g., 'BTC', 'LINK', 'SOL')"
      }
    ],
    "example_output": "[get_marketcap, BTC]"
  },
  {
    "name": "get_native_balance",
    "description": "Queries the on-chain native token balance (e.g., AVAX, ETH, MNT) for a given EVM wallet address on the active network.",
    "parameters": [
      {
        "name": "wallet",
        "type": "string",
        "description": "Valid EVM wallet address starting with 0x"
      }
    ],
    "example_output": "[get_native_balance, 0x8a50dC735814248d84d5dA162A0d4D692d7B7E6f]"
  },
  {
    "name": "get_system_wallet",
    "description": "Retrieves the configured system public EVM wallet address from vault storage without requiring System PIN authentication.",
    "parameters": [],
    "example_output": "[get_system_wallet]"
  },
  {
    "name": "get_system_wallet_with_pin",
    "description": "Authenticates with System PIN to decrypt secure vault storage and derive the system EVM wallet address.",
    "parameters": [
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN code"
      }
    ],
    "example_output": "[get_system_wallet_with_pin, 1234]"
  },
  {
    "name": "get_token_details_mainnet",
    "description": "Fetches smart contract details (name, symbol, decimals, total supply) for fungible ERC-20 token(s) on Mainnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token smart contract address"
      }
    ],
    "example_output": "[get_token_details_mainnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3]"
  },
  {
    "name": "get_token_details_testnet",
    "description": "Fetches smart contract details (name, symbol, decimals, total supply) for fungible ERC-20 token(s) on Testnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token smart contract address on Testnet"
      }
    ],
    "example_output": "[get_token_details_testnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3]"
  },
  {
    "name": "get_token_balance_mainnet",
    "description": "Fetches the ERC-20 token balance for specified wallet address(es) on Mainnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address"
      },
      {
        "name": "wallet_address",
        "type": "string",
        "description": "Target wallet address to query balance for"
      }
    ],
    "example_output": "[get_token_balance_mainnet, 0x06eFdBFf2a14a7c8E15944D1F4A48F9F95F663A4, 0xDb7608614dfdD9feBFC1b82A7609420fa7B3Bc34]"
  },
  {
    "name": "get_token_balance_testnet",
    "description": "Fetches the ERC-20 token balance for specified wallet address(es) on Testnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address on Testnet"
      },
      {
        "name": "wallet_address",
        "type": "string",
        "description": "Target wallet address to query balance for"
      }
    ],
    "example_output": "[get_token_balance_testnet, 0x06eFdBFf2a14a7c8E15944D1F4A48F9F95F663A4, 0xDb7608614dfdD9feBFC1b82A7609420fa7B3Bc34]"
  },
  {
    "name": "get_token_total_supply_mainnet",
    "description": "Queries the total minted supply of an ERC-20 token contract on Mainnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address"
      }
    ],
    "example_output": "[get_token_total_supply_mainnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3]"
  },
  {
    "name": "get_token_total_supply_testnet",
    "description": "Queries the total minted supply of an ERC-20 token contract on Testnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address on Testnet"
      }
    ],
    "example_output": "[get_token_total_supply_testnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3]"
  },
  {
    "name": "get_nft_details_mainnet",
    "description": "Queries metadata and collection details (name, symbol, total supply) for ERC-721 or ERC-1155 NFT contracts on Mainnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "NFT collection contract address"
      }
    ],
    "example_output": "[get_nft_details_mainnet, 0x815168946948065538eD1D70197b9A4B9F13125f]"
  },
  {
    "name": "get_nft_details_testnet",
    "description": "Queries metadata and collection details (name, symbol, total supply) for ERC-721 or ERC-1155 NFT contracts on Testnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "NFT collection contract address on Testnet"
      }
    ],
    "example_output": "[get_nft_details_testnet, 0x815168946948065538eD1D70197b9A4B9F13125f]"
  },
  {
    "name": "get_nft_balance_mainnet",
    "description": "Queries the quantity of NFTs owned by target wallet address(es) within a specified ERC-721/ERC-1155 collection on Mainnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "NFT collection contract address"
      },
      {
        "name": "wallet_address",
        "type": "string",
        "description": "Target wallet address"
      }
    ],
    "example_output": "[get_nft_balance_mainnet, 0x26d4461aFA7334E945071727751e156ff5a55D8f, 0x4DfB54cA7053c40fBa2C4AF713d8c583fE735686]"
  },
  {
    "name": "get_nft_balance_testnet",
    "description": "Queries the quantity of NFTs owned by target wallet address(es) within a specified ERC-721/ERC-1155 collection on Testnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "NFT collection contract address on Testnet"
      },
      {
        "name": "wallet_address",
        "type": "string",
        "description": "Target wallet address"
      }
    ],
    "example_output": "[get_nft_balance_testnet, 0x26d4461aFA7334E945071727751e156ff5a55D8f, 0x4DfB54cA7053c40fBa2C4AF713d8c583fE735686]"
  },
  {
    "name": "get_nft_total_supply_mainnet",
    "description": "Fetches total minted token count for an ERC-721 NFT collection on Mainnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "NFT collection contract address"
      }
    ],
    "example_output": "[get_nft_total_supply_mainnet, 0x815168946948065538eD1D70197b9A4B9F13125f]"
  },
  {
    "name": "get_nft_total_supply_testnet",
    "description": "Fetches total minted token count for an ERC-721 NFT collection on Testnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "NFT collection contract address on Testnet"
      }
    ],
    "example_output": "[get_nft_total_supply_testnet, 0x815168946948065538eD1D70197b9A4B9F13125f]"
  },
  {
    "name": "switch_chain",
    "description": "Dynamically switches active blockchain network and RPC provider endpoints (e.g., Avalanche, Ethereum, Mantle, Arbitrum, Base).",
    "parameters": [
      {
        "name": "query",
        "type": "string",
        "description": "Target network name or fuzzy chain match string"
      }
    ],
    "example_output": "[switch_chain, Avalanche]"
  },
  {
    "name": "transfer_token_mainnet",
    "description": "Constructs, signs, and broadcasts an ERC-20 token transfer transaction on Mainnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address"
      },
      {
        "name": "to_address",
        "type": "string",
        "description": "Recipient wallet address"
      },
      {
        "name": "amount",
        "type": "string",
        "description": "Token transfer amount"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[transfer_token_mainnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, 10, 1234]"
  },
  {
    "name": "transfer_token_testnet",
    "description": "Constructs, signs, and broadcasts an ERC-20 token transfer transaction on Testnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address on Testnet"
      },
      {
        "name": "to_address",
        "type": "string",
        "description": "Recipient wallet address"
      },
      {
        "name": "amount",
        "type": "string",
        "description": "Token transfer amount"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[transfer_token_testnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, 10, 1234]"
  },
  {
    "name": "approve_token_mainnet",
    "description": "Approves an ERC-20 token allowance for a spender contract or address on Mainnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address"
      },
      {
        "name": "spender_address",
        "type": "string",
        "description": "Approved spender contract/wallet address"
      },
      {
        "name": "amount",
        "type": "string",
        "description": "Allowance amount to approve"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[approve_token_mainnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3, 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE, 50, 1234]"
  },
  {
    "name": "approve_token_testnet",
    "description": "Approves an ERC-20 token allowance for a spender contract or address on Testnet.",
    "parameters": [
      {
        "name": "token_address",
        "type": "string",
        "description": "ERC-20 token contract address on Testnet"
      },
      {
        "name": "spender_address",
        "type": "string",
        "description": "Approved spender contract/wallet address"
      },
      {
        "name": "amount",
        "type": "string",
        "description": "Allowance amount to approve"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[approve_token_testnet, 0x79379C0E09a41d7978f883a56246290eE9a8c4d3, 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE, 50, 1234]"
  },
  {
    "name": "transfer_nft_mainnet",
    "description": "Executes safeTransferFrom for an ERC-721 NFT from owner to recipient on Mainnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "ERC-721 NFT contract address"
      },
      {
        "name": "from_address",
        "type": "string",
        "description": "Current owner wallet address"
      },
      {
        "name": "to_address",
        "type": "string",
        "description": "Recipient wallet address"
      },
      {
        "name": "token_id",
        "type": "string",
        "description": "NFT token identifier"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[transfer_nft_mainnet, 0x815168946948065538eD1D70197b9A4B9F13125f, 0x4DfB54cA7053c40fBa2C4AF713d8c583fE735686, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, 1, 1234]"
  },
  {
    "name": "transfer_nft_testnet",
    "description": "Executes safeTransferFrom for an ERC-721 NFT from owner to recipient on Testnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "ERC-721 NFT contract address on Testnet"
      },
      {
        "name": "from_address",
        "type": "string",
        "description": "Current owner wallet address"
      },
      {
        "name": "to_address",
        "type": "string",
        "description": "Recipient wallet address"
      },
      {
        "name": "token_id",
        "type": "string",
        "description": "NFT token identifier"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[transfer_nft_testnet, 0x815168946948065538eD1D70197b9A4B9F13125f, 0x4DfB54cA7053c40fBa2C4AF713d8c583fE735686, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, 1, 1234]"
  },
  {
    "name": "approve_nft_mainnet",
    "description": "Approves an operator to manage a specific ERC-721 token ID on Mainnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "ERC-721 NFT contract address"
      },
      {
        "name": "to_address",
        "type": "string",
        "description": "Approved operator address"
      },
      {
        "name": "token_id",
        "type": "string",
        "description": "NFT token identifier"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[approve_nft_mainnet, 0x815168946948065538eD1D70197b9A4B9F13125f, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, 1, 1234]"
  },
  {
    "name": "approve_nft_testnet",
    "description": "Approves an operator to manage a specific ERC-721 token ID on Testnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "ERC-721 NFT contract address on Testnet"
      },
      {
        "name": "to_address",
        "type": "string",
        "description": "Approved operator address"
      },
      {
        "name": "token_id",
        "type": "string",
        "description": "NFT token identifier"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[approve_nft_testnet, 0x815168946948065538eD1D70197b9A4B9F13125f, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, 1, 1234]"
  },
  {
    "name": "set_approval_for_all_nft_mainnet",
    "description": "Grants or revokes full operator approval for all NFTs in an ERC-721 collection on Mainnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "ERC-721 NFT contract address"
      },
      {
        "name": "operator_address",
        "type": "string",
        "description": "Target operator address"
      },
      {
        "name": "approved",
        "type": "boolean",
        "description": "True to grant approval, false to revoke approval"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[set_approval_for_all_nft_mainnet, 0x815168946948065538eD1D70197b9A4B9F13125f, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, true, 1234]"
  },
  {
    "name": "set_approval_for_all_nft_testnet",
    "description": "Grants or revokes full operator approval for all NFTs in an ERC-721 collection on Testnet.",
    "parameters": [
      {
        "name": "nft_address",
        "type": "string",
        "description": "ERC-721 NFT contract address on Testnet"
      },
      {
        "name": "operator_address",
        "type": "string",
        "description": "Target operator address"
      },
      {
        "name": "approved",
        "type": "boolean",
        "description": "True to grant approval, false to revoke approval"
      },
      {
        "name": "pin",
        "type": "string",
        "description": "System authentication PIN"
      }
    ],
    "example_output": "[set_approval_for_all_nft_testnet, 0x815168946948065538eD1D70197b9A4B9F13125f, 0x42b017dA370fDE0EeD32e24F99850C0E6fb7a8a6, true, 1234]"
  },
  {
    "name": "aave_wrapped_token_get_wrappedethaddress_token",
    "description": "Queries the Wrapped ETH token address from the Aave Wrapped Token Gateway contract.",
    "parameters": [
      {
        "name": "gateway_address",
        "type": "string",
        "description": "Aave Wrapped Token Gateway contract address"
      }
    ],
    "example_output": "[aave_wrapped_token_get_wrappedethaddress_token, 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE]"
  },
  {
    "name": "aave_wrapped_token_get_weth_address",
    "description": "Queries the underlying WETH token address from the Aave Wrapped Token Gateway contract.",
    "parameters": [
      {
        "name": "gateway_address",
        "type": "string",
        "description": "Aave Wrapped Token Gateway contract address"
      }
    ],
    "example_output": "[aave_wrapped_token_get_weth_address, 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE]"
  },
  {
    "name": "aave_wrapped_token_get_pool_address",
    "description": "Queries the connected Aave Pool address from the Aave Wrapped Token Gateway contract.",
    "parameters": [
      {
        "name": "gateway_address",
        "type": "string",
        "description": "Aave Wrapped Token Gateway contract address"
      }
    ],
    "example_output": "[aave_wrapped_token_get_pool_address, 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE]"
  },
  {
    "name": "aave_wrapped_token_get_owner",
    "description": "Queries the administrative owner address of the Aave Wrapped Token Gateway contract.",
    "parameters": [
      {
        "name": "gateway_address",
        "type": "string",
        "description": "Aave Wrapped Token Gateway contract address"
      }
    ],
    "example_output": "[aave_wrapped_token_get_owner, 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE]"
  }
]
```

---

## Detailed Service Functions Reference

Below is a breakdown of all Rust underlying service functions mapped into the tools registry:

| Tool Name | Rust Function Signature | File Location | Category |
| :--- | :--- | :--- | :--- |
| `get_price` | `pub async fn get_price(_price: &str) -> String` | `src/services/getter/market.rs` | Market & Price |
| `get_marketcap` | `pub async fn get_marketcap(coin: &str) -> String` | `src/services/getter/market.rs` | Market & Price |
| `get_native_balance` | `pub async fn get_native_balance(wallet: &str) -> String` | `src/services/getter/wallet.rs` | Wallet & Balance |
| `get_system_wallet` | `pub async fn get_system_wallet() -> String` | `src/services/getter/wallet.rs` | System & Config |
| `get_system_wallet_with_pin` | `pub fn get_system_wallet_with_pin(pin: &str) -> String` | `src/services/getter/wallet.rs` | System & Config |
| `get_token_details_mainnet` | `pub async fn get_token_details_mainnet(token_address: &str) -> String` | `src/services/getter/token.rs` | Token Getter |
| `get_token_details_testnet` | `pub async fn get_token_details_testnet(token_address: &str) -> String` | `src/services/getter/token.rs` | Token Getter |
| `get_token_balance_mainnet` | `pub async fn get_token_balance_mainnet(token_address: &str, wallet_address: &str) -> String` | `src/services/getter/token.rs` | Token Getter |
| `get_token_balance_testnet` | `pub async fn get_token_balance_testnet(token_address: &str, wallet_address: &str) -> String` | `src/services/getter/token.rs` | Token Getter |
| `get_token_total_supply_mainnet` | `pub async fn get_token_total_supply_mainnet(token_address: &str) -> String` | `src/services/getter/token.rs` | Token Getter |
| `get_token_total_supply_testnet` | `pub async fn get_token_total_supply_testnet(token_address: &str) -> String` | `src/services/getter/token.rs` | Token Getter |
| `get_nft_details_mainnet` | `pub async fn get_nft_details_mainnet(nft_address: &str) -> String` | `src/services/getter/nft.rs` | NFT Getter |
| `get_nft_details_testnet` | `pub async fn get_nft_details_testnet(nft_address: &str) -> String` | `src/services/getter/nft.rs` | NFT Getter |
| `get_nft_balance_mainnet` | `pub async fn get_nft_balance_mainnet(nft_address: &str, wallet_address: &str) -> String` | `src/services/getter/nft.rs` | NFT Getter |
| `get_nft_balance_testnet` | `pub async fn get_nft_balance_testnet(nft_address: &str, wallet_address: &str) -> String` | `src/services/getter/nft.rs` | NFT Getter |
| `get_nft_total_supply_mainnet` | `pub async fn get_nft_total_supply_mainnet(nft_address: &str) -> String` | `src/services/getter/nft.rs` | NFT Getter |
| `get_nft_total_supply_testnet` | `pub async fn get_nft_total_supply_testnet(nft_address: &str) -> String` | `src/services/getter/nft.rs` | NFT Getter |
| `switch_chain` | `pub fn switch_chain(query: &str) -> String` | `src/settings/chain.rs` | Chain Management |
| `transfer_token_mainnet` | `pub async fn transfer_token_mainnet(token_address: &str, to_address: &str, amount: &str, pin: &str) -> String` | `src/services/setter/token.rs` | Token Setter |
| `transfer_token_testnet` | `pub async fn transfer_token_testnet(token_address: &str, to_address: &str, amount: &str, pin: &str) -> String` | `src/services/setter/token.rs` | Token Setter |
| `approve_token_mainnet` | `pub async fn approve_token_mainnet(token_address: &str, spender_address: &str, amount: &str, pin: &str) -> String` | `src/services/setter/token.rs` | Token Setter |
| `approve_token_testnet` | `pub async fn approve_token_testnet(token_address: &str, spender_address: &str, amount: &str, pin: &str) -> String` | `src/services/setter/token.rs` | Token Setter |
| `transfer_nft_mainnet` | `pub async fn transfer_nft_mainnet(nft_address: &str, from_address: &str, to_address: &str, token_id: &str, pin: &str) -> String` | `src/services/setter/nft.rs` | NFT Setter |
| `transfer_nft_testnet` | `pub async fn transfer_nft_testnet(nft_address: &str, from_address: &str, to_address: &str, token_id: &str, pin: &str) -> String` | `src/services/setter/nft.rs` | NFT Setter |
| `approve_nft_mainnet` | `pub async fn approve_nft_mainnet(nft_address: &str, to_address: &str, token_id: &str, pin: &str) -> String` | `src/services/setter/nft.rs` | NFT Setter |
| `approve_nft_testnet` | `pub async fn approve_nft_testnet(nft_address: &str, to_address: &str, token_id: &str, pin: &str) -> String` | `src/services/setter/nft.rs` | NFT Setter |
| `set_approval_for_all_nft_mainnet` | `pub async fn set_approval_for_all_nft_mainnet(nft_address: &str, operator_address: &str, approved: bool, pin: &str) -> String` | `src/services/setter/nft.rs` | NFT Setter |
| `set_approval_for_all_nft_testnet` | `pub async fn set_approval_for_all_nft_testnet(nft_address: &str, operator_address: &str, approved: bool, pin: &str) -> String` | `src/services/setter/nft.rs` | NFT Setter |
| `aave_wrapped_token_get_wrappedethaddress_token` | `pub async fn aave_wrapped_token_get_wrappedethaddress_token(gateway_address: &str) -> String` | `src/services/getter/protocols/lending/aave.rs` | Protocol (Aave) |
| `aave_wrapped_token_get_weth_address` | `pub async fn aave_wrapped_token_get_weth_address(gateway_address: &str) -> String` | `src/services/getter/protocols/lending/aave.rs` | Protocol (Aave) |
| `aave_wrapped_token_get_pool_address` | `pub async fn aave_wrapped_token_get_pool_address(gateway_address: &str) -> String` | `src/services/getter/protocols/lending/aave.rs` | Protocol (Aave) |
| `aave_wrapped_token_get_owner` | `pub async fn aave_wrapped_token_get_owner(gateway_address: &str) -> String` | `src/services/getter/protocols/lending/aave.rs` | Protocol (Aave) |
