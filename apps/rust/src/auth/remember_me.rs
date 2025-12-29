//! Remember Me Tokens for long-term authentication.
//!
//! # Example
//!
//! ```ignore
//! use rust::auth::remember_me::{RememberMe, RememberToken};
//!
//! let remember = RememberMe::new(redis_pool);
//!
//! // Create token (30 days)
//! let token = remember.create("user_id_123").await?;
//!
//! // Validate and get user
//! let user_id = remember.validate(&token).await?;
//! ```

use chrono::{DateTime, Duration, Utc};
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Remember me error.
#[derive(Debug, thiserror::Error)]
pub enum RememberError {
    #[error("Token expired")]
    Expired,
    #[error("Token not found")]
    NotFound,
    #[error("Redis error: {0}")]
    RedisError(String),
}

/// Remember token data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RememberToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

/// Remember me manager.
#[derive(Clone)]
pub struct RememberMe {
    pool: Arc<Pool>,
    prefix: String,
    expiry_days: i64,
}

impl RememberMe {
    /// Create a new remember me manager.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "remember:".to_string(),
            expiry_days: 30,
        }
    }

    /// Set custom expiry.
    pub fn with_expiry(mut self, days: i64) -> Self {
        self.expiry_days = days;
        self
    }

    fn token_key(&self, token: &str) -> String {
        format!("{}{}", self.prefix, token)
    }

    fn user_key(&self, user_id: &str) -> String {
        format!("{}user:{}", self.prefix, user_id)
    }

    /// Generate a secure token.
    fn generate_token() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let bytes: Vec<u8> = (0..48).map(|_| rng.random()).collect();
        hex::encode(bytes)
    }

    /// Create a remember me token.
    pub async fn create(&self, user_id: &str) -> Result<String, RememberError> {
        self.create_with_meta(user_id, None, None).await
    }

    /// Create with metadata.
    pub async fn create_with_meta(
        &self,
        user_id: &str,
        user_agent: Option<&str>,
        ip_address: Option<&str>,
    ) -> Result<String, RememberError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;

        let token = Self::generate_token();
        let data = RememberToken {
            token: token.clone(),
            user_id: user_id.to_string(),
            expires_at: Utc::now() + Duration::days(self.expiry_days),
            created_at: Utc::now(),
            user_agent: user_agent.map(String::from),
            ip_address: ip_address.map(String::from),
        };

        let json =
            serde_json::to_string(&data).map_err(|e| RememberError::RedisError(e.to_string()))?;
        let key = self.token_key(&token);
        let expiry_secs = self.expiry_days * 86400;

        conn.set_ex::<_, _, ()>(&key, &json, expiry_secs as u64)
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;

        // Track tokens per user
        let user_key = self.user_key(user_id);
        let _: () = conn.sadd(&user_key, &token).await.unwrap_or(());
        let _: () = conn
            .expire(&user_key, expiry_secs as i64)
            .await
            .unwrap_or(());

        Ok(token)
    }

    /// Validate a token and return user ID.
    pub async fn validate(&self, token: &str) -> Result<String, RememberError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;

        let key = self.token_key(token);
        let json: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;

        match json {
            Some(j) => {
                let data: RememberToken = serde_json::from_str(&j)
                    .map_err(|e| RememberError::RedisError(e.to_string()))?;
                if Utc::now() > data.expires_at {
                    return Err(RememberError::Expired);
                }
                Ok(data.user_id)
            }
            None => Err(RememberError::NotFound),
        }
    }

    /// Revoke a single token.
    pub async fn revoke(&self, token: &str) -> Result<(), RememberError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;
        conn.del::<_, ()>(&self.token_key(token))
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;
        Ok(())
    }

    /// Revoke all tokens for a user.
    pub async fn revoke_all(&self, user_id: &str) -> Result<(), RememberError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;

        let user_key = self.user_key(user_id);
        let tokens: Vec<String> = conn.smembers(&user_key).await.unwrap_or_default();

        for token in tokens {
            let _: () = conn.del(&self.token_key(&token)).await.unwrap_or(());
        }
        let _: () = conn.del(&user_key).await.unwrap_or(());

        Ok(())
    }

    /// Get all active sessions for a user.
    pub async fn get_sessions(&self, user_id: &str) -> Result<Vec<RememberToken>, RememberError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| RememberError::RedisError(e.to_string()))?;

        let user_key = self.user_key(user_id);
        let tokens: Vec<String> = conn.smembers(&user_key).await.unwrap_or_default();

        let mut sessions = Vec::new();
        for token in tokens {
            if let Ok(Some(json)) = conn.get::<_, Option<String>>(&self.token_key(&token)).await {
                if let Ok(data) = serde_json::from_str::<RememberToken>(&json) {
                    sessions.push(data);
                }
            }
        }

        Ok(sessions)
    }
}
