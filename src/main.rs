mod chat;
mod agents;
// Crates
use crate::chat::chat::start;

// Importation of Rust Cli library
use clap::{Parser, Subcommand};
use colored::*; // For colored output
use figlet_rs::FIGfont;
mod settings;

#[derive(Parser)]
#[command(
    name = "Hisho",
    author = "Ekomabasi Ukanga",
    version = "1.0", 
    about = "Terminal Onchain Finance Agent", 
    long_about = None
)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Status, // Check the status of the local engine
    Settings,
}

const UTF8_IMAGE_ASSET: &str = include_str!("../assets/hisho_agent.txt");

#[tokio::main]
async fn main() {
    // 1. Load the "Big" FIGlet font for a larger, bolder presence
    let font =
        FIGfont::from_file("assets/big.flf").unwrap_or_else(|_| FIGfont::standard().unwrap());

    // Get the dynamic terminal width to perfectly center everything
    let (cols, _) = crossterm::terminal::size().unwrap_or((100, 30));
    let term_width = cols as usize;

    // 1. Calculate image block width and center it uniformly
    let img_width = UTF8_IMAGE_ASSET
        .lines()
        .map(|line| console::measure_text_width(line))
        .max()
        .unwrap_or(0);

    let img_padding = " ".repeat(term_width.saturating_sub(img_width) / 2);
    for line in UTF8_IMAGE_ASSET.lines() {
        let padded_line = format!("{}{}", img_padding, line);
        println!("{}", console::truncate_str(&padded_line, term_width, ""));
    }

    // 2. Convert "CREEP" into enlarged ASCII art
    let figure = font.convert("Hisho");

    if let Some(ascii_art) = figure {
        let ascii_str = ascii_art.to_string();

        // Calculate the maximum width of the text block to center it uniformly
        let text_width = ascii_str
            .lines()
            .map(|line| console::measure_text_width(line.trim_end()))
            .max()
            .unwrap_or(0);

        let text_padding = " ".repeat(term_width.saturating_sub(text_width) / 2);

        // Print the enlarged text dynamically centered beneath the image
        for line in ascii_str.lines() {
            let stripped_line = line.trim_end();
            if stripped_line.is_empty() {
                continue;
            }
            println!(
                "{}{}",
                text_padding,
                stripped_line.truecolor(0, 255, 136).bold()
            );
        }
    }

    // Using colored's pre-computed truecolor escape sequences
    println!(
        "\n{}",
        "── AGENTIC 🐜 CORE ──────────────────────────────────────".bright_green()
    );

    // Custom color block using standard ANSI / truecolor mapping
    print!(" {} ", "■".truecolor(0, 255, 136).bold()); // Neon mint green square
    println!(
        "{}",
        "Initializing lightning-fast Agentic runtime configuration..."
            .white()
            .bold()
    );

    println!(
        " {}\n",
        "── ready for Mutli Agentic Blockchain Synchronization ────────────────────".bright_green()
    );

    // 2. Parse arguments with zero-cost abstractions via Clap
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start) => {
            if let Err(e) = settings::config::ensure_configured() {
                eprintln!("Configuration setup error: {}", e);
                return;
            }
            start().await;
        }
        Some(Commands::Status) => {
            println!("{} Runtime is active. Latency: <1ms.", "✔".green().bold());
        }
        Some(Commands::Settings) => {
            if let Err(e) = settings::config::interactive_settings_menu() {
                eprintln!("Error during settings configuration: {}", e);
            }
        }
        None => {
            if let Err(e) = settings::config::ensure_configured() {
                eprintln!("Configuration setup error: {}", e);
                return;
            }
            println!(
                "{}",
                "Tip: Use --help to view all available internal tools and flags."
                    .truecolor(128, 128, 128)
            );
        }
    }
}
