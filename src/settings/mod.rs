pub mod ai_engine;
pub mod chain;
pub mod rpc;
pub mod store;
pub mod wallet;

pub struct Settings {
    pub ai_engine_key: String,
    pub rpc_url: String,
    pub wallet_pk: String,
}

pub fn run_setup() -> Result<Settings, Box<dyn std::error::Error>> {
    println!("\nInitializing Interactive Setup...\n");
    let ai_engine_key = ai_engine::get_or_set_key()?;
    println!();
    let rpc_url = rpc::get_or_set_endpoint()?;
    println!();
    let wallet_pk = wallet::get_or_set_wallet()?;
    println!();

    Ok(Settings {
        ai_engine_key,
        rpc_url,
        wallet_pk,
    })
}

pub fn reset_all() -> Result<(), Box<dyn std::error::Error>> {
    let _ = ai_engine::reset();
    let _ = rpc::reset();
    let _ = wallet::reset();
    println!("\x1b[32m✔ All credentials cleared from keychain.\x1b[0m");
    Ok(())
}
