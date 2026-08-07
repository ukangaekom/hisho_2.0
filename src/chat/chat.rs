use colored::*;
use std::io::{self, Write};

use crate::agents::*;
use crate::chat::ui::{
    render_command_help, render_quick_guide, render_reasoning_card, render_report_card,
    render_session_header, render_user_prompt, AnimatedSpinner,
};
use crate::settings::config::AppSettings;

pub async fn start() {
    // Load active network setting if available
    let active_network = AppSettings::fetch()
        .ok()
        .flatten()
        .map(|s| s.default_chain.name)
        .unwrap_or_else(|| "Mantle Sepolia".to_string());

    // Clear screen and render initial session HUD header
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let _ = io::stdout().flush();

    render_session_header(&active_network);
    render_quick_guide();

    let mut input = String::new();

    loop {
        print!(
            "{} ",
            "[💎 HISHO-AGENT] ❯".truecolor(0, 255, 170).bold()
        );
        let _ = io::stdout().flush();

        input.clear();

        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Error reading input.".red());
            break;
        }

        let text = input.trim();

        if text.is_empty() {
            continue;
        }

        // Handle Slash Commands
        if text.starts_with('/') {
            match text.to_lowercase().as_str() {
                "/exit" | "/quit" => {
                    println!(
                        "\n{}\n",
                        "🔒 Zeroized Keyring Locked. Safely shutting down Hisho Financial Agent..."
                            .truecolor(255, 180, 0)
                            .italic()
                    );
                    break;
                }
                "/help" => {
                    render_command_help();
                    continue;
                }
                "/clear" => {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    let _ = io::stdout().flush();
                    render_session_header(&active_network);
                    render_quick_guide();
                    continue;
                }
                "/status" | "/wallet" => {
                    println!("\n{}", "╭── 🛡️ ZEROIZED FINANCIAL VAULT STATUS ─────────────────────────╮".truecolor(0, 255, 170));
                    println!("│ {:<62} │", format!("Active Network : {}", active_network).bright_green().bold());
                    println!("│ {:<62} │", "Keyring Vault  : 🔒 AES-256-GCM (Argon2id Encrypted)".truecolor(255, 215, 0));
                    println!("│ {:<62} │", "Memory Policy  : 🛡️ ZeroizeOnDrop Heap Protection Active".bright_white());
                    println!("│ {:<62} │", "AI Engine      : ⚡ Gemini 2.5 Multi-Agent Orchestrator".truecolor(0, 225, 255));
                    println!("{}\n", "╰────────────────────────────────────────────────────────────────╯".truecolor(0, 255, 170));
                    continue;
                }
                _ => {
                    println!(
                        "   {} Unknown slash command. Type {} for available commands.\n",
                        "❌".red(),
                        "/help".cyan().bold()
                    );
                    continue;
                }
            }
        }

        // Standard exit commands
        if text.eq_ignore_ascii_case("exit") || text.eq_ignore_ascii_case("quit") {
            println!(
                "\n{}\n",
                "🔒 Zeroized Keyring Locked. Safely shutting down Hisho Financial Agent..."
                    .truecolor(255, 180, 0)
                    .italic()
            );
            break;
        }

        // 1. Render User Prompt Box
        render_user_prompt(text);

        // 2. Stage 1: Tactical Reasoning Engine
        let spinner = AnimatedSpinner::start("⚡ COMPUTING ON-CHAIN REASONING");
        let reasoning_res = processing_agent::process(text).await;
        spinner.stop().await;

        let reasoning_text = match reasoning_res {
            Ok(res) => res,
            Err(e) => format!("Reasoning Engine Error: {}", e),
        };

        render_reasoning_card(&reasoning_text);

        // 3. Stage 2: Financial Action & Report Synthesis
        let spinner = AnimatedSpinner::start("📊 SYNTHESIZING FINANCIAL REPORT & ACTION EXECUTION");
        let report_res = report_agent::report_result(&reasoning_text).await;
        spinner.stop().await;

        let report_text = match report_res {
            Ok(res) => res,
            Err(e) => format!("Report Engine Error: {}", e),
        };

        render_report_card(&report_text);
    }
}
