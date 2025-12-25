//! Image cache API endpoint
//!
//! POST /api/proxy/image-cache - Cache an image and return CDN URL

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::helpers::ImageCache;
use crate::routes::AppState;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/proxy/image-cache";
pub const ENDPOINT_DESCRIPTION: &str = "Cache an image to CDN and return the cached URL";
pub const ENDPOINT_TAG: &str = "proxy";
pub const OPERATION_ID: &str = "proxy_image_cache";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ImageCacheResponse>";

/// Request body for image caching
#[derive(Debug, Deserialize, ToSchema)]
pub struct ImageCacheRequest {
    /// Original image URL to cache
    pub url: String,
    /// If true, returns original URL immediately and caches in background
    #[serde(default)]
    pub lazy: bool,
}

/// Response containing the cached URL
#[derive(Debug, Serialize, ToSchema)]
pub struct ImageCacheResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Original image URL
    pub original_url: String,
    /// CDN URL (either cached or original as fallback)
    pub cdn_url: String,
    /// Whether the image was already cached
    pub from_cache: bool,
    /// If true, the image caching is pending in the background. Only present in lazy mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
}

/// Batch request for multiple images
#[derive(Debug, Deserialize, ToSchema)]
pub struct ImageCacheBatchRequest {
    /// List of image URLs to cache
    pub urls: Vec<String>,
}

/// Batch response with multiple cached URLs
#[derive(Debug, Serialize, ToSchema)]
pub struct ImageCacheBatchResponse {
    pub success: bool,
    pub results: Vec<ImageCacheResult>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImageCacheResult {
    pub original_url: String,
    pub cdn_url: String,
    pub from_cache: bool,
}

/// Cache a single image
#[utoipa::path(
    post,
    path = "/api/proxy/image-cache",
    request_body = ImageCacheRequest,
    responses(
        (status = 200, description = "Image cached successfully", body = ImageCacheResponse)
    ),
    tag = "proxy"
)]
pub async fn image_cache(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ImageCacheRequest>,
) -> impl IntoResponse {
    let cache = ImageCache::new(&state.db, &state.redis_pool)
        .with_semaphore(state.image_processing_semaphore.clone());

    // Check if already cached
    if let Some(cdn_url) = cache.get_cdn_url(&req.url).await {
        return Json(ImageCacheResponse {
            success: true,
            original_url: req.url,
            cdn_url,
            from_cache: true,
            pending: None,
        });
    }

    // Lazy mode: return original URL immediately, upload in background
    if req.lazy {
        let url = req.url.clone();
        let db = state.db.clone();
        let redis = state.redis_pool.clone();
        let semaphore = state.image_processing_semaphore.clone();

        tokio::spawn(async move {
            let cache = ImageCache::new(&db, &redis).with_semaphore(semaphore);
            match cache.get_or_cache(&url).await {
                Ok(cdn_url) => {
                    tracing::info!("[LazyCache] Successfully cached: {} -> {}", url, cdn_url)
                }
                Err(e) => tracing::warn!("[LazyCache] Background upload failed for {}: {}", url, e),
            }
        });

        return Json(ImageCacheResponse {
            success: true,
            original_url: req.url.clone(),
            cdn_url: req.url,
            from_cache: false,
            pending: Some(true),
        });
    }

    // Blocking mode: wait for upload
    match cache.get_or_cache(&req.url).await {
        Ok(cdn_url) => Json(ImageCacheResponse {
            success: true,
            original_url: req.url,
            cdn_url,
            from_cache: false,
            pending: None,
        }),
        Err(e) => {
            tracing::error!("ImageCache error: {}", e);
            Json(ImageCacheResponse {
                success: false,
                original_url: req.url.clone(),
                cdn_url: req.url,
                from_cache: false,
                pending: None,
            })
        }
    }
}

/// Cache multiple images in batch
pub async fn image_cache_batch(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ImageCacheBatchRequest>,
) -> impl IntoResponse {
    let cache = ImageCache::new(&state.db, &state.redis_pool);
    let mut results = Vec::with_capacity(req.urls.len());

    for url in req.urls {
        let from_cache = cache.get_cdn_url(&url).await.is_some();
        let cdn_url = match cache.get_or_cache(&url).await {
            Ok(u) => u,
            Err(_) => url.clone(),
        };
        results.push(ImageCacheResult {
            original_url: url,
            cdn_url,
            from_cache,
        });
    }

    Json(ImageCacheBatchResponse {
        success: true,
        results,
    })
}

/// Register routes for this endpoint
pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route(ENDPOINT_PATH, post(image_cache))
        .route("/api/proxy/image-cache/batch", post(image_cache_batch))
}
