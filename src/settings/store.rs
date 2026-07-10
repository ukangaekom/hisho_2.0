use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfig {
    pub gemini_api_key: Option<String>,
    pub rpc_url: Option<String>,
    pub wallet_pk: Option<String>,
}

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "mantle-agent-cli") {
        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }
        Ok(config_dir.join("config.json"))
    } else {
        Err("Could not determine user configuration directory".into())
    }
}

pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let path = get_config_path()?;
    if path.exists() {
        let data = fs::read_to_string(&path)?;
        let config: AppConfig = serde_json::from_str(&data).unwrap_or_default();
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let path = get_config_path()?;
    let data = serde_json::to_string_pretty(config)?;
    fs::write(&path, data)?;

    // Attempt to set secure file permissions on unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o600); // Read/Write for owner only
        let _ = fs::set_permissions(&path, perms);
    }

    Ok(())
}
