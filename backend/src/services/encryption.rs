// backend/src/services/encryption.rs

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use aes_gcm::aes::cipher::InvalidLength;
use rand::rngs::OsRng;
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

use std::fmt;
use std::error::Error;

/// Local error type that wraps the different error kinds we may see.
#[derive(Debug)]
pub enum EncryptionError {
    Aes(aes_gcm::Error),
    Base64(base64::DecodeError),
    InvalidData(&'static str),
    InvalidLength(InvalidLength),
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionError::Aes(e) => write!(f, "AES error: {:?}", e),
            EncryptionError::Base64(e) => write!(f, "Base64 decode error: {}", e),
            EncryptionError::InvalidData(s) => write!(f, "Invalid data: {}", s),
            EncryptionError::InvalidLength(e) => write!(f, "Invalid length: {:?}", e),
        }
    }
}

impl Error for EncryptionError {}

impl From<aes_gcm::Error> for EncryptionError {
    fn from(e: aes_gcm::Error) -> Self {
        EncryptionError::Aes(e)
    }
}

impl From<base64::DecodeError> for EncryptionError {
    fn from(e: base64::DecodeError) -> Self {
        EncryptionError::Base64(e)
    }
}

impl From<InvalidLength> for EncryptionError {
    fn from(e: InvalidLength) -> Self {
        EncryptionError::InvalidLength(e)
    }
}

/// Encrypt amount using AES-256-GCM
/// Returns (base64(nonce || ciphertext), hex(nonce))
pub fn encrypt_amount(
    amount: i128,
    master_key: &str,
) -> Result<(String, String), EncryptionError> {
    // Convert amount to bytes (little-endian to match your original)
    let amount_bytes = amount.to_le_bytes();

    // Generate 12-byte nonce (GCM standard)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes); // note: this may emit a dependency warning in your toolchain

    // Derive 32-byte key (simple truncation/pad as in your original)
    let key_bytes = derive_key(master_key);

    // Construct cipher. new_from_slice returns InvalidLength on wrong key size
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;

    // Encrypt plaintext
    let ciphertext = cipher.encrypt(nonce, amount_bytes.as_ref())?;

    // Serialize nonce + ciphertext together and base64-encode
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted_data);

    // Also return nonce as hex string (your "decryption_key")
    let decryption_key = hex::encode(&nonce_bytes);

    Ok((encrypted_base64, decryption_key))
}

/// Decrypt amount using AES-256-GCM
/// `decryption_key` is expected to be hex-encoded nonce (kept to match your API)
pub fn decrypt_amount(
    encrypted_base64: &str,
    _decryption_key: &str,
    master_key: &str,
) -> Result<i128, EncryptionError> {
    // Base64 decode
    let encrypted_data = general_purpose::STANDARD.decode(encrypted_base64)?;

    // Must contain at least 12 bytes of nonce
    if encrypted_data.len() < 12 {
        return Err(EncryptionError::InvalidData("encrypted data too short"));
    }

    // Split nonce and ciphertext
    let nonce_bytes = &encrypted_data[..12];
    let ciphertext = &encrypted_data[12..];
    let nonce = Nonce::from_slice(nonce_bytes);

    // Derive key and construct cipher
    let key_bytes = derive_key(master_key);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;

    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    // Expect 16 bytes for i128
    if plaintext.len() != 16 {
        return Err(EncryptionError::InvalidData("invalid plaintext length"));
    }

    let mut amount_bytes = [0u8; 16];
    amount_bytes.copy_from_slice(&plaintext);
    let amount = i128::from_le_bytes(amount_bytes);

    Ok(amount)
}

/// Simple 32-byte key derivation (pad/truncate).
/// For production use, prefer a proper KDF (HKDF / Argon2 / PBKDF2) or SHA-256.
fn derive_key(master_key: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let bytes = master_key.as_bytes();
    let len = bytes.len().min(32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let amount = 150_000i128;
        let master_key = "test-master-key-32-chars-long!!";

        let (encrypted, _nonce_hex) = encrypt_amount(amount, master_key).unwrap();
        let decrypted = decrypt_amount(&encrypted, &_nonce_hex, master_key).unwrap();

        assert_eq!(amount, decrypted);
    }
}
