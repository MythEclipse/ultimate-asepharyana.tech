//! Cache tags for tag-based cache invalidation.
//!
//! Extends the basic cache helper with tag support for grouped invalidation.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::cache_tags::TaggedCache;
//!
//! let cache = TaggedCache::new(redis_pool);
//!
//! // Set with tags
//! cache.put_tagged("user:123", data, &["users", "user:123"], 3600).await?;
//! cache.put_tagged("user:456", data2, &["users", "user:456"], 3600).await?;
//!
//! // Invalidate all users
//! cache.flush_tag("users").await?;
//! ```

use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// Tagged cache error types.
#[derive(Debug, thiserror::Error)]
pub enum CacheTagError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

/// Cache with tag-based invalidation support.
#[derive(Clone)]
pub struct TaggedCache {
    pool: Arc<Pool>,
    prefix: String,
    tag_prefix: String,
}

impl TaggedCache {
    /// Create a new tagged cache.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "cache:".to_string(),
            tag_prefix: "cache:tag:".to_string(),
        }
    }

    /// Create with custom prefix.
    pub fn with_prefix(pool: Arc<Pool>, prefix: &str) -> Self {
        Self {
            pool,
            prefix: format!("{}:", prefix),
            tag_prefix: format!("{}:tag:", prefix),
        }
    }

    /// Generate cache key.
    fn key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }

    /// Generate tag key.
    fn tag_key(&self, tag: &str) -> String {
        format!("{}{}", self.tag_prefix, tag)
    }

    /// Get a value from cache.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let cache_key = self.key(key);
        let value: Option<String> = conn.get(&cache_key).await.map_err(|e| {
            tracing::error!("Redis get error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        match value {
            Some(json) => {
                let data: T = serde_json::from_str(&json)
                    .map_err(|e| CacheTagError::DeserializationError(e.to_string()))?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    /// Put a value in cache with TTL (seconds).
    pub async fn put<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: u64,
    ) -> Result<(), CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let cache_key = self.key(key);
        let json = serde_json::to_string(value)
            .map_err(|e| CacheTagError::SerializationError(e.to_string()))?;

        conn.set_ex::<_, _, ()>(&cache_key, &json, ttl)
            .await
            .map_err(|e| {
                tracing::error!("Redis set error: {}", e);
                CacheTagError::RedisError(e.to_string())
            })?;

        Ok(())
    }

    /// Put a value in cache with tags.
    pub async fn put_tagged<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        tags: &[&str],
        ttl: u64,
    ) -> Result<(), CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let cache_key = self.key(key);
        let json = serde_json::to_string(value)
            .map_err(|e| CacheTagError::SerializationError(e.to_string()))?;

        // Set the value
        conn.set_ex::<_, _, ()>(&cache_key, &json, ttl)
            .await
            .map_err(|e| {
                tracing::error!("Redis set error: {}", e);
                CacheTagError::RedisError(e.to_string())
            })?;

        // Add key to each tag set
        for tag in tags {
            let tag_key = self.tag_key(tag);
            conn.sadd::<_, _, ()>(&tag_key, &cache_key)
                .await
                .map_err(|e| {
                    tracing::error!("Redis sadd error: {}", e);
                    CacheTagError::RedisError(e.to_string())
                })?;
        }

        Ok(())
    }

    /// Delete a specific key.
    pub async fn forget(&self, key: &str) -> Result<(), CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let cache_key = self.key(key);
        conn.del::<_, ()>(&cache_key).await.map_err(|e| {
            tracing::error!("Redis del error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        Ok(())
    }

    /// Flush all keys associated with a tag.
    pub async fn flush_tag(&self, tag: &str) -> Result<usize, CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let tag_key = self.tag_key(tag);

        // Get all keys in the tag set
        let keys: Vec<String> = conn.smembers(&tag_key).await.map_err(|e| {
            tracing::error!("Redis smembers error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let count = keys.len();

        // Delete each key
        for key in &keys {
            let _: () = conn.del(key).await.unwrap_or(());
        }

        // Delete the tag set itself
        conn.del::<_, ()>(&tag_key).await.map_err(|e| {
            tracing::error!("Redis del error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        tracing::info!("Flushed {} keys for tag '{}'", count, tag);
        Ok(count)
    }

    /// Flush multiple tags at once.
    pub async fn flush_tags(&self, tags: &[&str]) -> Result<usize, CacheTagError> {
        let mut total = 0;
        for tag in tags {
            total += self.flush_tag(tag).await?;
        }
        Ok(total)
    }

    /// Check if a key exists.
    pub async fn has(&self, key: &str) -> Result<bool, CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let cache_key = self.key(key);
        let exists: bool = conn.exists(&cache_key).await.map_err(|e| {
            tracing::error!("Redis exists error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        Ok(exists)
    }

    /// Get or set a value (cache-aside pattern).
    pub async fn remember<T, F, Fut>(&self, key: &str, ttl: u64, f: F) -> Result<T, CacheTagError>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, CacheTagError>>,
    {
        // Try to get from cache
        if let Some(value) = self.get::<T>(key).await? {
            return Ok(value);
        }

        // Generate value
        let value = f().await?;

        // Store in cache
        self.put(key, &value, ttl).await?;

        Ok(value)
    }

    /// Get or set a value with tags.
    pub async fn remember_tagged<T, F, Fut>(
        &self,
        key: &str,
        tags: &[&str],
        ttl: u64,
        f: F,
    ) -> Result<T, CacheTagError>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, CacheTagError>>,
    {
        // Try to get from cache
        if let Some(value) = self.get::<T>(key).await? {
            return Ok(value);
        }

        // Generate value
        let value = f().await?;

        // Store in cache with tags
        self.put_tagged(key, &value, tags, ttl).await?;

        Ok(value)
    }

    /// Get cache statistics for a tag.
    pub async fn tag_count(&self, tag: &str) -> Result<usize, CacheTagError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        let tag_key = self.tag_key(tag);
        let count: usize = conn.scard(&tag_key).await.map_err(|e| {
            tracing::error!("Redis scard error: {}", e);
            CacheTagError::RedisError(e.to_string())
        })?;

        Ok(count)
    }
}

/// Helper to create cache tags from entity type and ID.
pub fn entity_tags(entity_type: &str, id: &str) -> Vec<String> {
    vec![entity_type.to_string(), format!("{}:{}", entity_type, id)]
}

/// Helper to create cache key for entity.
pub fn entity_key(entity_type: &str, id: &str) -> String {
    format!("{}:{}", entity_type, id)
}
