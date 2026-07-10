use dialoguer::{theme::ColorfulTheme, Input, Select, Password};
use std::error::Error;
use super::store;

pub fn get_or_set_endpoint() -> Result<String, Box<dyn Error>> {
    let mut config = store::load_config()?;

    if let Some(stored_url) = config.rpc_url {
        return Ok(stored_url);
    }

    let networks = vec![
        "Alchemy RPC",
        "QuickNode RPC",
        "Infura RPC",
        "Custom RPC (Testnet/Mainnet)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your Mantle RPC Provider")
        .default(0)
        .items(&networks)
        .interact()?;

    let final_url = match selection {
        0 => {
            let api_key: String = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Alchemy API Key")
                .interact()?;
            format!("https://mantle-mainnet.g.alchemy.com/v2/{}", api_key)
        }
        1 => {
            let endpoint: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your QuickNode Endpoint (e.g., https://xxx.quiknode.pro/abc/)")
                .interact()?;
            endpoint
        }
        2 => {
            let api_key: String = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Infura API Key")
                .interact()?;
            format!("https://mantle-mainnet.infura.io/v3/{}", api_key)
        }
        3 => {
            let custom_url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Custom Mantle RPC URL")
                .interact()?;
            custom_url
        }
        _ => unreachable!(),
    };

    config.rpc_url = Some(final_url.clone());
    store::save_config(&config)?;
    
    println!("\x1b[32m✔ RPC Endpoint securely configured.\x1b[0m");
    Ok(final_url)
}

pub fn is_configured() -> bool {
    store::load_config()
        .map(|c| c.rpc_url.is_some())
        .unwrap_or(false)
}

pub fn reset() -> Result<(), Box<dyn Error>> {
    let mut config = store::load_config()?;
    config.rpc_url = None;
    store::save_config(&config)
}