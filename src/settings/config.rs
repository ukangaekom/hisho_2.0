use colored::*;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use inquire::{Password, Select, Text};
use serde::{Deserialize, Serialize};

use crate::settings::chain::{AppConfig, Blockchain};
use crate::settings::storage;
use crate::settings::wallet::{get_evm_wallet_address, SecureMnemonic};

fn shiny_render_config() -> RenderConfig<'static> {
    let mut config = RenderConfig::default_colored();
    config.prompt_prefix = Styled::new("⚡").with_fg(Color::LightYellow);
    config.highlighted_option_prefix = Styled::new(" 🌟 ❯ ").with_fg(Color::LightYellow);
    config.selected_option = Some(StyleSheet::new().with_fg(Color::White));
    config.help_message = StyleSheet::new().with_fg(Color::LightCyan);
    config
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub default_chain: Blockchain,
    pub custom_rpc: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    // Kept to read and migrate configurations created before model selection.
    pub gemini_api_key: Option<String>,
    #[serde(default)]
    pub wallet_address: Option<String>,
}

const MODEL_FAMILIES: &[&str] = &[
    "OpenAI",
    "OpenAI Responses",
    "Anthropic",
    "Gemini",
    "xAI",
    "DeepSeek",
    "Moonshot / Kimi",
    "Zai",
    "Cohere",
    "Mimo",
    "OpenCode Go",
    "Atlas Cloud",
    "Qwen Cloud",
    "Fireworks",
    "Ollama (local fallback)",
];

const DEFAULT_MODEL: &str = "gemini-2.5-flash";

fn api_key_env_name(model_name: &str) -> Option<&'static str> {
    let model = model_name.to_ascii_lowercase();
    if model.starts_with("opencode_go::") {
        Some("OPENCODE_GO_API_KEY")
    } else if model.starts_with("atlascloud::") {
        Some("ATLAS_CLOUD_API_KEY")
    } else if model.starts_with("qwen_cloud::") {
        Some("QWEN_CLOUD_API_KEY")
    } else if model.contains("fireworks") {
        Some("FIREWORKS_API_KEY")
    } else if model.starts_with("claude-") {
        Some("ANTHROPIC_API_KEY")
    } else if model.starts_with("gemini-") {
        Some("GEMINI_API_KEY")
    } else if model.starts_with("grok-") {
        Some("XAI_API_KEY")
    } else if model.starts_with("deepseek-") {
        Some("DEEPSEEK_API_KEY")
    } else if model.starts_with("moonshot-") || model.starts_with("kimi") {
        Some("MOONSHOT_API_KEY")
    } else if model.starts_with("glm-") {
        Some("ZAI_API_KEY")
    } else if model.starts_with("command-") || model.starts_with("embed-") {
        Some("COHERE_API_KEY")
    } else if model.starts_with("mimo-") {
        Some("MIMO_API_KEY")
    } else if model.starts_with("gpt-")
        || model.starts_with("o1-")
        || model.starts_with("o3-")
        || model.starts_with("o4-")
        || model.starts_with("chatgpt-")
        || model.starts_with("codex-")
    {
        Some("OPENAI_API_KEY")
    } else {
        None
    }
}

fn has_ai_api_key(settings: &AppSettings, model_name: &str) -> bool {
    let Some(env_name) = api_key_env_name(model_name) else {
        return true;
    };
    std::env::var(env_name)
        .ok()
        .is_some_and(|key| !key.trim().is_empty())
        || settings
            .api_key
            .as_deref()
            .is_some_and(|key| !key.trim().is_empty())
        || (env_name == "GEMINI_API_KEY"
            && settings
                .gemini_api_key
                .as_deref()
                .is_some_and(|key| !key.trim().is_empty()))
}

