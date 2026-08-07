use bip39::{Language, Mnemonic};
use rand::RngCore;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureMnemonic {
    phrase: Zeroizing<String>,
}

impl SecureMnemonic {
    #[allow(dead_code)]
    pub fn new(phrase: String) -> Self {
        Self {
            phrase: Zeroizing::new(phrase),
        }
    }

    /// Generates a 24-word BIP-39 seed phrase using 256 bits of cryptographically secure entropy.
    pub fn generate_wallet() -> Result<Self, String> {
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut entropy);

        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)
            .map_err(|e| format!("Failed to generate Mnemonic: {}", e))?;
        entropy.zeroize();

        Ok(Self {
            phrase: Zeroizing::new(mnemonic.to_string()),
        })
    }

    /// Parses and validates an existing 24-word BIP-39 raw phrase.
    pub fn from_phrase(raw_phrase: String) -> Result<Self, String> {
        let _mnemonic = Mnemonic::parse_in_normalized(Language::English, &raw_phrase)
            .map_err(|e| format!("Invalid BIP-39 seed phrase: {}", e))?;

        Ok(Self {
            phrase: Zeroizing::new(raw_phrase),
        })
    }

    /// Returns a reference to the inner zeroed mnemonic phrase string.
    pub fn phrase(&self) -> &str {
        &self.phrase
    }
}

#[allow(dead_code)]
pub fn get_bitcoin_wallet_address(_mnemonic: &SecureMnemonic) -> Result<String, String> {
    // Stub for Bitcoin wallet derivation
    Err("Bitcoin wallet derivation not implemented yet".to_string())
}

#[allow(dead_code)]
pub fn get_evm_wallet_address(_mnemonic: &SecureMnemonic) -> Result<String, String> {
    // Stub for EVM wallet address derivation
    Err("EVM wallet derivation not implemented yet".to_string())
}

#[allow(dead_code)]
pub fn get_solana_wallet_address(_mnemonic: &SecureMnemonic) -> Result<String, String> {
    // Stub for Solana wallet derivation
    Err("Solana wallet derivation not implemented yet".to_string())
}
