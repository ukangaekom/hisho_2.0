use colored::*;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use inquire::{Password, Select, Text};
use serde::{Deserialize, Serialize};

use crate::settings::chain::{AppConfig, Blockchain};
use crate::settings::storage;
use crate::settings::wallet::SecureMnemonic;

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
    pub gemini_api_key: Option<String>,
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

/// Ensures the application is configured. Runs setup wizard if settings or wallet are missing.
pub fn ensure_configured() -> Result<AppSettings, String> {
    if let Ok(Some(existing_settings)) = storage::load_app_settings() {
        if storage::has_wallet() {
            if let Some(ref key) = existing_settings.gemini_api_key {
                unsafe {
                    std::env::set_var("GEMINI_API_KEY", key);
                }
            } else if let Ok(env_key) = std::env::var("GEMINI_API_KEY") {
                let mut updated = existing_settings.clone();
                updated.gemini_api_key = Some(env_key.clone());
                unsafe {
                    std::env::set_var("GEMINI_API_KEY", env_key);
                }
                let _ = storage::save_app_settings(&updated);
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
    if !storage::has_wallet() {
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
        storage::save_wallet_with_pin(&mnemonic, &pin)?;

        println!(
            "   {}\n",
            "✔ 24-word Seed phrase generated and locked inside OS Keyring."
                .truecolor(0, 255, 136)
                .bold()
        );
    } else {
        println!(
            "   {}\n",
            "✔ Wallet seed phrase already configured and secured in OS Keyring."
                .truecolor(0, 255, 136)
                .bold()
        );
    }

    // 3. Gemini API Key Setup
    render_banner("STEP 3: GEMINI AI ENGINE API KEY");
    let gemini_key = Text::new("Enter Gemini API Key (press Enter to skip):")
        .prompt()
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| std::env::var("GEMINI_API_KEY").ok());

    if let Some(ref key) = gemini_key {
        unsafe {
            std::env::set_var("GEMINI_API_KEY", key);
        }
        println!("   {} Gemini API Key stored.", "✔".truecolor(0, 255, 136).bold());
    } else {
        println!(
            "   {}",
            "ℹ Gemini API Key skipped. You can set it anytime in settings."
                .truecolor(255, 180, 0)
        );
    }

    let settings = AppSettings {
        default_chain: selected_chain,
        custom_rpc: None,
        gemini_api_key: gemini_key,
    };

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
            "Manage active network, security, AI key, and seed phrase",
        );

        let options = vec![
            "🌐  1. Switch Active Network (Mainnet & Testnet)",
            "🤖  2. Configure Gemini API Key",
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
            let new_key = Text::new("Enter new Gemini API Key:")
                .prompt()
                .ok()
                .filter(|s| !s.trim().is_empty());

            settings.gemini_api_key = new_key;
            storage::save_app_settings(&settings)?;
            println!("\n   {} Gemini API Key updated.", "✔".truecolor(0, 255, 136).bold());
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
            
            println!("\n{}", "╭── 📊 SYSTEM STATUS & CONFIGURATION CARD ────────────────────╮".truecolor(0, 200, 255));
            println!("│ {:<60} │", format!("Active Network   : {}", settings.default_chain.name).bright_white().bold());
            println!("│ {:<60} │", format!("Mainnet Chain ID : {}", settings.default_chain.mainnet.chain_id).dimmed());
            println!("│ {:<60} │", format!("Mainnet RPC      : {}", settings.default_chain.mainnet.rpc_url).dimmed());
            println!("│ {:<60} │", format!("Testnet Name     : {}", testnet_name).dimmed());
            println!("│ {:<60} │", format!("Testnet Chain ID : {}", settings.default_chain.testnet.chain_id).dimmed());
            println!("│ {:<60} │", format!("Testnet RPC      : {}", settings.default_chain.testnet.rpc_url).dimmed());
            println!("{}", "├──────────────────────────────────────────────────────────────┤".truecolor(0, 200, 255));
            println!("│ {:<60} │", format!("Gemini API Key   : {}", if settings.gemini_api_key.is_some() { "● Active (Configured)" } else { "○ Not Set" }));
            println!("│ {:<60} │", format!("Wallet Seed      : {}", if storage::has_wallet() { "🔒 Locked in OS Zeroized Keyring" } else { "❌ Missing" }));
            println!("{}", "╰──────────────────────────────────────────────────────────────╯".truecolor(0, 200, 255));
        } else if choice.starts_with("🚪") {
            println!("\nExiting settings menu.");
            break;
        }
    }

    Ok(())
}