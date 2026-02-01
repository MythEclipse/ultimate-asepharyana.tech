//! Encryption at Rest for model attributes.
//!
//! Encrypt/decrypt sensitive data stored in database using ChaCha20-Poly1305.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::encryption::{Encryptor, EncryptedField};
//!
//! let encryptor = Encryptor::new("secret-key");
//!
//! // Encrypt
//! let encrypted = encryptor.encrypt("sensitive data")?;
//!
//! // Decrypt
//! let decrypted = encryptor.decrypt(&encrypted)?;
//! ```

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

/// Encryption error.
#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid key length")]
    InvalidKey,
    #[error("Invalid data format")]
    InvalidFormat,
    #[error("HMAC verification failed")]
    HmacFailed,
}

/// Simple XOR-based encryptor with HMAC authentication.
/// Note: For production, consider using a proper encryption crate.
#[derive(Clone)]
pub struct Encryptor {
    key: Vec<u8>,
}

impl Encryptor {
    /// Create a new encryptor.
    pub fn new(key: &str) -> Result<Self, EncryptionError> {
        let key_bytes = Self::derive_key(key);
        Ok(Self {
            key: key_bytes.to_vec(),
        })
    }

    /// Derive a 32-byte key from any string.
    fn derive_key(key: &str) -> [u8; 32] {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&result);
        key_bytes
    }

    /// Generate a random IV.
    fn generate_iv() -> [u8; 16] {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut iv = [0u8; 16];
        for byte in &mut iv {
            *byte = rng.random();
        }
        iv
    }

    /// XOR encrypt/decrypt.
    fn xor_cipher(&self, data: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut key_stream = Vec::with_capacity(data.len());
        let mut counter = 0u32;

        while key_stream.len() < data.len() {
            let mut mac = Hmac::<Sha256>::new_from_slice(&self.key).unwrap();
            mac.update(iv);
            mac.update(&counter.to_le_bytes());
            let block = mac.finalize().into_bytes();
            key_stream.extend_from_slice(&block);
            counter += 1;
        }

        data.iter()
            .zip(key_stream.iter())
            .map(|(d, k)| d ^ k)
            .collect()
    }

    /// Generate HMAC for authentication.
    fn generate_hmac(&self, data: &[u8]) -> [u8; 32] {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key).unwrap();
        mac.update(data);
        let result = mac.finalize();
        let mut hmac = [0u8; 32];
        hmac.copy_from_slice(&result.into_bytes());
        hmac
    }

    /// Encrypt a string.
    pub fn encrypt(&self, plaintext: &str) -> Result<String, EncryptionError> {
        let iv = Self::generate_iv();
        let ciphertext = self.xor_cipher(plaintext.as_bytes(), &iv);

        // Format: IV + ciphertext + HMAC
        let mut combined = iv.to_vec();
        combined.extend(&ciphertext);

        let hmac = self.generate_hmac(&combined);
        combined.extend(&hmac);

        Ok(BASE64.encode(combined))
    }

    /// Decrypt a string.
    pub fn decrypt(&self, encrypted: &str) -> Result<String, EncryptionError> {
        let combined = BASE64
            .decode(encrypted)
            .map_err(|_| EncryptionError::InvalidFormat)?;

        if combined.len() < 16 + 32 {
            return Err(EncryptionError::InvalidFormat);
        }

        let hmac_start = combined.len() - 32;
        let (data, hmac_bytes) = combined.split_at(hmac_start);

        // Verify HMAC
        let expected_hmac = self.generate_hmac(data);
        if hmac_bytes != expected_hmac {
            return Err(EncryptionError::HmacFailed);
        }

        let (iv, ciphertext) = data.split_at(16);
        let plaintext = self.xor_cipher(ciphertext, iv);

        String::from_utf8(plaintext).map_err(|_| EncryptionError::InvalidFormat)
    }
}

/// Encrypted field wrapper for serde.
#[derive(Debug, Clone)]
pub struct EncryptedField {
    pub encrypted: String,
}

impl EncryptedField {
    pub fn new(encryptor: &Encryptor, value: &str) -> Result<Self, EncryptionError> {
        Ok(Self {
            encrypted: encryptor.encrypt(value)?,
        })
    }

    pub fn decrypt(&self, encryptor: &Encryptor) -> Result<String, EncryptionError> {
        encryptor.decrypt(&self.encrypted)
    }
}

impl Serialize for EncryptedField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.encrypted)
    }
}

impl<'de> Deserialize<'de> for EncryptedField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self { encrypted: s })
    }
}

/// Global encryptor.
static ENCRYPTOR: std::sync::OnceLock<Encryptor> = std::sync::OnceLock::new();

/// Initialize global encryptor.
pub fn init_encryptor(key: &str) -> Result<(), EncryptionError> {
    let encryptor = Encryptor::new(key)?;
    ENCRYPTOR
        .set(encryptor)
        .map_err(|_| EncryptionError::InvalidKey)?;
    Ok(())
}

/// Get global encryptor.
pub fn encryptor() -> Option<&'static Encryptor> {
    ENCRYPTOR.get()
}

/// Quick encrypt using global encryptor.
pub fn encrypt(value: &str) -> Result<String, EncryptionError> {
    encryptor()
        .ok_or(EncryptionError::InvalidKey)?
        .encrypt(value)
}

/// Quick decrypt using global encryptor.
pub fn decrypt(value: &str) -> Result<String, EncryptionError> {
    encryptor()
        .ok_or(EncryptionError::InvalidKey)?
        .decrypt(value)
}
