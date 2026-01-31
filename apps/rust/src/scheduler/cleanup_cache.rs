//! Scheduled task for cleaning up old cached data.

use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use tracing::{info, warn};

use crate::entities::image_cache;
use crate::helpers::cache::Cache;
use crate::infra::redis::REDIS_POOL;

use super::ScheduledTask;

/// Cleanup old cache data to prevent disk/memory bloat.
/// Runs daily at 2 AM to clean:
/// - Old image cache entries (>30 days)
/// - Orphaned cache keys in Redis
/// - Expired data without TTL
pub struct CleanupOldCache {
    db: Arc<DatabaseConnection>,
}

impl CleanupOldCache {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ScheduledTask for CleanupOldCache {
    fn name(&self) -> &'static str {
        "cleanup_old_cache"
    }

    fn schedule(&self) -> &'static str {
        // Daily at 2 AM
        "0 0 2 * * *"
    }

    async fn run(&self) {
        info!("🧹 Starting old cache cleanup...");

        let mut total_cleaned = 0;

        // 1. Clean old image cache (>30 days)
        match self.cleanup_old_images(30).await {
            Ok(count) => {
                info!("✓ Cleaned {} old image cache entries", count);
                total_cleaned += count;
            }
            Err(e) => {
                warn!("Failed to clean old image cache: {}", e);
            }
        }

        // 2. Clean orphaned Redis keys
        match self.cleanup_orphaned_redis_keys().await {
            Ok(count) => {
                info!("✓ Cleaned {} orphaned Redis keys", count);
                total_cleaned += count;
            }
            Err(e) => {
                warn!("Failed to clean orphaned Redis keys: {}", e);
            }
        }

        // 3. Compact Redis memory
        match self.compact_redis_memory().await {
            Ok(()) => {
                info!("✓ Redis memory compacted");
            }
            Err(e) => {
                warn!("Failed to compact Redis memory: {}", e);
            }
        }

        info!("🎉 Cache cleanup complete: {} items cleaned", total_cleaned);
    }
}

impl CleanupOldCache {
    /// Remove image cache entries older than specified days.
    async fn cleanup_old_images(&self, days: i64) -> Result<usize, String> {
        use chrono::{Duration, Utc};

        let cutoff = Utc::now() - Duration::days(days);

        // Find old entries
        let old_images = image_cache::Entity::find()
            .filter(image_cache::Column::CreatedAt.lt(cutoff))
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        let count = old_images.len();

        if count == 0 {
            return Ok(0);
        }

        // Delete from database
        let ids: Vec<String> = old_images.iter().map(|img| img.id.clone()).collect();

        image_cache::Entity::delete_many()
            .filter(image_cache::Column::Id.is_in(ids))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        // Also clean from Redis
        let redis_pool = REDIS_POOL.clone();
        let cache = Cache::new(&redis_pool);
        for img in old_images {
            let cache_key = format!("img_cache:{}", Self::hash_url(&img.original_url));
            let _ = cache.delete(&cache_key).await;
        }

        Ok(count)
    }

    /// Clean orphaned Redis keys (keys without TTL that shouldn't exist).
    async fn cleanup_orphaned_redis_keys(&self) -> Result<usize, String> {
        use deadpool_redis::redis::AsyncCommands;

        let mut conn = REDIS_POOL
            .get()
            .await
            .map_err(|e| format!("Failed to get Redis connection: {}", e))?;

        let mut cleaned = 0;

        // Find keys without TTL (should not exist)
        let patterns = vec!["anime:*", "komik:*", "user:*:profile", "img_cache:*"];

        for pattern in patterns {
            let keys: Vec<String> = conn
                .keys(pattern)
                .await
                .map_err(|e| format!("Failed to get keys: {}", e))?;

            for key in keys {
                let ttl: i64 = conn.ttl(&key).await.unwrap_or(-1);

                // TTL = -1 means no expiration (orphaned)
                // TTL = -2 means key doesn't exist
                if ttl == -1 {
                    // Set a default TTL of 7 days for orphaned keys
                    let _: () = conn.expire(&key, 604800).await.unwrap_or(());
                    cleaned += 1;
                }
            }
        }

        Ok(cleaned)
    }

    /// Compact Redis memory to free up fragmented space.
    async fn compact_redis_memory(&self) -> Result<(), String> {
        let mut conn = REDIS_POOL
            .get()
            .await
            .map_err(|e| format!("Failed to get Redis connection: {}", e))?;

        // Run MEMORY PURGE command - using cmd method on connection
        let _: String = deadpool_redis::redis::cmd("MEMORY")
            .arg("PURGE")
            .query_async(&mut *conn)
            .await
            .map_err(|e| format!("Failed to purge memory: {}", e))?;

        Ok(())
    }

    /// Simple hash function for URL (same as in image_cache.rs).
    /// Simple hash function for URL (same as in image_cache.rs).
    fn hash_url(url: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule() {
        use sea_orm::{DatabaseBackend, MockDatabase};
        let db = MockDatabase::new(DatabaseBackend::Sqlite).into_connection();
        let task = CleanupOldCache { db: Arc::new(db) };
        assert_eq!(task.schedule(), "0 0 2 * * *");
    }

    #[test]
    fn test_hash_url() {
        let hash = CleanupOldCache::hash_url("https://example.com/image.jpg");
        assert_eq!(hash.len(), 64); // SHA256 hash length
    }
}
