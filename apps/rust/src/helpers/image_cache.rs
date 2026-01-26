//! Image caching helper using Picser CDN (picser.pages.dev).
//!
//! This module provides utilities to cache images via jsDelivr CDN
//! with database storage for URL mapping.

use crate::entities::image_cache;
use chrono::Utc;
use deadpool_redis::Pool as RedisPool;
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use super::cache::Cache;

use crate::helpers::cache_ttl::CACHE_TTL_IMAGE;

/// Default TTL for image cache in Redis (24 hours)
pub const IMAGE_CACHE_TTL: u64 = CACHE_TTL_IMAGE;

/// Redis key prefix for image cache
pub const IMAGE_CACHE_PREFIX: &str = "img_cache";

/// Redis key prefix for caching locks (to prevent duplicate uploads)
pub const IMAGE_CACHE_LOCK_PREFIX: &str = "img_cache_lock";

/// Lock TTL (60 seconds - enough time for upload to complete)
pub const IMAGE_CACHE_LOCK_TTL: u64 = 60;

/// Picser API endpoint (web upload - no token required)
pub const PICSER_API_URL: &str = "https://picser.pages.dev/api/upload";

/// Create a hash of the URL for cache key
pub fn url_hash(url: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..8]) // Use first 8 bytes for shorter key
}

/// Response from Picser API (/api/upload)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PicserResponse {
    pub success: bool,
    pub url: Option<String>,
    pub urls: Option<PicserUrls>,
    pub filename: Option<String>,
    pub size: Option<u64>,
    #[serde(rename = "type")]
    pub content_type: Option<String>,
    pub commit_sha: Option<String>,
    pub github_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PicserUrls {
    pub github: Option<String>,
    pub raw: Option<String>,
    pub jsdelivr: Option<String>,
    pub jsdelivr_commit: Option<String>,
}

/// Configuration for image cache
#[derive(Debug, Clone)]
pub struct ImageCacheConfig {
    /// GitHub token for Picser API (optional - uses public upload if not set)
    pub github_token: Option<String>,
    /// GitHub owner for uploads
    pub github_owner: String,
    /// GitHub repo for uploads
    pub github_repo: String,
    /// GitHub branch
    pub github_branch: String,
    /// Upload folder
    pub folder: String,
}

impl Default for ImageCacheConfig {
    fn default() -> Self {
        Self {
            github_token: None,
            github_owner: "sh20raj".to_string(),
            github_repo: "picser".to_string(),
            github_branch: "main".to_string(),
            folder: "uploads".to_string(),
        }
    }
}

/// Image cache service
pub struct ImageCache {
    db: Arc<DatabaseConnection>,
    redis: RedisPool,
    client: Client,
    _config: ImageCacheConfig,
    semaphore: Option<std::sync::Arc<tokio::sync::Semaphore>>,
}

// Add imports for Request Coalescing
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::broadcast;

// Global In-Flight Uploads Map
// Maps Original URL -> Broadcast Sender
static IN_FLIGHT_UPLOADS: Lazy<DashMap<String, broadcast::Sender<Result<String, String>>>> =
    Lazy::new(DashMap::new);

impl ImageCache {
    /// Create a new image cache instance
    pub fn new(db: Arc<DatabaseConnection>, redis: RedisPool) -> Self {
        Self {
            db,
            redis,
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
            _config: ImageCacheConfig::default(),
            semaphore: None,
        }
    }

    /// Create with custom configuration
    pub fn with_config(
        db: Arc<DatabaseConnection>,
        redis: RedisPool,
        config: ImageCacheConfig,
    ) -> Self {
        Self {
            db,
            redis,
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
            _config: config,
            semaphore: None,
        }
    }

    /// Set concurrency limiter
    pub fn with_semaphore(mut self, semaphore: std::sync::Arc<tokio::sync::Semaphore>) -> Self {
        self.semaphore = Some(semaphore);
        self
    }

