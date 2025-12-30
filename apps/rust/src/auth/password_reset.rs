//! Password Reset Tokens.
//!
//! Generate and validate password reset tokens.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::auth::password_reset::{PasswordReset, ResetToken};
//!
//! let reset = PasswordReset::new(redis_pool);
//!
//! // Create token
//! let token = reset.create_token("user@example.com").await?;
//!
//! // Verify and get email
//! let email = reset.verify_token(&token).await?;
//! ```

use chrono::{DateTime, Duration, Utc};
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Password reset error.
#[derive(Debug, thiserror::Error)]
pub enum ResetError {
    #[error("Token expired")]
    Expired,
    #[error("Token not found")]
    NotFound,
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Too many requests")]
    RateLimited,
}

/// Reset token data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetToken {
    pub token: String,
    pub email: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Password reset manager.
#[derive(Clone)]
pub struct PasswordReset {
    pool: Arc<Pool>,
    prefix: String,
    expiry_hours: i64,
}

impl PasswordReset {
    /// Create a new password reset manager.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "password_reset:".to_string(),
            expiry_hours: 1,
        }
    }

    /// Set custom expiry.
    pub fn with_expiry(mut self, hours: i64) -> Self {
        self.expiry_hours = hours;
        self
    }

    fn key(&self, token: &str) -> String {
        format!("{}{}", self.prefix, token)
    }

    fn rate_key(&self, email: &str) -> String {
        format!("{}rate:{}", self.prefix, email)
    }

    /// Generate a secure token.
    fn generate_token() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        hex::encode(bytes)
    }

    /// Create a password reset token.
    pub async fn create_token(&self, email: &str) -> Result<String, ResetError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ResetError::RedisError(e.to_string()))?;

        // Rate limiting: max 3 per hour
        let rate_key = self.rate_key(email);
        let count: i64 = conn.incr(&rate_key, 1).await.unwrap_or(1);
        if count == 1 {
            let _: () = conn.expire(&rate_key, 3600).await.unwrap_or(());
        }
        if count > 3 {
            return Err(ResetError::RateLimited);
        }

        let token = Self::generate_token();
        let data = ResetToken {
            token: token.clone(),
            email: email.to_string(),
            expires_at: Utc::now() + Duration::hours(self.expiry_hours),
            created_at: Utc::now(),
        };

        let json =
            serde_json::to_string(&data).map_err(|e| ResetError::RedisError(e.to_string()))?;
        let key = self.key(&token);
        let expiry_secs = self.expiry_hours * 3600;

        conn.set_ex::<_, _, ()>(&key, &json, expiry_secs as u64)
            .await
            .map_err(|e| ResetError::RedisError(e.to_string()))?;

        Ok(token)
    }

    /// Verify a token and return the email.
    pub async fn verify_token(&self, token: &str) -> Result<String, ResetError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ResetError::RedisError(e.to_string()))?;

        let key = self.key(token);
        let json: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| ResetError::RedisError(e.to_string()))?;

        match json {
            Some(j) => {
                let data: ResetToken =
                    serde_json::from_str(&j).map_err(|e| ResetError::RedisError(e.to_string()))?;
                if Utc::now() > data.expires_at {
                    return Err(ResetError::Expired);
                }
                Ok(data.email)
            }
            None => Err(ResetError::NotFound),
        }
    }

    /// Invalidate a token after use.
    pub async fn invalidate(&self, token: &str) -> Result<(), ResetError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| ResetError::RedisError(e.to_string()))?;
        conn.del::<_, ()>(&self.key(token))
            .await
            .map_err(|e| ResetError::RedisError(e.to_string()))?;
        Ok(())
    }

    /// Hash a password.
    pub fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hex::encode(hasher.finalize())
    }
}
