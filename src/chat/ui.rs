use colored::*;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// Renders a premium financial agent HUD header at the top of the terminal session
pub fn render_session_header(active_network: &str) {
    let (cols, _) = crossterm::terminal::size().unwrap_or((100, 30));
    let width = (cols as usize).min(92);
    let inner_width = width.saturating_sub(4);

    let border_line = "─".repeat(inner_width);

    println!("\n{}", format!("┌─{}─┐", border_line).truecolor(0, 255, 136));
    println!(
        "│  {:<width$}  │",
        "⚡ HISHO ON-CHAIN FINANCIAL AGENT CONSOLE".truecolor(0, 255, 170).bold(),
        width = inner_width
    );
    println!(
        "│  {:<width$}  │",
        "Autonomous Agentic Reasoning • Zeroized Keyring Security • Multi-Chain Orchestration".truecolor(170, 170, 170).italic(),
        width = inner_width
    );
    println!("{}", format!("├─{}─┤", border_line).truecolor(0, 255, 136));
    
    let status_line = format!(
        "NET: {}  │  VAULT: {}  │  STATUS: {}",
        active_network.bright_green().bold(),
        "🔒 AES-256-GCM (ARGON2ID)".truecolor(255, 215, 0),
        "🟢 ONLINE".truecolor(0, 225, 255).bold()
    );
    println!("│  {:<width$}  │", status_line, width = inner_width);
    println!("{}\n", format!("└─{}─┘", border_line).truecolor(0, 255, 136));
}

/// Renders quick prompt guide and commands info
pub fn render_quick_guide() {
    println!(
        "   {} {}",
        "💡 TIP:".truecolor(255, 215, 0).bold(),
        "Type your financial query or use slash commands: /help /status /wallet /clear /exit"
            .truecolor(180, 180, 180)
    );
    println!();
}

/// Renders a styled User Prompt box
pub fn render_user_prompt(text: &str) {
    let (cols, _) = crossterm::terminal::size().unwrap_or((100, 30));
    let width = (cols as usize).min(92);
    let border_len = width.saturating_sub(20);

    println!(
        "\n{}",
        format!("┌── 👤 YOUR ON-CHAIN QUERY {}┐", "─".repeat(border_len)).truecolor(0, 225, 255)
    );
    for line in text.lines() {
        println!("│  {}", line.bright_white().bold());
    }
    println!(
        "{}",
        format!("└{}┘", "─".repeat(width.saturating_sub(2))).truecolor(0, 225, 255)
    );
}

/// Renders Tactical Reasoning Engine Output in a styled golden box
pub fn render_reasoning_card(reasoning: &str) {
    let (cols, _) = crossterm::terminal::size().unwrap_or((100, 30));
    let width = (cols as usize).min(92);
    let border_len = width.saturating_sub(29);

    println!(
        "\n{}",
        format!("┌── 🧠 TACTICAL AGENT REASONING {}┐", "─".repeat(border_len)).truecolor(255, 200, 0)
    );

    for line in reasoning.lines() {
        if line.trim().is_empty() {
            println!("│");
        } else {
            println!("│  {}", line.truecolor(255, 240, 160));
        }
    }

    println!(
        "{}",
        format!("└{}┘", "─".repeat(width.saturating_sub(2))).truecolor(255, 200, 0)
    );
}

/// Renders On-Chain Financial Action & Analysis Report in a glowing neon mint green box
pub fn render_report_card(report: &str) {
    let (cols, _) = crossterm::terminal::size().unwrap_or((100, 30));
    let width = (cols as usize).min(92);
    let border_len = width.saturating_sub(31);

    println!(
        "\n{}",
        format!("┌── 📊 FINANCIAL ACTION & REPORT {}┐", "─".repeat(border_len)).truecolor(0, 255, 136)
    );

    for line in report.lines() {
        if line.trim().is_empty() {
            println!("│");
        } else {
            println!("│  {}", line.bright_white().bold());
        }
    }

    println!(
        "{}\n",
        format!("└{}┘", "─".repeat(width.saturating_sub(2))).truecolor(0, 255, 136)
    );
}

/// Renders an interactive slash command directory table
pub fn render_command_help() {
    println!("\n{}", "╭── 📜 HISHO COMMAND DIRECTORY ─────────────────────────────────╮".truecolor(0, 225, 255));
    println!("│  {:<62} │", "/help     - Display this command directory".bright_white());
    println!("│  {:<62} │", "/status   - Show financial wallet & zeroized key status".bright_white());
    println!("│  {:<62} │", "/wallet   - View active wallet state & OS keyring health".bright_white());
    println!("│  {:<62} │", "/clear    - Clear console screen & refresh HUD".bright_white());
    println!("│  {:<62} │", "/exit     - Safely lock vault & exit session".bright_white());
    println!("{}\n", "╰────────────────────────────────────────────────────────────────╯".truecolor(0, 225, 255));
}

/// High-tech animated terminal spinner for async tasks
pub struct AnimatedSpinner {
    stop_flag: Arc<AtomicBool>,
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl AnimatedSpinner {
    pub fn start(message: &'static str) -> Self {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop_flag);

        let handle = tokio::spawn(async move {
            let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut idx = 0;
            while !stop_clone.load(Ordering::Relaxed) {
                let frame = spinner_frames[idx % spinner_frames.len()];
                print!(
                    "\r  {} {} {}",
                    frame.truecolor(0, 255, 136).bold(),
                    message.truecolor(255, 215, 0).bold(),
                    "..."
                );
                let _ = io::stdout().flush();
                idx += 1;
                sleep(Duration::from_millis(70)).await;
            }
            // Clear spinner line cleanly
            print!("\r{}\r", " ".repeat(90));
            let _ = io::stdout().flush();
        });

        Self {
            stop_flag,
            handle: Some(handle),
        }
    }

    pub async fn stop(mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.await;
        }
    }
}
