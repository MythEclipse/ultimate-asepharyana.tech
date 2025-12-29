//! Feature flags implementation.

use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

/// Feature status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FeatureStatus {
    /// Feature is enabled for everyone.
    Enabled,
    /// Feature is disabled for everyone.
    Disabled,
    /// Feature is enabled only for specific users.
    Partial,
    /// Feature is in beta (percentage rollout).
    Beta,
}

/// Feature definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    /// Feature name/key.
    pub name: String,
    /// Human-readable description.
    pub description: Option<String>,
    /// Current status.
    pub status: FeatureStatus,
    /// Percentage of users (for beta rollout, 0-100).
    pub percentage: Option<u8>,
    /// Specific user IDs enabled.
    pub enabled_users: HashSet<String>,
    /// Specific user IDs disabled.
    pub disabled_users: HashSet<String>,
}

impl Feature {
    /// Create a new feature.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            status: FeatureStatus::Disabled,
            percentage: None,
            enabled_users: HashSet::new(),
            disabled_users: HashSet::new(),
        }
    }

    /// Create an enabled feature.
    pub fn enabled(name: &str) -> Self {
        Self {
            status: FeatureStatus::Enabled,
            ..Self::new(name)
        }
    }

    /// Create a disabled feature.
    pub fn disabled(name: &str) -> Self {
        Self {
            status: FeatureStatus::Disabled,
            ..Self::new(name)
        }
    }

    /// Create a beta feature with percentage rollout.
    pub fn beta(name: &str, percentage: u8) -> Self {
        Self {
            status: FeatureStatus::Beta,
            percentage: Some(percentage.min(100)),
            ..Self::new(name)
        }
    }

    /// Add description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Enable for specific user.
    pub fn enable_for(mut self, user_id: &str) -> Self {
        self.enabled_users.insert(user_id.to_string());
        self
    }

    /// Disable for specific user.
    pub fn disable_for(mut self, user_id: &str) -> Self {
        self.disabled_users.insert(user_id.to_string());
        self
    }
}

/// Feature flags error.
#[derive(Debug, thiserror::Error)]
pub enum FeatureError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Feature not found: {0}")]
    NotFound(String),
}

/// Feature flags manager.
#[derive(Clone)]
pub struct FeatureFlags {
    pool: Arc<Pool>,
    prefix: String,
}

impl FeatureFlags {
    /// Create a new feature flags manager.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "feature:".to_string(),
        }
    }

    /// Create with custom prefix.
    pub fn with_prefix(pool: Arc<Pool>, prefix: &str) -> Self {
        Self {
            pool,
            prefix: format!("{}:", prefix),
        }
    }

    fn key(&self, name: &str) -> String {
        format!("{}{}", self.prefix, name)
    }

    /// Register a feature.
    pub async fn register(&self, feature: Feature) -> Result<(), FeatureError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            FeatureError::RedisError(e.to_string())
        })?;

        let json = serde_json::to_string(&feature)
            .map_err(|e| FeatureError::SerializationError(e.to_string()))?;

        conn.set::<_, _, ()>(&self.key(&feature.name), &json)
            .await
            .map_err(|e| {
                tracing::error!("Redis set error: {}", e);
                FeatureError::RedisError(e.to_string())
            })?;

        Ok(())
    }

    /// Get a feature.
    pub async fn get(&self, name: &str) -> Result<Option<Feature>, FeatureError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            FeatureError::RedisError(e.to_string())
        })?;

        let json: Option<String> = conn.get(&self.key(name)).await.map_err(|e| {
            tracing::error!("Redis get error: {}", e);
            FeatureError::RedisError(e.to_string())
        })?;

        match json {
            Some(j) => {
                let feature: Feature = serde_json::from_str(&j)
                    .map_err(|e| FeatureError::SerializationError(e.to_string()))?;
                Ok(Some(feature))
            }
            None => Ok(None),
        }
    }

    /// Check if a feature is globally enabled.
    pub async fn is_enabled(&self, name: &str) -> bool {
        match self.get(name).await {
            Ok(Some(feature)) => feature.status == FeatureStatus::Enabled,
            _ => false,
        }
    }

    /// Check if a feature is enabled for a specific user.
    pub async fn is_enabled_for(&self, name: &str, user_id: &str) -> bool {
        match self.get(name).await {
            Ok(Some(feature)) => {
                // Check if explicitly disabled for user
                if feature.disabled_users.contains(user_id) {
                    return false;
                }

                // Check if explicitly enabled for user
                if feature.enabled_users.contains(user_id) {
                    return true;
                }

                match feature.status {
                    FeatureStatus::Enabled => true,
                    FeatureStatus::Disabled => false,
                    FeatureStatus::Partial => false,
                    FeatureStatus::Beta => {
                        // Use user_id hash for consistent percentage rollout
                        if let Some(pct) = feature.percentage {
                            let hash = simple_hash(user_id);
                            (hash % 100) < pct as u64
                        } else {
                            false
                        }
                    }
                }
            }
            _ => false,
        }
    }

    /// Enable a feature globally.
    pub async fn enable(&self, name: &str) -> Result<(), FeatureError> {
        let mut feature = self.get(name).await?.unwrap_or_else(|| Feature::new(name));
        feature.status = FeatureStatus::Enabled;
        self.register(feature).await
    }

    /// Disable a feature globally.
    pub async fn disable(&self, name: &str) -> Result<(), FeatureError> {
        let mut feature = self.get(name).await?.unwrap_or_else(|| Feature::new(name));
        feature.status = FeatureStatus::Disabled;
        self.register(feature).await
    }

    /// Enable a feature for a specific user.
    pub async fn enable_for_user(&self, name: &str, user_id: &str) -> Result<(), FeatureError> {
        let mut feature = self.get(name).await?.unwrap_or_else(|| Feature::new(name));
        feature.enabled_users.insert(user_id.to_string());
        feature.disabled_users.remove(user_id);
        if feature.status == FeatureStatus::Disabled {
            feature.status = FeatureStatus::Partial;
        }
        self.register(feature).await
    }

    /// Disable a feature for a specific user.
    pub async fn disable_for_user(&self, name: &str, user_id: &str) -> Result<(), FeatureError> {
        let mut feature = self.get(name).await?.unwrap_or_else(|| Feature::new(name));
        feature.disabled_users.insert(user_id.to_string());
        feature.enabled_users.remove(user_id);
        self.register(feature).await
    }

    /// Set beta percentage.
    pub async fn set_beta(&self, name: &str, percentage: u8) -> Result<(), FeatureError> {
        let mut feature = self.get(name).await?.unwrap_or_else(|| Feature::new(name));
        feature.status = FeatureStatus::Beta;
        feature.percentage = Some(percentage.min(100));
        self.register(feature).await
    }

    /// Delete a feature.
    pub async fn delete(&self, name: &str) -> Result<(), FeatureError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            FeatureError::RedisError(e.to_string())
        })?;

        conn.del::<_, ()>(&self.key(name)).await.map_err(|e| {
            tracing::error!("Redis del error: {}", e);
            FeatureError::RedisError(e.to_string())
        })?;

        Ok(())
    }
}

/// Simple hash function for consistent percentage rollout.
fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for c in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(c as u64);
    }
    hash
}

/// Macro for feature flag checks.
#[macro_export]
macro_rules! feature_enabled {
    ($flags:expr, $name:expr) => {
        $flags.is_enabled($name).await
    };
    ($flags:expr, $name:expr, $user:expr) => {
        $flags.is_enabled_for($name, $user).await
    };
}
