//! Cryptography utilities.

use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};

/// Hash a password with bcrypt.
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(Into::into)
}

/// Verify a password against a bcrypt hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

/// Generate SHA-256 hash of a string.
pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Encode bytes to base64.
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Decode base64 to bytes.
pub fn base64_decode(encoded: &str) -> anyhow::Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(encoded)
        .map_err(Into::into)
}

/// Encode string to base64 URL-safe format.
pub fn base64_url_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Decode base64 URL-safe format.
pub fn base64_url_decode(encoded: &str) -> anyhow::Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(Into::into)
}

/// Generate a secure random token.
pub fn generate_token(length: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    hex::encode(bytes)
}

/// Generate a short verification code (6 digits).
pub fn generate_verification_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1000000))
}

/// Hash for cache keys (short, fast).
pub fn cache_key_hash(input: &str) -> String {
    let hash = sha256(input);
    hash[..16].to_string()
}

/// HMAC-SHA256 signature.
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key length error");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "secret123";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash));
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn test_sha256() {
        let result = sha256("hello");
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_base64() {
        let data = b"hello world";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(data.to_vec(), decoded);
    }
}