    /// Get CDN URL for an image, caching if needed
    pub async fn get_or_cache(&self, original_url: &str) -> Result<String, String> {
        let cache_key = format!("{}:{}", IMAGE_CACHE_PREFIX, url_hash(original_url));
        let lock_key = format!("{}:{}", IMAGE_CACHE_LOCK_PREFIX, url_hash(original_url));

        // 1. Check Redis cache first
        let redis_cache = Cache::new(&self.redis);
        if let Some(cached_url) = redis_cache.get::<String>(&cache_key).await {
            info!("ImageCache: Redis hit for {}", original_url);
            return Ok(cached_url);
        }

        // 2. Check Request Coalescing (SingleFlight)
        // This handles concurrent requests in this process/instance
        let (tx, is_leader) = {
            use dashmap::mapref::entry::Entry;
            match IN_FLIGHT_UPLOADS.entry(original_url.to_string()) {
                Entry::Occupied(entry) => {
                    info!("ImageCache: Joining in-flight upload for {}", original_url);
                    (entry.get().clone(), false)
                }
                Entry::Vacant(entry) => {
                    let (tx, _) = broadcast::channel(1);
                    entry.insert(tx.clone());
                    info!("ImageCache: Starting leader upload for {}", original_url);
                    (tx, true)
                }
            }
        };

        if !is_leader {
            // Follower: Wait for result
            let mut rx = tx.subscribe();
            return match rx.recv().await {
                Ok(Ok(url)) => Ok(url),
                Ok(Err(e)) => Err(e),
                Err(e) => {
                    warn!(
                        "ImageCache: Coalesce receive error for {}: {:?}",
                        original_url, e
                    );
                    Err("Upload coalescing failed".to_string())
                }
            };
        }

        // Leader: Perform the work
        // We wrap the work in a closure/block to easily capture the result
        let result = async {
            // 3. Check database (Double check inside leader to be sure)
            if let Some(db_entry) = self.find_in_db(original_url).await? {
                // Store in Redis for faster access
                let _ = redis_cache
                    .set_with_ttl(&cache_key, &db_entry.cdn_url, IMAGE_CACHE_TTL)
                    .await;
                info!("ImageCache: DB hit for {}", original_url);
                return Ok(db_entry.cdn_url);
            }

            // 4. Check if another process is already caching this URL (Distributed Lock check)
            if redis_cache.get::<bool>(&lock_key).await.is_some() {
                // Even if locked by another process, strict single-flight within this instance
                // is good. But if another process is working, we might want to wait or just return error?
                // Current logic returns error.
                info!(
                    "ImageCache: Already being cached by another process: {}",
                    original_url
                );
                return Err(format!("URL {} is already being cached", original_url));
            }

            // 5. Acquire lock in Redis
            let _ = redis_cache
                .set_with_ttl(&lock_key, &true, IMAGE_CACHE_LOCK_TTL)
                .await;

            // 6. Upload
            info!("ImageCache: Miss - uploading {} to Picser", original_url);

            // Acquire permit if semaphore is set
            let redis_clone = self.redis.clone();
            let lock_key_for_error = lock_key.clone();
            let _permit = if let Some(sem) = &self.semaphore {
                Some(sem.acquire().await.map_err(|e| {
                    let redis = redis_clone.clone();
                    let lock = lock_key_for_error.clone();
                    tokio::spawn(async move {
                        let cache = Cache::new(&redis);
                        let _ = cache.delete(&lock).await;
                    });
                    e.to_string()
                })?)
            } else {
                None
            };

            // Work
            let work_result = async {
                // Upload to Picser
                let cdn_url = self.upload_to_picser(original_url).await?;

                // Save to database
                self.save_to_db(original_url, &cdn_url).await?;

                // Cache in Redis
                let _ = redis_cache
                    .set_with_ttl(&cache_key, &cdn_url, IMAGE_CACHE_TTL)
                    .await;

                // Invalidate API caches
                let _ = self.invalidate_api_caches().await;

                Ok(cdn_url)
            }
            .await;

            // Release Redis lock
            let _ = redis_cache.delete(&lock_key).await;

            work_result
        }
        .await;

        // Broadcast result
        let _ = tx.send(result.clone());

        // Remove from map
        IN_FLIGHT_UPLOADS.remove(original_url);

        result
    }

    /// Get CDN URL without uploading (read-only lookup)
    pub async fn get_cdn_url(&self, original_url: &str) -> Option<String> {
        let cache_key = format!("{}:{}", IMAGE_CACHE_PREFIX, url_hash(original_url));

        // Check Redis first
        let redis_cache = Cache::new(&self.redis);
        if let Some(cached_url) = redis_cache.get::<String>(&cache_key).await {
            return Some(cached_url);
        }

        // Check database
        if let Ok(Some(entry)) = self.find_in_db(original_url).await {
            return Some(entry.cdn_url);
        }

        None
    }

