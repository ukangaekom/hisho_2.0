use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use crate::settings::storage;

/// Table definition in redb for non-native chain tokens.
/// Key: `u64` (`chain_id`)
/// Value: `&str` (JSON string representing `Vec<TokenData>`)
pub const CHAIN_TOKENS_TABLE: TableDefinition<u64, &str> = TableDefinition::new("chain_tokens");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenData {
    #[serde(default)]
    pub chain_id: u64,
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainTokensEntry {
    pub chain_name: String,
    pub network_type: String,
    #[serde(default)]
    pub chain_id: u64,
    pub rpc_url: Option<String>,
    pub native_token: Option<serde_json::Value>,
    pub tokens: Vec<TokenData>,
}

pub fn get_database_path() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", "hisho", "hisho") {
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).map_err(|e| format!("Failed to create database directory: {}", e))?;
        Ok(data_dir.join("hisho_chain.redb"))
    } else {
        Ok(PathBuf::from("hisho_chain.redb"))
    }
}

static DB_INSTANCE: OnceLock<Arc<Database>> = OnceLock::new();

/// Initializes or retrieves the global redb database handle.
pub fn get_or_init_database() -> Result<Arc<Database>, String> {
    if let Some(db) = DB_INSTANCE.get() {
        return Ok(db.clone());
    }

    let db_path = get_database_path()?;
    let db = Database::create(&db_path)
        .or_else(|_| Database::open(&db_path))
        .map_err(|e| format!("Failed to open or create redb database at {:?}: {}", db_path, e))?;

    let arc_db = Arc::new(db);
    let _ = DB_INSTANCE.set(arc_db.clone());

    // Auto-ingest token.json at runtime if present
    let json_path = Path::new("token.json");
    if json_path.exists() {
        let _ = ingest_token_json_into_db(&arc_db, json_path);
    }

    Ok(arc_db)
}

/// Ensures token.json is ingested into redb at runtime if present.
pub fn ensure_tokens_ingested() -> Result<usize, String> {
    let json_path = Path::new("token.json");
    if json_path.exists() {
        ingest_token_json(json_path)
    } else {
        Ok(0)
    }
}

/// Helper function to open or create a database at a specific custom path (useful for testing or custom paths).
#[allow(dead_code)]
pub fn open_database_at_path(path: &Path) -> Result<Database, String> {
    Database::create(path)
        .or_else(|_| Database::open(path))
        .map_err(|e| format!("Failed to open or create redb database at {:?}: {}", path, e))
}

/// Ingests token details from a `token.json` file into a redb database instance.
/// Existing tokens are merged and deduplicated based on normalized (lowercase) contract address per chain_id.
pub fn ingest_token_json_into_db(db: &Database, json_path: &Path) -> Result<usize, String> {
    let content = fs::read_to_string(json_path)
        .map_err(|e| format!("Failed to read token JSON file at {:?}: {}", json_path, e))?;

    let raw_entries: HashMap<String, ChainTokensEntry> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse token JSON structure: {}", e))?;

    let write_txn = db.begin_write().map_err(|e| format!("Failed to begin redb write transaction: {}", e))?;
    let mut total_added = 0;

    {
        let mut table = write_txn.open_table(CHAIN_TOKENS_TABLE)
            .map_err(|e| format!("Failed to open CHAIN_TOKENS_TABLE: {}", e))?;

        for (chain_id_str, entry) in raw_entries {
            let chain_id: u64 = chain_id_str.parse().unwrap_or(entry.chain_id);
            if chain_id == 0 {
                continue;
            }

            let mut existing_tokens: Vec<TokenData> = if let Ok(Some(access)) = table.get(chain_id) {
                serde_json::from_str(access.value()).unwrap_or_default()
            } else {
                Vec::new()
            };

            let mut known_addresses: HashMap<String, usize> = existing_tokens
                .iter()
                .enumerate()
                .map(|(idx, token)| (token.address.to_lowercase(), idx))
                .collect();

            for new_token in entry.tokens {
                let mut token_item = new_token;
                token_item.chain_id = chain_id;
                let norm_addr = token_item.address.to_lowercase();

                if let Some(&idx) = known_addresses.get(&norm_addr) {
                    existing_tokens[idx] = token_item;
                } else {
                    known_addresses.insert(norm_addr, existing_tokens.len());
                    existing_tokens.push(token_item);
                    total_added += 1;
                }
            }

            let json_val = serde_json::to_string(&existing_tokens)
                .map_err(|e| format!("Failed to serialize tokens for chain_id {}: {}", chain_id, e))?;

            table.insert(chain_id, json_val.as_str())
                .map_err(|e| format!("Failed to insert tokens into redb for chain_id {}: {}", chain_id, e))?;
        }
    }

    write_txn.commit().map_err(|e| format!("Failed to commit redb ingestion transaction: {}", e))?;
    Ok(total_added)
}

/// Ingests token details from `token.json` into the primary global database.
pub fn ingest_token_json(json_path: &Path) -> Result<usize, String> {
    let db = get_or_init_database()?;
    ingest_token_json_into_db(&db, json_path)
}