fn model_matches_family(family: &str, model_name: &str) -> bool {
    let model = model_name.to_ascii_lowercase();
    match family {
        "OpenAI" => {
            (model.starts_with("gpt-")
                && !model.starts_with("gpt-5")
                && !model.starts_with("gpt-6")
                && !model.contains("codex")
                && !model.contains("pro"))
                || model.starts_with("o1-")
                || model.starts_with("o3-")
                || model.starts_with("o4-")
                || model.starts_with("chatgpt-")
                || model.starts_with("codex-")
        }
        "OpenAI Responses" => {
            model.starts_with("gpt-5")
                || model.starts_with("gpt-6")
                || (model.starts_with("gpt-")
                    && (model.contains("codex") || model.contains("pro")))
        }
        "Anthropic" => model.starts_with("claude-"),
        "Gemini" => model.starts_with("gemini-"),
        "xAI" => model.starts_with("grok-"),
        "DeepSeek" => model.starts_with("deepseek-"),
        "Moonshot / Kimi" => model.starts_with("moonshot-") || model.starts_with("kimi"),
        "Zai" => model.starts_with("glm-"),
        "Cohere" => model.starts_with("command-") || model.starts_with("embed-"),
        "Mimo" => model.starts_with("mimo-"),
        "OpenCode Go" => model.starts_with("opencode_go::"),
        "Atlas Cloud" => model.starts_with("atlascloud::"),
        "Qwen Cloud" => model.starts_with("qwen_cloud::"),
        "Fireworks" => model.contains("fireworks"),
        _ => api_key_env_name(&model).is_none(),
    }
}

pub fn apply_configured_ai_key(settings: &AppSettings) {
    let model_name = settings.model_name.as_deref().unwrap_or(DEFAULT_MODEL);
    let Some(env_name) = api_key_env_name(model_name) else {
        return;
    };
    let key = std::env::var(env_name)
        .ok()
        .filter(|key| !key.trim().is_empty())
        .or_else(|| settings.api_key.clone())
        .or_else(|| {
            if env_name == "GEMINI_API_KEY" {
                settings.gemini_api_key.clone()
            } else {
                None
            }
        });
    if let Some(key) = key {
        unsafe {
            std::env::set_var(env_name, &key);
            if model_name.starts_with("qwen_cloud::") {
                std::env::set_var("ALIYUN_API_KEY", key);
            }
        }
    }
}

pub fn resolve_genai_model_name(model_name: &str) -> Result<String, String> {
    if model_name.starts_with("gpt-6-") {
        return Ok(format!("openai_resp::{}", model_name));
    }
    if let Some(model) = model_name.strip_prefix("qwen_cloud::") {
        return Ok(format!("aliyun::{}", model));
    }
    if model_name.starts_with("kimi") {
        return Ok(format!("moonshot::{}", model_name));
    }
    if model_name.starts_with("atlascloud::") {
        return Err(
            "Atlas Cloud is not supported by the installed genai adapter. Select another model provider."
                .to_string(),
        );
    }
    Ok(model_name.to_string())
}

fn prompt_ai_settings(current_model: Option<&str>) -> Result<(String, Option<String>), String> {
    render_banner("AI MODEL AND API KEY");
    let family = Select::new("Choose a model provider:", MODEL_FAMILIES.to_vec())
        .with_render_config(shiny_render_config())
        .with_page_size(10)
        .prompt()
        .map_err(|e| format!("Model provider selection cancelled: {}", e))?;

    let model_hint = match family {
        "OpenAI" => "gpt-*, o1-*, o3-*, o4-*, chatgpt-*, or codex-*",
        "OpenAI Responses" => "gpt-5-*, gpt-6-*, or gpt-* containing codex or pro",
        "Anthropic" => "claude-*",
        "Gemini" => "gemini-*",
        "xAI" => "grok-*",
        "DeepSeek" => "deepseek-*",
        "Moonshot / Kimi" => "moonshot-* or kimi*",
        "Zai" => "glm-*",
        "Cohere" => "command-* or embed-*",
        "Mimo" => "mimo-*",
        "OpenCode Go" => "opencode_go::model-name",
        "Atlas Cloud" => "atlascloud::model-name",
        "Qwen Cloud" => "qwen_cloud::model-name",
        "Fireworks" => "model name containing fireworks",
        _ => "Any other model name (uses local Ollama)",
    };
    let model_prompt = Text::new("Enter the exact model name:").with_help_message(model_hint);
    let model_prompt = if let Some(model) = current_model.filter(|model| model_matches_family(family, model)) {
        model_prompt.with_default(model)
    } else {
        model_prompt
    };
    let model_name = model_prompt
        .prompt()
        .map_err(|e| format!("Model name entry cancelled: {}", e))?;
    let model_name = model_name.trim().to_string();
    if model_name.is_empty() {
        return Err("Model name cannot be empty.".to_string());
    }
    if !model_matches_family(family, &model_name) {
        return Err(format!("Model '{}' does not match the selected {} family.", model_name, family));
    }

    let key_env = api_key_env_name(&model_name);
    let entered_key = if key_env.is_some() {
        Password::new("Enter API key (leave blank to use the environment key):")
            .without_confirmation()
            .prompt()
            .ok()
            .filter(|key| !key.trim().is_empty())
    } else {
        None
    };

    Ok((model_name, entered_key))
}

