//! Two-Factor Authentication (TOTP).
//!
//! Provides TOTP generation compatible with Google Authenticator.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::auth::totp::{Totp, TotpConfig};
//!
//! let totp = Totp::new("user@example.com", "MyApp");
//!
//! // Get QR code URL for setup
//! let qr_url = totp.qr_code_url();
//!
//! // Verify code from authenticator
//! if totp.verify("123456") {
//!     // 2FA passed
//! }
//! ```

use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

/// TOTP configuration.
#[derive(Debug, Clone)]
pub struct TotpConfig {
    /// Secret key (base32 encoded).
    pub secret: String,
    /// Issuer name (app name).
    pub issuer: String,
    /// Account name (usually email).
    pub account: String,
    /// Digits in OTP (default 6).
    pub digits: u32,
    /// Time step in seconds (default 30).
    pub period: u64,
    /// Algorithm (default SHA1).
    pub algorithm: String,
}

impl TotpConfig {
    /// Create a new TOTP config with generated secret.
    pub fn new(account: &str, issuer: &str) -> Self {
        Self {
            secret: generate_secret(),
            issuer: issuer.to_string(),
            account: account.to_string(),
            digits: 6,
            period: 30,
            algorithm: "SHA1".to_string(),
        }
    }

    /// Create with existing secret.
    pub fn with_secret(account: &str, issuer: &str, secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
            issuer: issuer.to_string(),
            account: account.to_string(),
            digits: 6,
            period: 30,
            algorithm: "SHA1".to_string(),
        }
    }
}

/// TOTP generator and verifier.
pub struct Totp {
    config: TotpConfig,
}

impl Totp {
    /// Create a new TOTP instance with generated secret.
    pub fn new(account: &str, issuer: &str) -> Self {
        Self {
            config: TotpConfig::new(account, issuer),
        }
    }

    /// Create from existing config.
    pub fn from_config(config: TotpConfig) -> Self {
        Self { config }
    }

    /// Create with existing secret.
    pub fn with_secret(account: &str, issuer: &str, secret: &str) -> Self {
        Self {
            config: TotpConfig::with_secret(account, issuer, secret),
        }
    }

    /// Get the secret (for storage).
    pub fn secret(&self) -> &str {
        &self.config.secret
    }

    /// Generate current OTP code.
    pub fn generate(&self) -> String {
        self.generate_at(current_timestamp())
    }

    /// Generate OTP at specific timestamp.
    pub fn generate_at(&self, timestamp: u64) -> String {
        let counter = timestamp / self.config.period;
        self.generate_hotp(counter)
    }

    /// Generate HOTP (counter-based).
    fn generate_hotp(&self, counter: u64) -> String {
        let secret_bytes = base32_decode(&self.config.secret);
        let counter_bytes = counter.to_be_bytes();

        // HMAC can take keys of any size, but we handle the error safely
        let mut mac = match Hmac::<Sha1>::new_from_slice(&secret_bytes) {
            Ok(mac) => mac,
            Err(_) => {
                tracing::error!("Failed to create HMAC instance");
                return "000000".to_string(); // Return safe default
            }
        };
        mac.update(&counter_bytes);
        let result = mac.finalize().into_bytes();

        let offset = (result[19] & 0x0f) as usize;
        let binary = ((result[offset] & 0x7f) as u32) << 24
            | (result[offset + 1] as u32) << 16
            | (result[offset + 2] as u32) << 8
            | (result[offset + 3] as u32);

        let otp = binary % 10u32.pow(self.config.digits);
        format!("{:0>width$}", otp, width = self.config.digits as usize)
    }

    /// Verify an OTP code (allows 1 step drift).
    pub fn verify(&self, code: &str) -> bool {
        self.verify_with_window(code, 1)
    }

    /// Verify with custom time window.
    pub fn verify_with_window(&self, code: &str, window: u64) -> bool {
        let timestamp = current_timestamp();
        let current_counter = timestamp / self.config.period;

        for i in 0..=window {
            if self.generate_hotp(current_counter - i) == code {
                return true;
            }
            if i > 0 && self.generate_hotp(current_counter + i) == code {
                return true;
            }
        }

        false
    }

    /// Get the otpauth:// URI for QR codes.
    pub fn otpauth_url(&self) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm={}&digits={}&period={}",
            urlencoding::encode(&self.config.issuer),
            urlencoding::encode(&self.config.account),
            self.config.secret,
            urlencoding::encode(&self.config.issuer),
            self.config.algorithm,
            self.config.digits,
            self.config.period
        )
    }

    /// Get Google Charts QR code URL.
    pub fn qr_code_url(&self) -> String {
        let otpauth = self.otpauth_url();
        format!(
            "https://chart.googleapis.com/chart?chs=200x200&chld=M|0&cht=qr&chl={}",
            urlencoding::encode(&otpauth)
        )
    }
}

/// Generate a random base32 secret.
pub fn generate_secret() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..20).map(|_| rng.random()).collect();
    base32_encode(&bytes)
}

/// Generate backup codes.
pub fn generate_backup_codes(count: usize) -> Vec<String> {
    use rand::Rng;
    let mut rng = rand::rng();
    (0..count)
        .map(|_| {
            let code: u32 = rng.random_range(10000000..99999999);
            format!("{}-{}", &code.to_string()[..4], &code.to_string()[4..])
        })
        .collect()
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| {
            tracing::warn!("System time is before UNIX epoch, using 0");
            std::time::Duration::from_secs(0)
        })
        .as_secs()
}

// Base32 encoding/decoding (RFC 4648)
const BASE32_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

fn base32_encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut buffer: u64 = 0;
    let mut bits = 0;

    for &byte in data {
        buffer = (buffer << 8) | (byte as u64);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            let index = ((buffer >> bits) & 0x1f) as usize;
            result.push(BASE32_ALPHABET[index] as char);
        }
    }

    if bits > 0 {
        let index = ((buffer << (5 - bits)) & 0x1f) as usize;
        result.push(BASE32_ALPHABET[index] as char);
    }

    result
}

fn base32_decode(encoded: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut buffer: u64 = 0;
    let mut bits = 0;

    for c in encoded.to_uppercase().chars() {
        let value = match c {
            'A'..='Z' => c as u64 - 'A' as u64,
            '2'..='7' => c as u64 - '2' as u64 + 26,
            _ => continue,
        };

        buffer = (buffer << 5) | value;
        bits += 5;

        if bits >= 8 {
            bits -= 8;
            result.push((buffer >> bits) as u8);
        }
    }

    result
}
