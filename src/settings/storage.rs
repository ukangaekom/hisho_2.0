use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use keyring::Entry;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use zeroize::{Zeroize, Zeroizing};

use crate::settings::config::AppSettings;
use crate::settings::wallet::SecureMnemonic;

const SERVICE_NAME: &str = "hisho";
const KEYCHAIN_WALLET_BUNDLE: &str = "hisho_wallet_bundle";

#[derive(Serialize, Deserialize)]
struct WalletBundle {
    salt: String,
    pin_hash: String,
    encrypted_seed_hex: String,
}

fn get_config_path() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", "hisho", "hisho") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
        Ok(config_dir.join("config.json"))
    } else {
        Ok(PathBuf::from("config.json"))
    }
}

pub fn save_app_settings(config: &AppSettings) -> Result<(), String> {
    let path = get_config_path()?;
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(path, json).map_err(|e| format!("Failed to save config file: {}", e))
}

pub fn load_app_settings() -> Result<Option<AppSettings>, String> {
    let path = get_config_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
    let config: AppSettings = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;
    Ok(Some(config))
}

// Hardening of Memory
pub fn harden_process_memory() {
    #[cfg(target_family = "unix")]
    unsafe {
        #[cfg(target_os = "linux")]
        {
            // Disable core dumps on Linux to prevent RAM forensics from crash states
            libc::prctl(libc::PR_SET_DUMPABLE, 0, 0, 0, 0);
        }
        let limit = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        libc::setrlimit(libc::RLIMIT_CORE, &limit);
    }
}

/// Checks if an encrypted wallet bundle is already anchored in the OS Keyring.
pub fn has_wallet() -> bool {
    let entry = Entry::new(SERVICE_NAME, KEYCHAIN_WALLET_BUNDLE);
    if let Ok(entry) = entry {
        if let Ok(pwd) = entry.get_password() {
            return !pwd.trim().is_empty();
        }
    }
    false
}

/// Key derivation helper using Argon2id to map PIN + Salt to a 32-byte encryption key.
fn derive_key_from_pin(pin: &str, salt_str: &str) -> Result<Zeroizing<[u8; 32]>, String> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(pin.as_bytes(), salt_str.as_bytes(), &mut key)
        .map_err(|e| format!("Argon2 key derivation error: {}", e))?;
    Ok(Zeroizing::new(key))
}

/// Encrypts and atomically writes the seed phrase to the OS Keyring protected by a System PIN.
pub fn save_wallet_with_pin(mnemonic: &SecureMnemonic, pin: &str) -> Result<(), String> {
    if has_wallet() {
        return Err("Wallet already exists. Seed phrases cannot be overwritten or changed.".to_string());
    }

    if pin.trim().len() < 4 {
        return Err("System PIN must be at least 4 characters long.".to_string());
    }

    harden_process_memory();

    let salt = SaltString::generate(&mut rand::thread_rng());
    let salt_str = salt.as_str().to_string();

    let argon2 = Argon2::default();
    let pin_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash PIN: {}", e))?
        .to_string();

    let derived_key = derive_key_from_pin(pin, &salt_str)?;

    let cipher = Aes256Gcm::new_from_slice(&*derived_key)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, mnemonic.phrase().as_bytes())
        .map_err(|e| format!("Encryption error: {}", e))?;

    let mut payload = nonce_bytes.to_vec();
    payload.extend(ciphertext);
    let hex_payload = hex::encode(payload);

    // Bundle into a single payload to guarantee atomic state writes
    let bundle = WalletBundle {
        salt: salt_str,
        pin_hash,
        encrypted_seed_hex: hex_payload,
    };

    let bundle_json = serde_json::to_string(&bundle)
        .map_err(|e| format!("Failed to serialize wallet bundle: {}", e))?;

    Entry::new(SERVICE_NAME, KEYCHAIN_WALLET_BUNDLE)
        .map_err(|e| e.to_string())?
        .set_password(&bundle_json)
        .map_err(|e| format!("Failed to store atomic wallet bundle in OS Keyring: {}", e))?;

    Ok(())
}

/// Authenticates the System PIN, decrypts the vault, and returns a SecureMnemonic.
pub fn view_wallet_with_pin(pin: &str) -> Result<SecureMnemonic, String> {
    if !has_wallet() {
        return Err("No wallet found in system. Please initialize a wallet first.".to_string());
    }

    harden_process_memory();

    let bundle_json = Entry::new(SERVICE_NAME, KEYCHAIN_WALLET_BUNDLE)
        .map_err(|e| e.to_string())?
        .get_password()
        .map_err(|e| format!("Failed to load wallet bundle from OS Keyring: {}", e))?;

    let bundle: WalletBundle = serde_json::from_str(&bundle_json)
        .map_err(|e| format!("Failed to parse wallet bundle: {}", e))?;

    let parsed_hash = PasswordHash::new(&bundle.pin_hash)
        .map_err(|e| format!("Invalid stored password hash: {}", e))?;

    if Argon2::default()
        .verify_password(pin.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err("Incorrect System PIN! Access denied.".to_string());
    }

    let payload = hex::decode(&bundle.encrypted_seed_hex)
        .map_err(|e| format!("Failed to decode encrypted seed payload: {}", e))?;

    if payload.len() < 12 {
        return Err("Corrupted seed phrase storage bundle.".to_string());
    }

    let (nonce_bytes, ciphertext) = payload.split_at(12);
    let derived_key = derive_key_from_pin(pin, &bundle.salt)?;

    let cipher = Aes256Gcm::new_from_slice(&*derived_key)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;

    let nonce = Nonce::from_slice(nonce_bytes);
    
    let mut decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    let raw_phrase = String::from_utf8(decrypted_bytes.clone())
        .map_err(|e| format!("Invalid UTF-8 seed phrase: {}", e))?;

    decrypted_bytes.zeroize();

    SecureMnemonic::from_phrase(raw_phrase)
}