fn update_ai_settings(settings: &mut AppSettings) -> Result<(), String> {
    let previous_model = settings.model_name.clone();
    let previous_key_env = settings.model_name.as_deref().and_then(api_key_env_name);
    let (model_name, api_key) = prompt_ai_settings(settings.model_name.as_deref())?;
    let selected_key_env = api_key_env_name(&model_name);
    settings.model_name = Some(model_name);
    settings.api_key = api_key.or_else(|| {
        if previous_key_env == selected_key_env {
            settings.api_key.clone()
        } else if previous_model.is_none() && selected_key_env == Some("GEMINI_API_KEY") {
            settings.gemini_api_key.clone()
        } else {
            None
        }
    });
    settings.gemini_api_key = if settings.model_name.as_deref().and_then(api_key_env_name)
        == Some("GEMINI_API_KEY")
    {
        settings.api_key.clone()
    } else {
        None
    };
    if !has_ai_api_key(settings, settings.model_name.as_deref().unwrap_or(DEFAULT_MODEL)) {
        return Err("An API key is required for the selected model. Configure it in Settings or the provider environment variable.".to_string());
    }
    apply_configured_ai_key(settings);
    storage::save_app_settings(settings)
}

impl AppSettings {
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), String> {
        storage::save_app_settings(self)
    }

    #[allow(dead_code)]
    pub fn fetch() -> Result<Option<Self>, String> {
        storage::load_app_settings()
    }
}

/// Renders a styled section banner box
fn render_banner(title: &str) {
    let width: usize = 64;
    let title_line = format!("── {} ", title);
    let fill_len = width.saturating_sub(title_line.chars().count());
    let line = format!("{}{}", title_line, "─".repeat(fill_len));
    println!("\n{}", line.truecolor(0, 255, 136).bold());
}

/// Renders a stylized section title box
fn render_card(title: &str, subtitle: &str) {
    println!("\n{}", "╭──────────────────────────────────────────────────────────────╮".truecolor(0, 225, 255));
    println!(
        "│ {:<60} │",
        title.bright_white().bold()
    );
    println!(
        "│ {:<60} │",
        subtitle.truecolor(160, 160, 160).italic()
    );
    println!("{}", "╰──────────────────────────────────────────────────────────────╯".truecolor(0, 225, 255));
}

/// Helper function to retrieve the configured public EVM wallet address without requiring PIN.
pub fn get_public_wallet_address() -> String {
    if let Ok(Some(settings)) = storage::load_app_settings() {
        if let Some(ref addr) = settings.wallet_address {
            if !addr.trim().is_empty() {
                return addr.clone();
            }
        }
    }
    "No wallet address configured in system settings.".to_string()
}

