//! API Key Authentication.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::auth::api_key::{ApiKeyManager, ApiKey};
//!
//! let manager = ApiKeyManager::new(redis_pool);
//!
//! // Create API key
//! let key = manager.create("my-app", "user_id").await?;
//!
//! // Validate
//! let api_key = manager.validate(&key.key).await?;
//! ```

use chrono::{DateTime, Utc};
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

/// API key error.
#[derive(Debug, thiserror::Error)]
pub enum ApiKeyError {
    #[error("Invalid API key")]
    Invalid,
    #[error("API key revoked")]
    Revoked,
    #[error("API key expired")]
    Expired,
    #[error("Redis error: {0}")]
    RedisError(String),
}

/// API key data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// The API key (prefix + secret).
    pub key: String,
    /// Key prefix (for identification).
    pub prefix: String,
    /// Human-readable name.
    pub name: String,
    /// Owner user ID.
    pub user_id: String,
    /// Scopes/permissions.
    pub scopes: HashSet<String>,
    /// Created at.
    pub created_at: DateTime<Utc>,
    /// Expires at (optional).
    pub expires_at: Option<DateTime<Utc>>,
    /// Last used at.
    pub last_used_at: Option<DateTime<Utc>>,
    /// Is revoked.
    pub revoked: bool,
}

impl ApiKey {
    /// Check if key has a scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.contains(scope) || self.scopes.contains("*")
    }

    /// Check if key has all scopes.
    pub fn has_all_scopes(&self, scopes: &[&str]) -> bool {
        scopes.iter().all(|s| self.has_scope(s))
    }
}

/// API key manager.
#[derive(Clone)]
pub struct ApiKeyManager {
    pool: Arc<Pool>,
    prefix: String,
}

impl ApiKeyManager {
    /// Create a new API key manager.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "api_key:".to_string(),
        }
    }

    fn key_storage(&self, key_prefix: &str) -> String {
        format!("{}{}", self.prefix, key_prefix)
    }

    fn user_keys(&self, user_id: &str) -> String {
        format!("{}user:{}", self.prefix, user_id)
    }

    /// Generate an API key.
    fn generate_key() -> (String, String) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Prefix: 8 chars
        let prefix_bytes: Vec<u8> = (0..4).map(|_| rng.gen()).collect();
        let prefix = format!("sk_{}", hex::encode(&prefix_bytes));

        // Secret: 32 chars
        let secret_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let secret = hex::encode(&secret_bytes);

        let full_key = format!("{}_{}", prefix, secret);
        (prefix, full_key)
    }

    /// Create an API key.
    pub async fn create(&self, name: &str, user_id: &str) -> Result<ApiKey, ApiKeyError> {
        self.create_with_scopes(name, user_id, &["*"]).await
    }

    /// Create with specific scopes.
    pub async fn create_with_scopes(
        &self,
        name: &str,
        user_id: &str,
        scopes: &[&str],
    ) -> Result<ApiKey, ApiKeyError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        let (prefix, key) = Self::generate_key();
        let api_key = ApiKey {
            key: key.clone(),
            prefix: prefix.clone(),
            name: name.to_string(),
            user_id: user_id.to_string(),
            scopes: scopes.iter().map(|s| s.to_string()).collect(),
            created_at: Utc::now(),
            expires_at: None,
            last_used_at: None,
            revoked: false,
        };

        let json =
            serde_json::to_string(&api_key).map_err(|e| ApiKeyError::RedisError(e.to_string()))?;
        conn.set::<_, _, ()>(&self.key_storage(&prefix), &json)
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        // Track keys per user
        let _: () = conn
            .sadd(&self.user_keys(user_id), &prefix)
            .await
            .unwrap_or(());

        Ok(api_key)
    }

    /// Validate an API key.
    pub async fn validate(&self, key: &str) -> Result<ApiKey, ApiKeyError> {
        // Extract prefix from key
        let parts: Vec<&str> = key.split('_').collect();
        if parts.len() < 3 {
            return Err(ApiKeyError::Invalid);
        }
        let prefix = format!("{}_{}", parts[0], parts[1]);

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        let json: Option<String> = conn
            .get(&self.key_storage(&prefix))
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        match json {
            Some(j) => {
                let mut api_key: ApiKey =
                    serde_json::from_str(&j).map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

                // Check if key matches
                if api_key.key != key {
                    return Err(ApiKeyError::Invalid);
                }

                // Check if revoked
                if api_key.revoked {
                    return Err(ApiKeyError::Revoked);
                }

                // Check expiry
                if let Some(expires) = api_key.expires_at {
                    if Utc::now() > expires {
                        return Err(ApiKeyError::Expired);
                    }
                }

                // Update last used
                api_key.last_used_at = Some(Utc::now());
                let updated_json = serde_json::to_string(&api_key)
                    .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;
                let _: () = conn
                    .set(&self.key_storage(&prefix), &updated_json)
                    .await
                    .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

                Ok(api_key)
            }
            None => Err(ApiKeyError::Invalid),
        }
    }

    /// Revoke an API key.
    pub async fn revoke(&self, prefix: &str) -> Result<(), ApiKeyError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        let key = self.key_storage(prefix);
        let json: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        if let Some(j) = json {
            let mut api_key: ApiKey =
                serde_json::from_str(&j).map_err(|e| ApiKeyError::RedisError(e.to_string()))?;
            api_key.revoked = true;
            let updated_json = serde_json::to_string(&api_key)
                .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;
            conn.set::<_, _, ()>(&key, &updated_json)
                .await
                .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;
        }

        Ok(())
    }

    /// Get all keys for a user.
    pub async fn get_user_keys(&self, user_id: &str) -> Result<Vec<ApiKey>, ApiKeyError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ApiKeyError::RedisError(e.to_string()))?;

        let prefixes: Vec<String> = conn
            .smembers(&self.user_keys(user_id))
            .await
            .unwrap_or_default();

        let mut keys = Vec::new();
        for prefix in prefixes {
            if let Ok(Some(json)) = conn
                .get::<_, Option<String>>(&self.key_storage(&prefix))
                .await
            {
                if let Ok(mut key) = serde_json::from_str::<ApiKey>(&json) {
                    // Hide the actual key
                    key.key = format!("{}...", &key.key[..12]);
                    keys.push(key);
                }
            }
        }

        Ok(keys)
    }
}
