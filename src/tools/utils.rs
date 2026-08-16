use alloy::{primitives::Address, providers::Provider, sol};

sol! {
    #[sol(rpc)]
    interface IERC165 {
        function supportsInterface(bytes4 interfaceId) external view returns (bool);
    }
}

pub fn clean_str(s: &str) -> String {
    s.trim()
        .trim_matches(|c: char| c == '[' || c == ']' || c == '"' || c == '\'' || c == '`' || c == '\\')
        .trim()
        .to_string()
}

pub fn extract_tool_params(input: &str) -> Vec<(String, Vec<String>)> {
    let clean_text = input
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let mut results = Vec::new();

    // 1. Attempt JSON parsing
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(clean_text) {
        if parse_json_value(&val, &mut results) && !results.is_empty() {
            return results;
        }
    }

    // 2. Fallback to bracketed parsing: [tool_name, arg1, arg2]
    parse_bracketed_text(input, &mut results);

    results
}

fn parse_json_value(val: &serde_json::Value, results: &mut Vec<(String, Vec<String>)>) -> bool {
    if let Some(output) = val.get("output") {
        return parse_json_array_or_item(output, results);
    }
    parse_json_array_or_item(val, results)
}

fn parse_json_array_or_item(val: &serde_json::Value, results: &mut Vec<(String, Vec<String>)>) -> bool {
    if let Some(arr) = val.as_array() {
        if arr.is_empty() {
            return true;
        }
        let is_2d = arr.iter().any(|v| v.is_array());
        if is_2d {
            for item in arr {
                if let Some(sub_arr) = item.as_array() {
                    extract_from_json_slice(sub_arr, results);
                }
            }
        } else {
            extract_from_json_slice(arr, results);
        }
        return true;
    }
    false
}

fn extract_from_json_slice(slice: &[serde_json::Value], results: &mut Vec<(String, Vec<String>)>) {
    if slice.is_empty() {
        return;
    }
    let tool_name = match slice[0].as_str() {
        Some(s) => clean_str(s).to_lowercase(),
        None => clean_str(&slice[0].to_string()).to_lowercase(),
    };

    if tool_name.is_empty() {
        return;
    }

    let params: Vec<String> = slice[1..]
        .iter()
        .map(|v| match v.as_str() {
            Some(s) => clean_str(s),
            None => clean_str(&v.to_string()),
        })
        .collect();

    results.push((tool_name, params));
}

fn parse_bracketed_text(text: &str, results: &mut Vec<(String, Vec<String>)>) {
    let mut start_idx = None;
    let mut bracket_depth = 0;

    for (idx, ch) in text.char_indices() {
        if ch == '[' {
            if bracket_depth == 0 {
                start_idx = Some(idx + 1);
            }
            bracket_depth += 1;
        } else if ch == ']' {
            if bracket_depth > 0 {
                bracket_depth -= 1;
                if bracket_depth == 0 {
                    if let Some(start) = start_idx {
                        let content = &text[start..idx];
                        if content.contains('[') {
                            parse_bracketed_text(content, results);
                        } else {
                            parse_single_bracket_content(content, results);
                        }
                    }
                    start_idx = None;
                }
            }
        }
    }
}

fn parse_single_bracket_content(content: &str, results: &mut Vec<(String, Vec<String>)>) {
    let parts: Vec<&str> = content.split(',').collect();
    if parts.is_empty() {
        return;
    }

    let raw_tool = clean_str(parts[0]);
    if raw_tool.is_empty() {
        return;
    }

    let tool_name = raw_tool.to_lowercase();
    let params: Vec<String> = parts[1..].iter().map(|s| clean_str(s)).collect();

    results.push((tool_name, params));
}

pub fn destructor_task(response_text: &str) -> Vec<String> {
    let tool_calls = extract_tool_params(response_text);
    if tool_calls.is_empty() {
        Vec::new()
    } else {
        tool_calls
            .into_iter()
            .map(|(tool, params)| {
                if params.is_empty() {
                    format!("[{}]", tool)
                } else {
                    format!("[{}, {}]", tool, params.join(", "))
                }
            })
            .collect()
    }
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
        let json_reasoning = "```json\n{\n \"output\": [\n \"switch_chain\",\n \"arbitrium\"\n ]\n}\n```";
        let tasks = destructor_task(json_reasoning);
        assert_eq!(tasks.len(), 1);

        let calls = extract_tool_params(&tasks[0]);
        assert_eq!(calls.len(), 1);
        let (tool_name, params) = &calls[0];
        assert_eq!(tool_name, "switch_chain");
        assert_eq!(params, &vec!["arbitrium".to_string()]);
    }

    #[test]
    fn test_multiple_tool_calls_bracketed() {
        let text = "Reasoning: [switch_chain, Avalanche] and [get_price, AVAX]";
        let calls = extract_tool_params(text);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("switch_chain".to_string(), vec!["Avalanche".to_string()]));
        assert_eq!(calls[1], ("get_price".to_string(), vec!["AVAX".to_string()]));
    }

    #[test]
    fn test_2d_json_array() {
        let json_text = r#"{"output": [["get_price", "AVAX"], ["switch_chain", "Avalanche"]]}"#;
        let calls = extract_tool_params(json_text);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("get_price".to_string(), vec!["AVAX".to_string()]));
        assert_eq!(calls[1], ("switch_chain".to_string(), vec!["Avalanche".to_string()]));
    }
}