/// Ensures the application is configured. Runs setup wizard if settings or wallet are missing.
pub fn ensure_configured() -> Result<AppSettings, String> {
    if let Ok(Some(mut existing_settings)) = storage::load_app_settings() {
        if storage::has_wallet() {
            // One-time auto-migration for existing wallets without cached public address
            if existing_settings.wallet_address.is_none() {
                render_banner("ONE-TIME PUBLIC WALLET ADDRESS MIGRATION");
                println!(
                    "{}",
                    "ℹ Seed phrase detected in OS Keyring. Enter your System PIN once to cache your public EVM address for PIN-free viewing."
                        .truecolor(0, 225, 255)
                );
                let pin_res = Password::new("Enter System PIN to cache public EVM wallet address:")
                    .without_confirmation()
                    .prompt();
                if let Ok(pin) = pin_res {
                    if let Ok(mnemonic) = storage::view_wallet_with_pin(&pin) {
                        if let Ok(addr) = get_evm_wallet_address(&mnemonic) {
                            existing_settings.wallet_address = Some(addr.clone());
                            let _ = storage::save_app_settings(&existing_settings);
                            println!(
                                "   {} Public EVM wallet address ({}) cached successfully for PIN-free viewing.",
                                "✔".truecolor(0, 255, 136).bold(),
                                addr.bright_green()
                            );
                        }
                    }
                }
            }

            let configured_model = existing_settings
                .model_name
                .as_deref()
                .unwrap_or(DEFAULT_MODEL);
            if existing_settings.model_name.is_none()
                || !has_ai_api_key(&existing_settings, configured_model)
            {
                update_ai_settings(&mut existing_settings)?;
            } else {
                apply_configured_ai_key(&existing_settings);
            }

            return Ok(existing_settings);
        }
    }

    render_card(
        "⚡ HISHO AGENT SYSTEM SETUP WIZARD",
        "Configure your network, zeroized wallet, and AI engine",
    );

    let config_data = AppConfig::load_default()?;
    if config_data.chains.is_empty() {
        return Err("No chains available in chain.json".to_string());
    }

    // 1. Chain Selection
    render_banner("STEP 1: BLOCKCHAIN SELECTION");
    let selected_chain = Select::new(
        "Choose default blockchain (Mainnet & Testnet auto-loaded):",
        config_data.chains.clone(),
    )
    .with_render_config(shiny_render_config())
    .with_page_size(10)
    .with_help_message("Use ↑↓ arrows to navigate • Enter to confirm selection")
    .prompt()
    .map_err(|e| format!("Chain selection cancelled: {}", e))?;

    println!(
        "   {} Active Chain: {} (Mainnet: {}, Testnet: {})",
        "✔".truecolor(0, 255, 136).bold(),
        selected_chain.name.bright_green().bold(),
        selected_chain.mainnet.rpc_url.dimmed(),
        selected_chain.testnet.rpc_url.dimmed()
    );

    // 2. Seed Phrase & System PIN Setup
    render_banner("STEP 2: ZEROIZED SEED PHRASE & SYSTEM PIN");
    let derived_wallet_address = if !storage::has_wallet() {
        println!(
            "{}",
            "🔒 Generating 24-word seed phrase protected inside OS Keyring..."
                .truecolor(255, 215, 0)
                .italic()
        );

        let pin = loop {
            let p1 = Password::new("Create System PIN to lock your seed phrase:")
                .prompt()
                .map_err(|e| format!("PIN creation cancelled: {}", e))?;

            if p1.trim().len() < 4 {
                println!("   {}", "❌ PIN must be at least 4 characters long.".red());
                continue;
            }

            let p2 = Password::new("Confirm System PIN:")
                .prompt()
                .map_err(|e| format!("PIN confirmation cancelled: {}", e))?;

            if p1 == p2 {
                break p1;
            } else {
                println!("   {}", "❌ PINs do not match. Please try again.".red());
            }
        };

        let mnemonic = SecureMnemonic::generate_wallet()?;
        let addr = get_evm_wallet_address(&mnemonic).ok();
        storage::save_wallet_with_pin(&mnemonic, &pin)?;

        println!(
            "   {}\n",
            "✔ 24-word Seed phrase generated and locked inside OS Keyring."
                .truecolor(0, 255, 136)
                .bold()
        );
        addr
    } else {
        println!(
            "   {}\n",
            "✔ Wallet seed phrase already configured and secured in OS Keyring."
                .truecolor(0, 255, 136)
                .bold()
        );
        None
    };

    // 3. AI Model and API Key Setup
    let (model_name, api_key) = prompt_ai_settings(None)?;
    let gemini_api_key = if api_key_env_name(&model_name) == Some("GEMINI_API_KEY") {
        api_key.clone()
    } else {
        None
    };

    let settings = AppSettings {
        default_chain: selected_chain,
        custom_rpc: None,
        model_name: Some(model_name),
        api_key,
        gemini_api_key,
        wallet_address: derived_wallet_address,
    };

    if !has_ai_api_key(&settings, settings.model_name.as_deref().unwrap_or(DEFAULT_MODEL)) {
        return Err("An API key is required for the selected model. Configure it in Settings or the provider environment variable.".to_string());
    }
    apply_configured_ai_key(&settings);
    storage::save_app_settings(&settings)?;
    println!(
        "\n{}\n",
        "✨ Initialization complete! Hisho is ready for operation."
            .truecolor(0, 255, 136)
            .bold()
    );

    Ok(settings)
}

