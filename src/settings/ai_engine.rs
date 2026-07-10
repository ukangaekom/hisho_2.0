use dialoguer::{theme::ColorfulTheme, Password};
use std::error::Error;
use super::store;

pub fn get_or_set_key() -> Result<String, Box<dyn Error>> {
    let mut config = store::load_config()?;

    if let Some(stored_key) = config.gemini_api_key {
        return Ok(stored_key);
    }

    let input: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your Gemini API Key")
        .interact()?;

    config.gemini_api_key = Some(input.clone());
    store::save_config(&config)?;
    
    println!("\x1b[32m✔ Gemini API Key securely locked to local config.\x1b[0m");
    Ok(input)
}

pub fn is_configured() -> bool {
    store::load_config()
        .map(|c| c.gemini_api_key.is_some())
        .unwrap_or(false)
}

pub fn reset() -> Result<(), Box<dyn Error>> {
    let mut config = store::load_config()?;
    config.gemini_api_key = None;
    store::save_config(&config)
}