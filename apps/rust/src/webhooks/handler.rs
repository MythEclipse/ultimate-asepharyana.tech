//! Webhook handler implementation.

use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;

/// Webhook error.
#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Missing signature header")]
    MissingSignature,
    #[error("Expired timestamp")]
    ExpiredTimestamp,
}

/// Webhook event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// Event ID.
    pub id: String,
    /// Provider name (e.g., "stripe", "github").
    pub provider: String,
    /// Event type (e.g., "payment.succeeded", "push").
    pub event_type: String,
    /// Raw payload.
    pub payload: serde_json::Value,
    /// Received timestamp.
    pub received_at: DateTime<Utc>,
}

impl WebhookEvent {
    /// Create a new webhook event.
    pub fn new(provider: &str, event_type: &str, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            provider: provider.to_string(),
            event_type: event_type.to_string(),
            payload,
            received_at: Utc::now(),
        }
    }

    /// Get a field from payload.
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.payload
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

/// Signature verification trait.
pub trait SignatureVerifier: Send + Sync {
    /// Verify the webhook signature.
    fn verify(&self, payload: &[u8], signature: &str, secret: &str) -> bool;

    /// Extract event type from payload.
    fn extract_event_type(&self, payload: &serde_json::Value) -> Option<String>;
}

/// Default HMAC-SHA256 verifier.
pub struct HmacSha256Verifier {
    /// Header prefix (e.g., "sha256=" for GitHub).
    pub prefix: String,
}

impl Default for HmacSha256Verifier {
    fn default() -> Self {
        Self {
            prefix: String::new(),
        }
    }
}

impl HmacSha256Verifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl SignatureVerifier for HmacSha256Verifier {
    fn verify(&self, payload: &[u8], signature: &str, secret: &str) -> bool {
        let sig = if !self.prefix.is_empty() && signature.starts_with(&self.prefix) {
            &signature[self.prefix.len()..]
        } else {
            signature
        };

        let sig_bytes = match hex::decode(sig) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };

        mac.update(payload);

        mac.verify_slice(&sig_bytes).is_ok()
    }

    fn extract_event_type(&self, payload: &serde_json::Value) -> Option<String> {
        // Common patterns
        payload
            .get("type")
            .or_else(|| payload.get("event"))
            .or_else(|| payload.get("event_type"))
            .or_else(|| payload.get("action"))
            .and_then(|v| v.as_str())
            .map(String::from)
    }
}

/// Stripe-specific verifier.
pub struct StripeVerifier;

impl SignatureVerifier for StripeVerifier {
    fn verify(&self, payload: &[u8], signature: &str, secret: &str) -> bool {
        // Stripe uses "t=timestamp,v1=signature" format
        let parts: HashMap<&str, &str> = signature
            .split(',')
            .filter_map(|part| {
                let mut kv = part.splitn(2, '=');
                Some((kv.next()?, kv.next()?))
            })
            .collect();

        let timestamp = match parts.get("t") {
            Some(t) => *t,
            None => return false,
        };

        let sig = match parts.get("v1") {
            Some(s) => *s,
            None => return false,
        };

        // Create signed payload
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));

        let sig_bytes = match hex::decode(sig) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };

        mac.update(signed_payload.as_bytes());

        mac.verify_slice(&sig_bytes).is_ok()
    }

    fn extract_event_type(&self, payload: &serde_json::Value) -> Option<String> {
        payload
            .get("type")
            .and_then(|v| v.as_str())
            .map(String::from)
    }
}

/// GitHub-specific verifier.
pub struct GitHubVerifier;

impl SignatureVerifier for GitHubVerifier {
    fn verify(&self, payload: &[u8], signature: &str, secret: &str) -> bool {
        HmacSha256Verifier::with_prefix("sha256=").verify(payload, signature, secret)
    }

    fn extract_event_type(&self, payload: &serde_json::Value) -> Option<String> {
        payload
            .get("action")
            .and_then(|v| v.as_str())
            .map(String::from)
    }
}

/// Webhook handler.
pub struct WebhookHandler {
    secrets: HashMap<String, String>,
    verifiers: HashMap<String, Arc<dyn SignatureVerifier>>,
}

impl Default for WebhookHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl WebhookHandler {
    /// Create a new webhook handler.
    pub fn new() -> Self {
        let mut verifiers: HashMap<String, Arc<dyn SignatureVerifier>> = HashMap::new();
        verifiers.insert("stripe".to_string(), Arc::new(StripeVerifier));
        verifiers.insert("github".to_string(), Arc::new(GitHubVerifier));
        verifiers.insert(
            "default".to_string(),
            Arc::new(HmacSha256Verifier::default()),
        );

        Self {
            secrets: HashMap::new(),
            verifiers,
        }
    }

    /// Add a secret for a provider.
    pub fn add_secret(mut self, provider: &str, secret: &str) -> Self {
        self.secrets
            .insert(provider.to_string(), secret.to_string());
        self
    }

    /// Add a custom verifier.
    pub fn add_verifier(mut self, provider: &str, verifier: Arc<dyn SignatureVerifier>) -> Self {
        self.verifiers.insert(provider.to_string(), verifier);
        self
    }

    /// Verify and parse a webhook.
    pub fn verify_and_parse(
        &self,
        provider: &str,
        payload: &[u8],
        signature: &str,
    ) -> Result<WebhookEvent, WebhookError> {
        let secret = self
            .secrets
            .get(provider)
            .ok_or_else(|| WebhookError::UnknownProvider(provider.to_string()))?;

        let verifier = self
            .verifiers
            .get(provider)
            .or_else(|| self.verifiers.get("default"))
            .ok_or_else(|| WebhookError::UnknownProvider(provider.to_string()))?;

        if !verifier.verify(payload, signature, secret) {
            return Err(WebhookError::InvalidSignature);
        }

        let payload_json: serde_json::Value =
            serde_json::from_slice(payload).map_err(|e| WebhookError::ParseError(e.to_string()))?;

        let event_type = verifier
            .extract_event_type(&payload_json)
            .unwrap_or_else(|| "unknown".to_string());

        Ok(WebhookEvent::new(provider, &event_type, payload_json))
    }

    /// Parse without verification (for testing).
    pub fn parse_unverified(
        &self,
        provider: &str,
        payload: &[u8],
    ) -> Result<WebhookEvent, WebhookError> {
        let payload_json: serde_json::Value =
            serde_json::from_slice(payload).map_err(|e| WebhookError::ParseError(e.to_string()))?;

        let verifier = self
            .verifiers
            .get(provider)
            .or_else(|| self.verifiers.get("default"));

        let event_type = verifier
            .and_then(|v| v.extract_event_type(&payload_json))
            .unwrap_or_else(|| "unknown".to_string());

        Ok(WebhookEvent::new(provider, &event_type, payload_json))
    }
}
