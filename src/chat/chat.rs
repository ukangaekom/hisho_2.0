use colored::*;
use std::io::{self, Write};

use crate::chat::ui::{
    get_chain_identifiers, render_command_help, render_quick_guide, render_reasoning_card,
    render_report_card, render_session_header, render_user_prompt, AnimatedSpinner,
};
use crate::settings::config::get_public_wallet_address;
use hisho::tools::tools_map::TOOLS;
use hisho::tools::utils::{destructor_task, extract_tool_params};

pub async fn start() {
    // Clear screen and render initial session HUD header
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let _ = io::stdout().flush();

    let (mainnet_info, testnet_info) = get_chain_identifiers();
    render_session_header(&mainnet_info, &testnet_info);
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
                    let (cur_main, cur_test) = get_chain_identifiers();
                    render_session_header(&cur_main, &cur_test);
                    render_quick_guide();
                    continue;
                }
                "/status" | "/wallet" => {
                    let (cur_main, cur_test) = get_chain_identifiers();
                    let wallet_addr = get_public_wallet_address();
                    println!(
                        "\n{}",
                        "╭── 🛡️ ZEROIZED FINANCIAL VAULT STATUS ─────────────────────────╮"
                            .truecolor(0, 255, 170)
                    );
                    println!(
                        "│ {:<62} │",
                        format!("Active Mainnet : {}", cur_main)
                            .bright_green()
                            .bold()
                    );
                    println!(
                        "│ {:<62} │",
                        format!("Active Testnet : {}", cur_test)
                            .truecolor(0, 225, 255)
                            .bold()
                    );
                    println!(
                        "│ {:<62} │",
                        format!("Wallet Address : {}", wallet_addr)
                            .bright_yellow()
                            .bold()
                    );
                    println!(
                        "│ {:<62} │",
                        "Keyring Vault  : 🔒 AES-256-GCM (Argon2id Encrypted)"
                            .truecolor(255, 215, 0)
                    );
                    println!(
                        "│ {:<62} │",
                        "Memory Policy  : 🛡️ ZeroizeOnDrop Heap Protection Active"
                            .bright_white()
                    );
                    println!(
                        "│ {:<62} │",
                        "AI Engine      : ⚡ Gemini 2.5 Multi-Agent Orchestrator"
                            .truecolor(0, 225, 255)
                    );
                    println!(
                        "{}\n",
                        "╰────────────────────────────────────────────────────────────────╯"
                            .truecolor(0, 255, 170)
                    );
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
        let reasoning_res = crate::agents::processing_agent::process(text).await;
        spinner.stop().await;

        let reasoning_text = match reasoning_res {
            Ok(res) => res,
            Err(e) => format!("Reasoning Engine Error: {}", e),
        };

        render_reasoning_card(&reasoning_text);

        // 3. Stage 2: Destructor Task & Tool Execution
        let agent_tasks = destructor_task(&reasoning_text);
        let mut tool_execution_outputs: Vec<String> = Vec::new();

        for task_str in agent_tasks {
            if let Some((tool_name, parameters)) = extract_tool_params(&task_str) {
                let params_ref: Vec<&str> = parameters.iter().map(|s| s.as_str()).collect();
                if let Some(tool_func) = TOOLS.get(tool_name.as_str()) {
                    let spinner = AnimatedSpinner::start("⚡ EXECUTING TOOL ACTION");
                    let result = tool_func(&params_ref).await;
                    spinner.stop().await;
                    tool_execution_outputs.push(format!("Tool [{}] Output: {}", tool_name, result));
                } else {
                    tool_execution_outputs.push(format!("Tool [{}] not found in tool registry.", tool_name));
                }
            }
        }

        // 4. Stage 3: Financial Action & Report Synthesis
        let report_input = if !tool_execution_outputs.is_empty() {
            format!(
                "User Query: {}\nTactical Reasoning: {}\nTool Execution Results:\n{}",
                text,
                reasoning_text,
                tool_execution_outputs.join("\n")
            )
        } else {
            reasoning_text.clone()
        };

        let spinner = AnimatedSpinner::start("📊 SYNTHESIZING FINANCIAL REPORT & ACTION EXECUTION");
        let report_res = crate::agents::report_agent::report_result(&report_input).await;
        spinner.stop().await;

        let report_text = match report_res {
            Ok(res) => res,
            Err(e) => format!("Report Engine Error: {}", e),
        };

        render_report_card(&report_text);
    }
}