    /// Invalidate cache for a URL
    pub async fn invalidate(&self, original_url: &str) -> Result<(), String> {
        let cache_key = format!("{}:{}", IMAGE_CACHE_PREFIX, url_hash(original_url));

        // Remove from Redis
        let redis_cache = Cache::new(&self.redis);
        let _ = redis_cache.delete(&cache_key).await;

        // Remove from database
        image_cache::Entity::delete_many()
            .filter(image_cache::Column::OriginalUrl.eq(original_url))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        info!("ImageCache: Invalidated {}", original_url);
        Ok(())
    }

    /// Find entry in database
    async fn find_in_db(&self, original_url: &str) -> Result<Option<image_cache::Model>, String> {
        image_cache::Entity::find()
            .filter(image_cache::Column::OriginalUrl.eq(original_url))
            .one(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())
    }

    /// Save URL mapping to database
    async fn save_to_db(&self, original_url: &str, cdn_url: &str) -> Result<(), String> {
        let model = image_cache::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            original_url: Set(original_url.to_string()),
            cdn_url: Set(cdn_url.to_string()),
            created_at: Set(Utc::now()),
            expires_at: Set(None),
        };

        model
            .insert(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Invalidate API response caches that may contain this image
    async fn invalidate_api_caches(&self) -> Result<(), String> {
        use deadpool_redis::redis::AsyncCommands;

        let mut conn = self.redis.get().await.map_err(|e| e.to_string())?;

        // Patterns untuk cache API yang berisi images
        let patterns = vec!["anime:*", "anime2:*", "komik:*"];

        let mut total_deleted = 0;

        for pattern in patterns {
            let keys: Vec<String> = conn.keys(pattern).await.map_err(|e| e.to_string())?;

            if !keys.is_empty() {
                let deleted: usize = conn.del(&keys).await.map_err(|e| e.to_string())?;
                total_deleted += deleted;
            }
        }

        if total_deleted > 0 {
            info!(
                "ImageCache: Invalidated {} API cache keys after image upload",
                total_deleted
            );
        }

        Ok(())
    }

    /// Upload image to Picser CDN
    async fn upload_to_picser(&self, original_url: &str) -> Result<String, String> {
        // Download the image first
        let image_bytes = self
            .client
            .get(original_url)
            .send()
            .await
            .map_err(|e| format!("Failed to download image: {}", e))?
            .bytes()
            .await
            .map_err(|e| format!("Failed to read image bytes: {}", e))?;

        // Determine filename from URL
        let filename = self.extract_filename(original_url);

        // Create multipart form - /api/upload only needs the file
        let part = reqwest::multipart::Part::bytes(image_bytes.to_vec())
            .file_name(filename.clone())
            .mime_str("image/jpeg")
            .map_err(|e| e.to_string())?;

        let form = reqwest::multipart::Form::new().part("file", part);

        // Upload to Picser
        let response = self
            .client
            .post(PICSER_API_URL)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Failed to upload to Picser: {}", e))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read Picser response: {}", e))?;

        // Log the raw response for debugging
        info!("ImageCache: Picser raw response: {}", response_text);

        let picser_response: PicserResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                format!(
                    "Failed to parse Picser response: {} - Raw: {}",
                    e, response_text
                )
            })?;

        if !picser_response.success {
            let err_msg = picser_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            error!(
                "ImageCache: Picser upload failed for {}: {}",
                original_url, err_msg
            );
            return Err(format!("Picser upload failed: {}", err_msg));
        }

        // Extract CDN URL - prefer jsdelivr_commit for permanence
        let cdn_url = picser_response
            .urls
            .as_ref()
            .and_then(|u| u.jsdelivr_commit.clone().or(u.jsdelivr.clone()))
            .or(picser_response.url.clone())
            .or(picser_response.github_url.clone())
            .ok_or_else(|| {
                error!(
                    "ImageCache: No CDN URL in Picser response for {}",
                    original_url
                );
                "No CDN URL in Picser response".to_string()
            })?;

        info!("ImageCache: Uploaded {} -> {}", original_url, cdn_url);
        Ok(cdn_url)
    }

    /// Extract filename from URL
    fn extract_filename(&self, url: &str) -> String {
        url.split('/')
            .last()
            .and_then(|s| s.split('?').next())
            .filter(|s| !s.is_empty() && s.contains('.'))
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}.jpg", url_hash(url)))
    }
}