/// Interactive Settings Menu for viewing/updating configuration and viewing seed phrase.
pub fn interactive_settings_menu() -> Result<(), String> {
    let mut settings = match storage::load_app_settings()? {
        Some(s) => s,
        None => ensure_configured()?,
    };

    loop {
        render_card(
            "⚙ HISHO DASHBOARD & SYSTEM SETTINGS",
            "Manage active network, security, AI model, and seed phrase",
        );

        let options = vec![
            "🌐  1. Switch Active Network (Mainnet & Testnet)",
            "🤖  2. Configure AI Model and API Key",
            "🔒  3. View Wallet Seed Phrase (Requires System PIN)",
            "📊  4. Display System Status & Configuration",
            "🚪  5. Exit Settings Menu",
        ];

        let choice = Select::new("Select configuration option:", options)
            .with_page_size(8)
            .prompt()
            .map_err(|e| format!("Menu cancelled: {}", e))?;

        if choice.starts_with("🌐") {
            let config_data = AppConfig::load_default()?;
            let new_chain = Select::new(
                "Select new default blockchain:",
                config_data.chains,
            )
            .with_render_config(shiny_render_config())
            .with_page_size(10)
            .prompt()
            .map_err(|e| format!("Selection cancelled: {}", e))?;

            settings.default_chain = new_chain.clone();
            storage::save_app_settings(&settings)?;
            println!(
                "\n   {} Active blockchain updated to: {}",
                "✔".truecolor(0, 255, 136).bold(),
                new_chain.name.bright_green().bold()
            );
        } else if choice.starts_with("🤖") {
            update_ai_settings(&mut settings)?;
            println!("\n   {} AI model and API key updated.", "✔".truecolor(0, 255, 136).bold());
        } else if choice.starts_with("🔒") {
            if !storage::has_wallet() {
                println!("\n   {}", "❌ No wallet found in system.".red());
                continue;
            }

            let pin = Password::new("Enter System PIN to unlock seed phrase:")
                .prompt()
                .map_err(|e| format!("PIN prompt cancelled: {}", e))?;

            match storage::view_wallet_with_pin(&pin) {
                Ok(mnemonic) => {
                    if let Ok(addr) = get_evm_wallet_address(&mnemonic) {
                        settings.wallet_address = Some(addr);
                        let _ = storage::save_app_settings(&settings);
                    }
                    println!("\n{}", "╭── 🔒 CONFIDENTIAL 24-WORD SEED PHRASE ──────────────────────╮".truecolor(255, 60, 60));
                    println!(
                        "│ {:<60} │",
                        "WARNING: Never disclose these words to anyone!".yellow().bold()
                    );
                    println!("{}", "├──────────────────────────────────────────────────────────────┤".truecolor(255, 60, 60));
                    
                    let words: Vec<&str> = mnemonic.phrase().split_whitespace().collect();
                    for chunk in words.chunks(4) {
                        let line = chunk.join(" ");
                        println!("│  {:<58}  │", line.bright_white().bold());
                    }
                    println!("{}", "╰──────────────────────────────────────────────────────────────╯".truecolor(255, 60, 60));
                }
                Err(e) => {
                    println!("\n   {} {}", "❌".red().bold(), e.red());
                }
            }
        } else if choice.starts_with("📊") {
            let testnet_name = settings.default_chain.testnet.label.as_deref().unwrap_or("Testnet");
            let wallet_display = settings.wallet_address.as_deref().unwrap_or("🔒 Seed in Vault (Address Uncached)");
            
            println!("\n{}", "╭── 📊 SYSTEM STATUS & CONFIGURATION CARD ────────────────────╮".truecolor(0, 200, 255));
            println!("│ {:<60} │", format!("Active Network   : {}", settings.default_chain.name).bright_white().bold());
            println!("│ {:<60} │", format!("Mainnet Chain ID : {}", settings.default_chain.mainnet.chain_id).dimmed());
            println!("│ {:<60} │", format!("Mainnet RPC      : {}", settings.default_chain.mainnet.rpc_url).dimmed());
            println!("│ {:<60} │", format!("Testnet Name     : {}", testnet_name).dimmed());
            println!("│ {:<60} │", format!("Testnet Chain ID : {}", settings.default_chain.testnet.chain_id).dimmed());
            println!("│ {:<60} │", format!("Testnet RPC      : {}", settings.default_chain.testnet.rpc_url).dimmed());
            println!("{}", "├──────────────────────────────────────────────────────────────┤".truecolor(0, 200, 255));
            println!("│ {:<60} │", format!("AI Model         : {}", settings.model_name.as_deref().unwrap_or(DEFAULT_MODEL)));
            println!("│ {:<60} │", format!("AI API Key       : {}", if has_ai_api_key(&settings, settings.model_name.as_deref().unwrap_or(DEFAULT_MODEL)) { "● Active (Configured)" } else { "○ Not Set" }));
            println!("│ {:<60} │", format!("Wallet Address   : {}", wallet_display).bright_green());
            println!("{}", "╰──────────────────────────────────────────────────────────────╯".truecolor(0, 200, 255));
        } else if choice.starts_with("🚪") {
            println!("\nExiting settings menu.");
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{api_key_env_name, model_matches_family, resolve_genai_model_name};

    #[test]
    fn model_names_map_to_provider_api_keys() {
        assert_eq!(api_key_env_name("gpt-5-mini"), Some("OPENAI_API_KEY"));
        assert_eq!(api_key_env_name("claude-sonnet-4"), Some("ANTHROPIC_API_KEY"));
        assert_eq!(api_key_env_name("kimi-k2"), Some("MOONSHOT_API_KEY"));
        assert_eq!(api_key_env_name("qwen_cloud::qwen-plus"), Some("QWEN_CLOUD_API_KEY"));
        assert_eq!(api_key_env_name("atlascloud::model"), Some("ATLAS_CLOUD_API_KEY"));
        assert_eq!(api_key_env_name("llama3.2"), None);
    }

    #[test]
    fn model_names_resolve_to_installed_provider_routes() {
        assert_eq!(
            resolve_genai_model_name("qwen_cloud::qwen-plus").unwrap(),
            "aliyun::qwen-plus"
        );
        assert_eq!(
            resolve_genai_model_name("kimi-k2").unwrap(),
            "moonshot::kimi-k2"
        );
        assert_eq!(
            resolve_genai_model_name("gpt-6-mini").unwrap(),
            "openai_resp::gpt-6-mini"
        );
        assert!(resolve_genai_model_name("atlascloud::model").is_err());
    }

    #[test]
    fn model_names_match_selected_families() {
        assert!(model_matches_family("OpenAI", "gpt-4.1"));
        assert!(model_matches_family("OpenAI Responses", "gpt-5-mini"));
        assert!(model_matches_family("OpenAI Responses", "gpt-4.1-pro"));
        assert!(model_matches_family("Qwen Cloud", "qwen_cloud::qwen-plus"));
        assert!(!model_matches_family("Gemini", "claude-sonnet-4"));
    }
}