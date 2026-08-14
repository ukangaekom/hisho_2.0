use colored::*;
use serde::{Deserialize, Serialize};
use crate::settings::storage;

pub const CHAIN_JSON: &str = include_str!("../../chain.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NativeToken {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Mainnet {
    pub chain_id: u64,
    pub rpc_url: String,
    pub native_token: NativeToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Testnet {
    pub label: Option<String>,
    pub chain_id: u64,
    pub rpc_url: String,
    pub native_token: NativeToken,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blockchain {
    pub name: String,
    pub mainnet: Mainnet,
    pub testnet: Testnet,
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let testnet_name = self
            .testnet
            .label
            .as_deref()
            .unwrap_or("Testnet");

        let icon = "⚡".truecolor(255, 215, 0).bold();
        let name_str = format!("{:<20}", self.name).truecolor(0, 255, 170).bold();
        let pipe = "│".truecolor(0, 225, 255);
        let mainnet_label = "Mainnet ID:".truecolor(180, 180, 180);
        let mainnet_id = format!("{:<6}", self.mainnet.chain_id).truecolor(0, 225, 255).bold();
        let testnet_label = "Testnet:".truecolor(180, 180, 180);
        let testnet_info = format!("{} ({})", testnet_name, self.testnet.chain_id)
            .truecolor(255, 200, 0)
            .bold();

        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            icon, name_str, pipe, mainnet_label, mainnet_id, pipe, testnet_label, testnet_info
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub chains: Vec<Blockchain>,
}

impl AppConfig {
    pub fn load_default() -> Result<Self, String> {
        serde_json::from_str(CHAIN_JSON).map_err(|e| format!("Failed to parse chain.json: {}", e))
    }
}

/// Normalizes a string by stripping non-alphanumeric characters and converting to lowercase.
fn normalize_str(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

/// Computes Levenshtein edit distance between two strings for fuzzy matching.
#[allow(dead_code)]
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();
    let m = v1.len();
    let n = v2.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if v1[i - 1] == v2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }

    dp[m][n]
}

/// Finds the best matching Blockchain directly from loaded chain.json data based on:
/// 1. Exact numeric chain ID (Mainnet or Testnet)
/// 2. Exact case-insensitive name or testnet label
/// 3. Normalized alphanumeric string equality (e.g., "x layer" == "X Layer" == "xlayer")
/// 4. Native token symbol match (e.g. "OKB", "BERA", "ETH", "MNT", "CRO")
/// 5. Substring containment across names or testnet labels
/// 6. Levenshtein edit distance fuzzy matching
#[allow(dead_code)]
pub fn find_best_chain_match<'a>(chains: &'a [Blockchain], query: &str) -> Option<&'a Blockchain> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    let norm_q = normalize_str(&q);

    // 1. Numeric chain ID match
    if let Ok(id) = q.parse::<u64>() {
        if let Some(c) = chains.iter().find(|b| b.mainnet.chain_id == id || b.testnet.chain_id == id) {
            return Some(c);
        }
    }

    // 2. Exact case-insensitive match on name or testnet label
    for c in chains {
        if c.name.eq_ignore_ascii_case(&q) {
            return Some(c);
        }
        if let Some(ref label) = c.testnet.label {
            if label.eq_ignore_ascii_case(&q) {
                return Some(c);
            }
        }
    }

    // 3. Normalized alphanumeric match (ignores spaces, hyphens, case, special characters)
    for c in chains {
        if normalize_str(&c.name) == norm_q {
            return Some(c);
        }
        if let Some(ref label) = c.testnet.label {
            if normalize_str(label) == norm_q {
                return Some(c);
            }
        }
    }

    // 4. Exact match on Native Token Symbol (e.g., "OKB", "BERA", "MNT", "CRO", "CELO", "GLMR")
    for c in chains {
        if c.mainnet.native_token.symbol.eq_ignore_ascii_case(&q)
            || c.testnet.native_token.symbol.eq_ignore_ascii_case(&q)
        {
            return Some(c);
        }
    }

    // 5. Substring containment match on normalized name or testnet label
    for c in chains {
        let norm_name = normalize_str(&c.name);
        let norm_label = c.testnet.label.as_ref().map(|l| normalize_str(l)).unwrap_or_default();

        if norm_name.contains(&norm_q) || norm_q.contains(&norm_name) {
            return Some(c);
        }
        if !norm_label.is_empty() && (norm_label.contains(&norm_q) || norm_q.contains(&norm_label)) {
            return Some(c);
        }
    }

    // 6. Dynamic Levenshtein Edit Distance Fuzzy Match
    let mut best_candidate: Option<(&Blockchain, usize)> = None;

    for c in chains {
        let norm_name = normalize_str(&c.name);
        let dist_name = levenshtein_distance(&norm_q, &norm_name);

        let dist_label = if let Some(ref label) = c.testnet.label {
            levenshtein_distance(&norm_q, &normalize_str(label))
        } else {
            usize::MAX
        };

        let min_dist = dist_name.min(dist_label);

        match best_candidate {
            None => best_candidate = Some((c, min_dist)),
            Some((_, best_dist)) => {
                if min_dist < best_dist {
                    best_candidate = Some((c, min_dist));
                }
            }
        }
    }

    // Accept candidate if edit distance is within acceptable tolerance
    if let Some((candidate, dist)) = best_candidate {
        let max_allowed_dist = (norm_q.len() / 2).max(3);
        if dist <= max_allowed_dist {
            return Some(candidate);
        }
    }

    None
}