/// Convenience function to create a CDN URL for an image
/// Returns the original URL if caching fails (graceful fallback)
pub async fn cache_image_url(
    db: Arc<DatabaseConnection>,
    redis: &RedisPool,
    original_url: &str,
) -> String {
    let cache = ImageCache::new(db, redis.clone());
    match cache.get_or_cache(original_url).await {
        Ok(cdn_url) => cdn_url,
        Err(e) => {
            warn!("ImageCache: Failed to cache {}: {}", original_url, e);
            original_url.to_string()
        }
    }
}

/// Batch cache multiple images
pub async fn cache_image_urls(
    db: Arc<DatabaseConnection>,
    redis: &RedisPool,
    urls: &[String],
) -> Vec<String> {
    let cache = ImageCache::new(db, redis.clone());
    let mut results = Vec::with_capacity(urls.len());

    for url in urls {
        let cdn_url = match cache.get_or_cache(url).await {
            Ok(u) => u,
            Err(_) => url.clone(),
        };
        results.push(cdn_url);
    }

    results
}

/// Helper to convert image URL to CDN URL in background (non-blocking)
/// Returns original URL immediately and caches in background
pub fn cache_image_url_lazy(
    db: Arc<DatabaseConnection>,
    redis: &RedisPool,
    original_url: String,
) -> String {
    // Optimized: No need to clone DB/Redis here if we change the signature to take Arc directly
    // but for now we clone only what's needed for the closure
    let db_owned = db.clone();
    let redis_owned = redis.clone();

    // Move string ownership into closure without extra clone if possible
    let url_for_cache = original_url.clone();
    let url_for_log = original_url.clone();

    // Spawn background task to cache
    tokio::spawn(async move {
        let cache = ImageCache::new(db_owned, redis_owned);
        match cache.get_or_cache(&url_for_cache).await {
            Ok(cdn_url) => {
                info!("[LazyImageCache] Cached: {} -> {}", url_for_cache, cdn_url);
            }
            Err(e) => {
                warn!("[LazyImageCache] Failed to cache {}: {}", url_for_log, e);
            }
        }
    });

    original_url
}

/// Convert image URL to CDN URL if already cached, otherwise return original
/// and trigger background caching for next request (with duplicate prevention)
pub async fn get_cached_or_original(
    db: Arc<DatabaseConnection>,
    redis: &RedisPool,
    original_url: &str,
) -> String {
    let cache = ImageCache::new(db.clone(), redis.clone());

    // Check if already cached (Redis or DB)
    if let Some(cdn_url) = cache.get_cdn_url(original_url).await {
        return cdn_url;
    }

    // Check if currently being cached by another process
    let lock_key = format!("{}:{}", IMAGE_CACHE_LOCK_PREFIX, url_hash(original_url));
    let redis_cache = Cache::new(redis);
    if redis_cache.get::<bool>(&lock_key).await.is_some() {
        // Already being cached, just return original URL
        info!(
            "[ImageCache] URL already being cached by another process: {}",
            original_url
        );
        return original_url.to_string();
    }

    // Not cached and not being cached - start background caching
    let db_owned = db.clone();
    let redis_owned = redis.clone();
    let url = original_url.to_string();
    tokio::spawn(async move {
        let cache = ImageCache::new(db_owned, redis_owned);
        match cache.get_or_cache(&url).await {
            Ok(cdn_url) => {
                info!("[BgCache] Successfully cached: {} -> {}", url, cdn_url);
            }
            Err(e) => {
                // Only warn if it's not already being cached
                if !e.contains("already being cached") {
                    warn!("[BgCache] Failed to cache {}: {}", url, e);
                }
            }
        }
    });

    original_url.to_string()
}

/// Batch process multiple image URLs - returns original URLs immediately
/// and triggers background caching for all
pub fn cache_image_urls_batch_lazy(
    db: Arc<DatabaseConnection>,
    redis: &RedisPool,
    urls: Vec<String>,
) -> Vec<String> {
    if urls.is_empty() {
        return urls;
    }

    let db_owned = db.clone();
    let redis_owned = redis.clone();

    // Avoid cloning the entire vector just for the background task if we return the original
    // But since we need to return the original vector, we must clone it once.
    let urls_clone = urls.clone();

    // Spawn background task to cache all URLs
    tokio::spawn(async move {
        let cache = ImageCache::new(db_owned, redis_owned);
        // Process sequentially to be nice to the semaphore/Picser API
        for url in urls_clone {
            if let Ok(cdn_url) = cache.get_or_cache(&url).await {
                debug!("[BatchLazyCache] Cached: {} -> {}", url, cdn_url);
            }
        }
    });

    urls
}
