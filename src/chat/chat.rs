use colored::*;
use std::io::{self, Write};

// Crates
use crate::agents::*;


pub async fn start() {
    let mut input = String::new();

    loop {
        print!("{}", "User >> ".yellow().bold());
        io::stdout().flush().unwrap();

        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let text = input.trim();

                if text.eq_ignore_ascii_case("exit") || text.eq_ignore_ascii_case("quit") {
                    println!("\n\n{}", "shutting down agent runtime.....".bright_black());
                    break;
                }

                if text.is_empty() {
                    continue;
                }

                let processing_agent_response = processing_agent::process(text).await.expect("Unable to get reasoning");
                let report_agent_response = report_agent::report_result(&processing_agent_response).await.expect("Unable to get reports");

                println!("{}", "Model > Thinking..........".bright_yellow());

                // Color the user's input and the model's response separately
                println!("{} {}\n\n", "You >".green().bold(), text.green());
                println!("{} {}\n\n", "Hisho's Reasoning>".yellow().bold(), processing_agent_response.cyan());
                println!("{} {}\n\n", "Hisho's Reports>".yellow().bold(), report_agent_response.cyan());
            }   

            Err(error) => {
                println!("Error: {}", error.to_string().red());
                break;
            }
        }
    }
}
