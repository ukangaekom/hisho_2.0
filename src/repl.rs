use colored::*;
use std::io::{self, Write};
// use crate::settings;
// use crate::agents::processing_agent;

pub async fn start() {
    println!("{}", "Initializing secure credentials...".cyan());

    // Load and inject credentials instantly into the environment for libraries to use
    if let Ok(gemini_key) = settings::ai_engine::get_or_set_key() {
        unsafe {
            std::env::set_var("GEMINI_API_KEY", gemini_key);
        }
    }

    // Ensure RPC and Wallet are configured so provider can use them
    let _ = settings::rpc::get_or_set_endpoint();
    let _ = settings::wallet::get_or_set_wallet();

    println!(
        "{}",
        "Connected to Hisho. Type 'exit' or 'quit' to terminate.\n".bright_green()
    );

    let mut input = String::new();

    loop {
        print!("{}", "User > ".bright_cyan().bold());
        io::stdout().flush().unwrap();

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let text = input.trim();

                if text.eq_ignore_ascii_case("exit") || text.eq_ignore_ascii_case("quit") {
                    println!("{}", "Shutting down agent runtime...".bright_black());
                    break;
                }

                if text.is_empty() {
                    continue;
                }

                println!("{}", "Agent > Thinking...".bright_black());

                // Call processing agent
                match processing_agent::process(text).await {
                    Some(response) => {
                        println!("{} {}\n", "Agent >".bright_green().bold(), response);
                    }
                    None => {
                        println!(
                            "{}\n",
                            "Agent > Error processing request. Please try again.".red()
                        );
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}
