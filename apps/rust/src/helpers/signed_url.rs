//! Signed URLs for temporary secure access.
//!
//! Generate URLs with expiration and signature verification.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::signed_url::{SignedUrl, SignedUrlConfig};
//!
//! let signer = SignedUrl::new("my-secret-key");
//!
//! // Sign a URL with 1 hour expiration
//! let signed = signer.sign("/files/secret.pdf", 3600);
//!
//! // Verify a signed URL
//! if signer.verify(&signed) {
//!     // URL is valid and not expired
//! }
//! ```

use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

/// Signed URL configuration.
#[derive(Debug, Clone)]
pub struct SignedUrlConfig {
    /// Secret key for signing.
    pub secret: String,
    /// Base URL prefix (optional).
    pub base_url: Option<String>,
    /// Parameter name for signature.
    pub signature_param: String,
    /// Parameter name for expiration.
    pub expires_param: String,
}

impl SignedUrlConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
            base_url: None,
            signature_param: "signature".to_string(),
            expires_param: "expires".to_string(),
        }
    }

    pub fn with_base_url(mut self, base: &str) -> Self {
        self.base_url = Some(base.trim_end_matches('/').to_string());
        self
    }
}

/// Signed URL generator and verifier.
#[derive(Clone)]
pub struct SignedUrl {
    config: SignedUrlConfig,
}

impl SignedUrl {
    /// Create a new signed URL generator.
    pub fn new(secret: &str) -> Self {
        Self {
            config: SignedUrlConfig::new(secret),
        }
    }

    /// Create from config.
    pub fn from_config(config: SignedUrlConfig) -> Self {
        Self { config }
    }

    /// Sign a URL with expiration (seconds from now).
    pub fn sign(&self, path: &str, expires_in_seconds: u64) -> String {
        let expires = Utc::now().timestamp() as u64 + expires_in_seconds;
        self.sign_with_timestamp(path, expires)
    }

    /// Sign a URL with specific expiration timestamp.
    pub fn sign_with_timestamp(&self, path: &str, expires: u64) -> String {
        let path = path.trim_start_matches('/');

        // Create the string to sign
        let to_sign = format!("{}:{}", path, expires);
        let signature = self.generate_signature(&to_sign);

        // Build URL
        let separator = if path.contains('?') { '&' } else { '?' };
        let base = match &self.config.base_url {
            Some(b) => format!("{}/{}", b, path),
            None => format!("/{}", path),
        };

        format!(
            "{}{}{}={}&{}={}",
            base,
            separator,
            self.config.expires_param,
            expires,
            self.config.signature_param,
            signature
        )
    }

    /// Sign with additional parameters.
    pub fn sign_with_params(
        &self,
        path: &str,
        expires_in_seconds: u64,
        params: &HashMap<String, String>,
    ) -> String {
        let expires = Utc::now().timestamp() as u64 + expires_in_seconds;
        let path = path.trim_start_matches('/');

        // Build params string
        let mut param_parts: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect();
        param_parts.sort();
        let params_str = param_parts.join("&");

        // Create the string to sign
        let to_sign = format!("{}:{}:{}", path, expires, params_str);
        let signature = self.generate_signature(&to_sign);

        // Build URL
        let separator = if path.contains('?') { '&' } else { '?' };
        let base = match &self.config.base_url {
            Some(b) => format!("{}/{}", b, path),
            None => format!("/{}", path),
        };

        let mut url = format!(
            "{}{}{}={}&{}={}",
            base,
            separator,
            self.config.expires_param,
            expires,
            self.config.signature_param,
            signature
        );

        if !params_str.is_empty() {
            url = format!("{}&{}", url, params_str);
        }

        url
    }

    /// Verify a signed URL.
    pub fn verify(&self, url: &str) -> bool {
        self.verify_url(url).is_ok()
    }

    /// Verify and return parsed info.
    pub fn verify_url(&self, url: &str) -> Result<SignedUrlInfo, SignedUrlError> {
        // Parse URL
        let (path, query) = match url.split_once('?') {
            Some((p, q)) => (p, q),
            None => return Err(SignedUrlError::MissingSignature),
        };

        // Parse query params
        let params: HashMap<String, String> = query
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();

        // Get expires
        let expires_str = params
            .get(&self.config.expires_param)
            .ok_or(SignedUrlError::MissingExpiration)?;
        let expires: u64 = expires_str
            .parse()
            .map_err(|_| SignedUrlError::InvalidExpiration)?;

        // Check expiration
        let now = Utc::now().timestamp() as u64;
        if now > expires {
            return Err(SignedUrlError::Expired);
        }

        // Get signature
        let signature = params
            .get(&self.config.signature_param)
            .ok_or(SignedUrlError::MissingSignature)?;

        // Get path without base URL
        let clean_path = match &self.config.base_url {
            Some(base) => path.trim_start_matches(base).trim_start_matches('/'),
            None => path.trim_start_matches('/'),
        };

        // Recreate signature
        let to_sign = format!("{}:{}", clean_path, expires);
        let expected_signature = self.generate_signature(&to_sign);

        if signature != &expected_signature {
            return Err(SignedUrlError::InvalidSignature);
        }

        Ok(SignedUrlInfo {
            path: clean_path.to_string(),
            expires: DateTime::from_timestamp(expires as i64, 0).unwrap_or_default(),
            params: params
                .into_iter()
                .filter(|(k, _)| {
                    k != &self.config.expires_param && k != &self.config.signature_param
                })
                .collect(),
        })
    }

    /// Generate HMAC signature.
    fn generate_signature(&self, data: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.config.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
}

/// Signed URL verification info.
#[derive(Debug, Clone)]
pub struct SignedUrlInfo {
    pub path: String,
    pub expires: DateTime<Utc>,
    pub params: HashMap<String, String>,
}

/// Signed URL errors.
#[derive(Debug, thiserror::Error)]
pub enum SignedUrlError {
    #[error("Missing signature")]
    MissingSignature,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Missing expiration")]
    MissingExpiration,
    #[error("Invalid expiration")]
    InvalidExpiration,
    #[error("URL has expired")]
    Expired,
}