/// Dynamic runtime function to switch active network chain.
#[allow(dead_code)]
pub fn switch_chain(query: &str) -> String {
    let config = match AppConfig::load_default() {
        Ok(c) => c,
        Err(e) => return format!("Failed to load chain configuration from chain.json: {}", e),
    };

    let matched_chain = match find_best_chain_match(&config.chains, query) {
        Some(c) => c.clone(),
        None => {
            let available_chains: Vec<String> = config.chains.iter().map(|c| c.name.clone()).collect();
            return format!(
                "Could not find any blockchain matching '{}' in chain.json. Available chains ({}): {}",
                query,
                available_chains.len(),
                available_chains.join(", ")
            );
        }
    };

    let mut settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => crate::settings::config::AppSettings {
            default_chain: matched_chain.clone(),
            custom_rpc: None,
            gemini_api_key: None,
            wallet_address: None,
        },
    };

    settings.default_chain = matched_chain.clone();

    if let Err(e) = storage::save_app_settings(&settings) {
        return format!("Matched chain '{}', but failed to persist setting: {}", matched_chain.name, e);
    }

    let testnet_label = matched_chain.testnet.label.as_deref().unwrap_or("Testnet");
    format!(
        "Successfully switched active network to {} (Mainnet Chain ID: {}, Testnet: {} [{}])",
        matched_chain.name, matched_chain.mainnet.chain_id, testnet_label, matched_chain.testnet.chain_id
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_chain_fuzzy_matching() {
        let config = AppConfig::load_default().unwrap();

        // Exact & normalized chain.json tests
        assert_eq!(find_best_chain_match(&config.chains, "x layer").unwrap().name, "X Layer");
        assert_eq!(find_best_chain_match(&config.chains, "xlayer").unwrap().name, "X Layer");
        assert_eq!(find_best_chain_match(&config.chains, "okb").unwrap().name, "X Layer");
        assert_eq!(find_best_chain_match(&config.chains, "196").unwrap().name, "X Layer");

        assert_eq!(find_best_chain_match(&config.chains, "eth").unwrap().name, "Ethereum");
        assert_eq!(find_best_chain_match(&config.chains, "sepolia").unwrap().name, "Ethereum");
        assert_eq!(find_best_chain_match(&config.chains, "avax").unwrap().name, "Avalanche");
        assert_eq!(find_best_chain_match(&config.chains, "fuji").unwrap().name, "Avalanche");
        assert_eq!(find_best_chain_match(&config.chains, "42161").unwrap().name, "Arbitrum One");
        assert_eq!(find_best_chain_match(&config.chains, "amoy").unwrap().name, "Polygon");

        // Additional chain.json chains
        assert_eq!(find_best_chain_match(&config.chains, "celo").unwrap().name, "Celo");
        assert_eq!(find_best_chain_match(&config.chains, "zksync").unwrap().name, "zkSync Era");
        assert_eq!(find_best_chain_match(&config.chains, "klaytn").unwrap().name, "Kaia (formerly Klaytn)");
        assert_eq!(find_best_chain_match(&config.chains, "berachain").unwrap().name, "Berachain");

        // Misspelling / Fuzzy tests
        assert_eq!(find_best_chain_match(&config.chains, "ethreum").unwrap().name, "Ethereum");
        assert_eq!(find_best_chain_match(&config.chains, "avalance").unwrap().name, "Avalanche");
    }
}