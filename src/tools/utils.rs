use alloy::{primitives::Address, providers::Provider, sol};
use once_cell::sync::Lazy;
use regex::Regex;

sol! {
    #[sol(rpc)]
    interface IERC165 {
        function supportsInterface(bytes4 interfaceId) external view returns (bool);
    }
}

// REGEX fallback
static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\[\s*"[^"]+"\s*(,\s*"[^"]+"\s*)*\]"#).unwrap());

pub fn destructor_task(response_text: &str) -> Vec<String> {
    let clean_text = response_text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    if let Ok(val) = serde_json::from_str::<serde_json::Value>(clean_text) {
        if let Some(arr) = val.get("output").and_then(|v| v.as_array()) {
            let task_str = format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            return vec![task_str];
        } else if let Some(arr) = val.as_array() {
            let task_str = format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            return vec![task_str];
        }
    }

    // Fallback: extract bracketed array using regex
    RE.find_iter(response_text)
        .map(|caps| {
            caps.as_str()
                .chars()
                .filter(|&c| c != '\n' && c != ' ')
                .collect()
        })
        .collect()
}

// FUNCTION IMPLEMENTATIONS

pub fn extract_tool_params(input: &str) -> Option<(String, Vec<String>)> {
    let start = input.find('[')?;
    let end = input.find(']')?;

    let content = &input[start + 1..end];
    let mut parts = content
        .split(',')
        .map(|s| s.trim().trim_matches('"'))
        .filter(|s| !s.is_empty());

    let first = parts.next()?.to_string();
    let rest = parts.map(|s| s.to_string()).collect();

    Some((first, rest))
}

// NFT VERIFYING FUNCTIONS

pub async fn is_erc721_nft_contract(provider: &impl Provider, addr: Address) -> bool {
    let erc165 = IERC165::new(addr, provider);
    erc165
        .supportsInterface([0x80, 0xac, 0x58, 0xcd].into())
        .call()
        .await
        .unwrap_or(false)
}

pub async fn is_erc1155_nft_contract(provider: &impl Provider, addr: Address) -> bool {
    let erc165 = IERC165::new(addr, provider);
    erc165
        .supportsInterface([0xd9, 0xb6, 0x7a, 0x26].into())
        .call()
        .await
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destructor_task_and_extract_tool_params() {
        let json_reasoning = "```json\n{\n \"output\": [\n  \"switch_chain\",\n  \"arbitrium\"\n ]\n}\n```";
        let tasks = destructor_task(json_reasoning);
        assert_eq!(tasks.len(), 1);

        let (tool_name, params) = extract_tool_params(&tasks[0]).unwrap();
        assert_eq!(tool_name, "switch_chain");
        assert_eq!(params, vec!["arbitrium"]);
    }
}
