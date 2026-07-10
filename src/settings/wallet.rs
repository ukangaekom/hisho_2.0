use super::store;
use alloy::signers::local::PrivateKeySigner;
use dialoguer::{Password, Select, theme::ColorfulTheme};
use std::error::Error;

pub fn get_or_set_wallet() -> Result<String, Box<dyn Error>> {
    let mut config = store::load_config()?;

    if let Some(stored_pk) = config.wallet_pk {
        return Ok(stored_pk);
    }

    let options = vec![
        "Import Existing EVM Private Key",
        "Generate New Local Wallet",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your Wallet Setup Method")
        .default(0)
        .items(&options)
        .interact()?;

    let private_key_hex = match selection {
        0 => {
            let pk: String = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Insert your Wallet Private Key (Hex format)")
                .interact()?;
            pk.trim_start_matches("0x").to_string()
        }
        1 => {
            // Generate new wallet using Alloy
            let signer = PrivateKeySigner::random();
            let pk_hex = hex::encode(signer.to_bytes());
            println!("\x1b[32m✔ Wallet generated successfully.\x1b[0m");
            println!("Your Public Address: \x1b[1m{}\x1b[0m", signer.address());
            println!("(Your private key has been safely encrypted in your local config.)");
            pk_hex
        }
        _ => unreachable!(),
    };

    config.wallet_pk = Some(private_key_hex.clone());
    store::save_config(&config)?;

    println!("\x1b[32m✔ Wallet securely locked to local config.\x1b[0m");
    Ok(private_key_hex)
}

pub fn is_configured() -> bool {
    store::load_config()
        .map(|c| c.wallet_pk.is_some())
        .unwrap_or(false)
}

pub fn reset() -> Result<(), Box<dyn Error>> {
    let mut config = store::load_config()?;
    config.wallet_pk = None;
    store::save_config(&config)
}
