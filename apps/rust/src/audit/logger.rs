//! Audit logger implementation.

use chrono::{DateTime, Utc};
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;

/// Audit action types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuditAction {
    /// Entity was created.
    Create,
    /// Entity was updated.
    Update,
    /// Entity was deleted (soft or hard).
    Delete,
    /// Entity was restored from soft delete.
    Restore,
    /// Custom action.
    Custom,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::Create => write!(f, "create"),
            AuditAction::Update => write!(f, "update"),
            AuditAction::Delete => write!(f, "delete"),
            AuditAction::Restore => write!(f, "restore"),
            AuditAction::Custom => write!(f, "custom"),
        }
    }
}

/// Audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique entry ID.
    pub id: String,
    /// Action performed.
    pub action: AuditAction,
    /// Entity type (e.g., "user", "post").
    pub entity_type: String,
    /// Entity ID.
    pub entity_id: String,
    /// User who performed the action (if known).
    pub actor_id: Option<String>,
    /// Timestamp of the action.
    pub timestamp: DateTime<Utc>,
    /// Old values (for updates).
    pub old_values: Option<serde_json::Value>,
    /// New values (for creates and updates).
    pub new_values: Option<serde_json::Value>,
    /// Additional metadata.
    pub metadata: Option<serde_json::Value>,
    /// IP address of the actor (if known).
    pub ip_address: Option<String>,
    /// User agent (if known).
    pub user_agent: Option<String>,
}

/// Audit logger error types.
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Audit logger implementation.
#[derive(Clone)]
pub struct AuditLogger {
    pool: Arc<Pool>,
    prefix: String,
    /// Maximum entries to keep per entity (0 = unlimited).
    max_entries: usize,
}

impl AuditLogger {
    /// Create a new audit logger.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "audit:".to_string(),
            max_entries: 100,
        }
    }

    /// Create with custom settings.
    pub fn with_settings(pool: Arc<Pool>, prefix: &str, max_entries: usize) -> Self {
        Self {
            pool,
            prefix: format!("{}:", prefix),
            max_entries,
        }
    }

    /// Generate audit key for an entity.
    fn key(&self, entity_type: &str, entity_id: &str) -> String {
        format!("{}{}:{}", self.prefix, entity_type, entity_id)
    }

    /// Generate global audit key for an entity type.
    fn type_key(&self, entity_type: &str) -> String {
        format!("{}{}:_all", self.prefix, entity_type)
    }

    /// Log an audit entry.
    pub async fn log(
        &self,
        action: AuditAction,
        entity_type: &str,
        entity_id: &str,
        actor_id: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> Result<String, AuditError> {
        self.log_full(
            action,
            entity_type,
            entity_id,
            actor_id,
            None,
            None,
            metadata,
            None,
            None,
        )
        .await
    }

    /// Log an audit entry with old/new values.
    pub async fn log_with_changes<T: Serialize>(
        &self,
        action: AuditAction,
        entity_type: &str,
        entity_id: &str,
        actor_id: Option<&str>,
        old_value: &T,
        new_value: &T,
    ) -> Result<String, AuditError> {
        let old_json = serde_json::to_value(old_value)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;
        let new_json = serde_json::to_value(new_value)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;

        self.log_full(
            action,
            entity_type,
            entity_id,
            actor_id,
            Some(old_json),
            Some(new_json),
            None,
            None,
            None,
        )
        .await
    }

    /// Log a full audit entry with all details.
    #[allow(clippy::too_many_arguments)]
    pub async fn log_full(
        &self,
        action: AuditAction,
        entity_type: &str,
        entity_id: &str,
        actor_id: Option<&str>,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
        metadata: Option<serde_json::Value>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<String, AuditError> {
        let entry_id = uuid::Uuid::new_v4().to_string();

        let entry = AuditEntry {
            id: entry_id.clone(),
            action,
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            actor_id: actor_id.map(String::from),
            timestamp: Utc::now(),
            old_values,
            new_values,
            metadata,
            ip_address: ip_address.map(String::from),
            user_agent: user_agent.map(String::from),
        };

        let json = serde_json::to_string(&entry)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            AuditError::RedisError(e.to_string())
        })?;

        // Key for entity-specific audit log
        let entity_key = self.key(entity_type, entity_id);

        // Key for type-wide audit log
        let type_key = self.type_key(entity_type);

        // Push to entity audit list
        conn.lpush::<_, _, ()>(&entity_key, &json)
            .await
            .map_err(|e| {
                tracing::error!("Redis lpush error: {}", e);
                AuditError::RedisError(e.to_string())
            })?;

        // Push to type-wide audit list
        conn.lpush::<_, _, ()>(&type_key, &json)
            .await
            .map_err(|e| {
                tracing::error!("Redis lpush error: {}", e);
                AuditError::RedisError(e.to_string())
            })?;

        // Trim lists if max_entries is set
        if self.max_entries > 0 {
            let _: () = conn
                .ltrim(&entity_key, 0, (self.max_entries - 1) as isize)
                .await
                .unwrap_or(());
            let _: () = conn
                .ltrim(&type_key, 0, (self.max_entries * 10 - 1) as isize)
                .await
                .unwrap_or(());
        }

        tracing::debug!(
            "Audit: {} {} {} by {:?}",
            action,
            entity_type,
            entity_id,
            actor_id
        );

        Ok(entry_id)
    }

    /// Get audit history for an entity.
    pub async fn history(
        &self,
        entity_type: &str,
        entity_id: &str,
        limit: usize,
    ) -> Result<Vec<AuditEntry>, AuditError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            AuditError::RedisError(e.to_string())
        })?;

        let key = self.key(entity_type, entity_id);
        let entries: Vec<String> =
            conn.lrange(&key, 0, (limit - 1) as isize)
                .await
                .map_err(|e| {
                    tracing::error!("Redis lrange error: {}", e);
                    AuditError::RedisError(e.to_string())
                })?;

        let mut result = Vec::new();
        for json in entries {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(&json) {
                result.push(entry);
            }
        }

        Ok(result)
    }

    /// Get audit history for all entities of a type.
    pub async fn type_history(
        &self,
        entity_type: &str,
        limit: usize,
    ) -> Result<Vec<AuditEntry>, AuditError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            AuditError::RedisError(e.to_string())
        })?;

        let key = self.type_key(entity_type);
        let entries: Vec<String> =
            conn.lrange(&key, 0, (limit - 1) as isize)
                .await
                .map_err(|e| {
                    tracing::error!("Redis lrange error: {}", e);
                    AuditError::RedisError(e.to_string())
                })?;

        let mut result = Vec::new();
        for json in entries {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(&json) {
                result.push(entry);
            }
        }

        Ok(result)
    }

    /// Clear audit history for an entity.
    pub async fn clear(&self, entity_type: &str, entity_id: &str) -> Result<(), AuditError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            AuditError::RedisError(e.to_string())
        })?;

        let key = self.key(entity_type, entity_id);
        conn.del::<_, ()>(&key).await.map_err(|e| {
            tracing::error!("Redis del error: {}", e);
            AuditError::RedisError(e.to_string())
        })?;

        Ok(())
    }
}

/// Helper trait to add audit logging to operations.
#[async_trait::async_trait]
pub trait Auditable: Serialize + DeserializeOwned + Send + Sync {
    /// Entity type name for audit logs.
    const ENTITY_TYPE: &'static str;

    /// Get the entity ID.
    fn entity_id(&self) -> String;
}
