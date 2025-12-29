//! Redis-backed session store implementation.

use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Configuration for the session store.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Session cookie name
    pub cookie_name: String,
    /// Session TTL (time to live)
    pub ttl: Duration,
    /// Cookie path
    pub path: String,
    /// Cookie domain (None = current domain)
    pub domain: Option<String>,
    /// Secure cookie (HTTPS only)
    pub secure: bool,
    /// HttpOnly cookie
    pub http_only: bool,
    /// SameSite policy
    pub same_site: SameSite,
}

/// SameSite cookie attribute
#[derive(Debug, Clone, Copy)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: "session_id".to_string(),
            ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
            path: "/".to_string(),
            domain: None,
            secure: true,
            http_only: true,
            same_site: SameSite::Lax,
        }
    }
}

/// Session data structure stored in Redis.
#[derive(Debug, Clone, Default, Serialize, serde::Deserialize)]
pub struct SessionData {
    /// Session ID
    pub id: String,
    /// Key-value data
    pub data: HashMap<String, serde_json::Value>,
    /// Flash messages (cleared after read)
    pub flash: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: i64,
    /// Last accessed timestamp
    pub accessed_at: i64,
}

impl SessionData {
    /// Create a new session with generated ID.
    pub fn new() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            data: HashMap::new(),
            flash: HashMap::new(),
            created_at: now,
            accessed_at: now,
        }
    }

    /// Get a value from session data.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Set a value in session data.
    pub fn set<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(json) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), json);
        }
    }

    /// Remove a value from session data.
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    /// Check if a key exists.
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Set a flash message.
    pub fn set_flash(&mut self, key: &str, message: &str) {
        self.flash.insert(key.to_string(), message.to_string());
    }

    /// Get and consume a flash message.
    pub fn get_flash(&mut self, key: &str) -> Option<String> {
        self.flash.remove(key)
    }

    /// Get all flash messages and clear them.
    pub fn consume_flash(&mut self) -> HashMap<String, String> {
        std::mem::take(&mut self.flash)
    }

    /// Update the accessed timestamp.
    pub fn touch(&mut self) {
        self.accessed_at = chrono::Utc::now().timestamp();
    }
}

/// Redis-backed session store.
#[derive(Clone)]
pub struct SessionStore {
    pool: Arc<Pool>,
    config: SessionConfig,
    prefix: String,
}

impl SessionStore {
    /// Create a new session store with the given Redis pool.
    pub fn new(pool: Arc<Pool>, config: SessionConfig) -> Self {
        Self {
            pool,
            config,
            prefix: "session:".to_string(),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults(pool: Arc<Pool>) -> Self {
        Self::new(pool, SessionConfig::default())
    }

    /// Get the session configuration.
    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    /// Generate a Redis key for the session.
    fn key(&self, session_id: &str) -> String {
        format!("{}{}", self.prefix, session_id)
    }

    /// Create a new session.
    pub async fn create(&self) -> Result<SessionData, SessionError> {
        let session = SessionData::new();
        self.save(&session).await?;
        Ok(session)
    }

    /// Load a session by ID.
    pub async fn load(&self, session_id: &str) -> Result<Option<SessionData>, SessionError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        let key = self.key(session_id);
        let data: Option<String> = conn.get(&key).await.map_err(|e| {
            tracing::error!("Failed to load session: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        match data {
            Some(json) => {
                let session: SessionData = serde_json::from_str(&json)
                    .map_err(|e| SessionError::DeserializationError(e.to_string()))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    /// Save a session to Redis.
    pub async fn save(&self, session: &SessionData) -> Result<(), SessionError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        let key = self.key(&session.id);
        let json = serde_json::to_string(session)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;

        let ttl_secs = self.config.ttl.as_secs() as i64;
        conn.set_ex::<_, _, ()>(&key, &json, ttl_secs as u64)
            .await
            .map_err(|e| {
                tracing::error!("Failed to save session: {}", e);
                SessionError::RedisError(e.to_string())
            })?;

        Ok(())
    }

    /// Delete a session.
    pub async fn destroy(&self, session_id: &str) -> Result<(), SessionError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        let key = self.key(session_id);
        conn.del::<_, ()>(&key).await.map_err(|e| {
            tracing::error!("Failed to destroy session: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        Ok(())
    }

    /// Regenerate session ID (for security after login).
    pub async fn regenerate(&self, old_session: &mut SessionData) -> Result<String, SessionError> {
        let old_id = old_session.id.clone();

        // Generate new ID
        old_session.id = Uuid::new_v4().to_string();
        old_session.touch();

        // Save with new ID
        self.save(old_session).await?;

        // Delete old session
        self.destroy(&old_id).await?;

        Ok(old_session.id.clone())
    }

    /// Extend session TTL.
    pub async fn touch(&self, session_id: &str) -> Result<(), SessionError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        let key = self.key(session_id);
        let ttl_secs = self.config.ttl.as_secs() as i64;
        conn.expire::<_, ()>(&key, ttl_secs).await.map_err(|e| {
            tracing::error!("Failed to touch session: {}", e);
            SessionError::RedisError(e.to_string())
        })?;

        Ok(())
    }
}

/// Errors that can occur during session operations.
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