/// Retrieves all non-native tokens for a specific chain_id from a redb database instance.
#[allow(dead_code)]
pub fn get_tokens_by_chain_id_from_db(db: &Database, chain_id: u64) -> Result<Vec<TokenData>, String> {
    let read_txn = db.begin_read().map_err(|e| format!("Failed to begin redb read transaction: {}", e))?;
    let table = read_txn.open_table(CHAIN_TOKENS_TABLE).map_err(|e| format!("Failed to open CHAIN_TOKENS_TABLE: {}", e))?;

    if let Some(access) = table.get(chain_id).map_err(|e| format!("Error querying chain_id {}: {}", chain_id, e))? {
        let json_str = access.value();
        let tokens: Vec<TokenData> = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to deserialize tokens for chain_id {}: {}", chain_id, e))?;
        Ok(tokens)
    } else {
        Ok(Vec::new())
    }
}

/// Retrieves all non-native tokens for a specific chain_id from the global database.
#[allow(dead_code)]
pub fn get_tokens_by_chain_id(chain_id: u64) -> Result<Vec<TokenData>, String> {
    let db = get_or_init_database()?;
    get_tokens_by_chain_id_from_db(&db, chain_id)
}

/// Queries redb database for a token by symbol or name for a specific chain_id (case-insensitive).
#[allow(dead_code)]
pub fn find_token_by_symbol_or_name_from_db(db: &Database, chain_id: u64, query: &str) -> Result<Option<TokenData>, String> {
    let tokens = get_tokens_by_chain_id_from_db(db, chain_id)?;
    let q_lower = query.trim().to_lowercase();

    for token in tokens {
        if token.symbol.to_lowercase() == q_lower || token.name.to_lowercase() == q_lower {
            return Ok(Some(token));
        }
    }
    Ok(None)
}

/// Queries global redb database for a token by symbol or name for a specific chain_id.
#[allow(dead_code)]
pub fn find_token_by_symbol_or_name(chain_id: u64, query: &str) -> Result<Option<TokenData>, String> {
    let db = get_or_init_database()?;
    find_token_by_symbol_or_name_from_db(&db, chain_id, query)
}

/// Dynamically resolves active network setting from app configuration and finds matching token contract details.
#[allow(dead_code)]
pub fn find_token_for_active_chain(query: &str) -> Result<Option<TokenData>, String> {
    let settings = storage::load_app_settings()
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            crate::settings::config::ensure_configured()
                .expect("Failed to load app settings for active chain token resolution")
        });

    let chain_id = settings.default_chain.mainnet.chain_id;
    find_token_by_symbol_or_name(chain_id, query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingest_and_query_deduplication() {
        let temp_dir = std::env::temp_dir();
        let id: u64 = rand::random();
        let db_path = temp_dir.join(format!("test_hisho_chain_{}.redb", id));
        let json_path = temp_dir.join(format!("test_token_{}.json", id));

        let db = open_database_at_path(&db_path).unwrap();

        let json_content = r#"{
            "1": {
                "chain_name": "Ethereum",
                "network_type": "mainnet",
                "chain_id": 1,
                "rpc_url": "https://ethereum-rpc.publicnode.com",
                "tokens": [
                    {
                        "name": "USD Coin",
                        "symbol": "USDC",
                        "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
                        "decimals": 6
                    }
                ]
            }
        }"#;

        fs::write(&json_path, json_content).unwrap();

        // Initial Ingestion
        let added1 = ingest_token_json_into_db(&db, &json_path).unwrap();
        assert_eq!(added1, 1);

        let tokens1 = get_tokens_by_chain_id_from_db(&db, 1).unwrap();
        assert_eq!(tokens1.len(), 1);
        assert_eq!(tokens1[0].symbol, "USDC");

        // Duplicate Ingestion (Re-ingesting same token should NOT duplicate)
        let added2 = ingest_token_json_into_db(&db, &json_path).unwrap();
        assert_eq!(added2, 0);

        let tokens2 = get_tokens_by_chain_id_from_db(&db, 1).unwrap();
        assert_eq!(tokens2.len(), 1);

        // Test symbol & name lookups
        let usdc = find_token_by_symbol_or_name_from_db(&db, 1, "usdc").unwrap();
        assert!(usdc.is_some());
        assert_eq!(usdc.unwrap().address, "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

        let usdc_by_name = find_token_by_symbol_or_name_from_db(&db, 1, "USD Coin").unwrap();
        assert!(usdc_by_name.is_some());

        // Cleanup
        let _ = fs::remove_file(db_path);
        let _ = fs::remove_file(json_path);
    }

    #[test]
    fn test_ingest_full_token_json_file() {
        let root_token_json = Path::new("token.json");
        if root_token_json.exists() {
            let temp_dir = std::env::temp_dir();
            let id: u64 = rand::random();
            let db_path = temp_dir.join(format!("test_full_hisho_chain_{}.redb", id));
            let db = open_database_at_path(&db_path).unwrap();

            let added = ingest_token_json_into_db(&db, root_token_json).unwrap();
            assert!(added > 0);

            // Verify Ethereum mainnet USDC (chain_id 1)
            let eth_usdc = find_token_by_symbol_or_name_from_db(&db, 1, "USDC").unwrap();
            assert!(eth_usdc.is_some());
            assert_eq!(eth_usdc.unwrap().address.to_lowercase(), "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

            // Verify Arbitrum One USDC (chain_id 42161)
            let arb_usdc = find_token_by_symbol_or_name_from_db(&db, 42161, "USDC").unwrap();
            assert!(arb_usdc.is_some());

            // Verify Sepolia testnet USDC (chain_id 11155111)
            let sepolia_usdc = find_token_by_symbol_or_name_from_db(&db, 11155111, "USDC").unwrap();
            assert!(sepolia_usdc.is_some());

            let _ = fs::remove_file(db_path);
        }
    }
}