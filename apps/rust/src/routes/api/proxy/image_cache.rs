//! Image cache API endpoint
//!
//! POST /api/proxy/image-cache - Cache an image and return CDN URL

use axum::{extract::State, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::services::images::cache::ImageCache;
use crate::routes::AppState;
use crate::events::bus::ImageRepaired;

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
    let cache = ImageCache::new(state.db.clone(), state.redis_pool.clone())
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
            let cache = ImageCache::new(db, redis).with_semaphore(semaphore);
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
#[utoipa::path(
    post,
    path = "/api/proxy/image-cache/batch",
    tag = "proxy",
    operation_id = "proxy_image_cache_batch",
    request_body = ImageCacheBatchRequest,
    responses(
        (status = 200, description = "Batch image caching successful", body = ImageCacheBatchResponse)
    )
)]
pub async fn image_cache_batch(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ImageCacheBatchRequest>,
) -> impl IntoResponse {
    let cache = ImageCache::new(state.db.clone(), state.redis_pool.clone());
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

/// Request body for deleting image cache
#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteImageCacheRequest {
    /// Original image URL to delete from cache
    pub url: String,
}

/// Response for deleting image cache
#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteImageCacheResponse {
    pub success: bool,
    pub original_url: String,
    pub message: String,
}

/// Delete an image from cache
#[utoipa::path(
    delete,
    path = "/api/proxy/image-cache",
    request_body = DeleteImageCacheRequest,
    responses(
        (status = 200, description = "Image cache deleted successfully", body = DeleteImageCacheResponse)
    ),
    tag = "proxy"
)]
pub async fn delete_image_cache(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DeleteImageCacheRequest>,
) -> impl IntoResponse {
    let cache = ImageCache::new(state.db.clone(), state.redis_pool.clone());
    
    match cache.invalidate(&req.url).await {
        Ok(_) => Json(DeleteImageCacheResponse {
            success: true,
            original_url: req.url,
            message: "Cache invalidated successfully".to_string(),
        }),
        Err(e) => {
            tracing::error!("Failed to invalidate cache: {}", e);
            Json(DeleteImageCacheResponse {
                success: false,
                original_url: req.url,
                message: format!("Failed to invalidate cache: {}", e),
            })
        }
    }
}

/// Request body for auditing image cache
#[derive(Debug, Deserialize, ToSchema)]
pub struct AuditImageCacheRequest {
    /// Original image URL to audit
    pub url: String,
}

/// Response for auditing image cache
#[derive(Debug, Serialize, ToSchema)]
pub struct AuditImageCacheResponse {
    pub success: bool,
    pub original_url: String,
    pub cdn_url: Option<String>,
    pub was_accessible: bool,
    pub re_uploaded: bool,
    pub message: String,
}

/// Audit an image cache entry, re-uploading if the CDN URL is inaccessible
#[utoipa::path(
    post,
    path = "/api/proxy/image-cache/audit",
    request_body = AuditImageCacheRequest,
    responses(
        (status = 200, description = "Image cache audited successfully", body = AuditImageCacheResponse)
    ),
    tag = "proxy"
)]
pub async fn audit_image_cache(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AuditImageCacheRequest>,
) -> impl IntoResponse {
    let cache = ImageCache::new(state.db.clone(), state.redis_pool.clone())
        .with_semaphore(state.image_processing_semaphore.clone());

    let cdn_url_opt = cache.get_cdn_url(&req.url).await;
    
    if let Some(cdn_url) = &cdn_url_opt {
        // Check if accessible
        let client = crate::infra::http_client::http_client().client();
        let is_accessible = match client.head(cdn_url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        };

        if is_accessible {
            return Json(AuditImageCacheResponse {
                success: true,
                original_url: req.url,
                cdn_url: Some(cdn_url.clone()),
                was_accessible: true,
                re_uploaded: false,
                message: "CDN URL is accessible".to_string(),
            });
        }

        // Not accessible, invalidate and re-upload
        tracing::info!("CDN URL {} for {} is inaccessible, re-uploading...", cdn_url, req.url);
        let _ = cache.invalidate(&req.url).await;
        
        match cache.get_or_cache(&req.url).await {
            Ok(new_cdn_url) => {
                // Publish event for real-time refresh
                state.event_bus.publish(ImageRepaired {
                    original_url: req.url.clone(),
                    cdn_url: new_cdn_url.clone(),
                }).await;

                Json(AuditImageCacheResponse {
                    success: true,
                    original_url: req.url,
                    cdn_url: Some(new_cdn_url),
                    was_accessible: false,
                    re_uploaded: true,
                    message: "CDN URL was inaccessible, successfully re-uploaded".to_string(),
                })
            },
            Err(e) => Json(AuditImageCacheResponse {
                success: false,
                original_url: req.url,
                cdn_url: None,
                was_accessible: false,
                re_uploaded: false,
                message: format!("CDN URL was inaccessible, and re-upload failed: {}", e),
            }),
        }
    } else {
        // Not cached yet, cache it now
        match cache.get_or_cache(&req.url).await {
            Ok(new_cdn_url) => {
                // Publish event for real-time refresh
                state.event_bus.publish(ImageRepaired {
                    original_url: req.url.clone(),
                    cdn_url: new_cdn_url.clone(),
                }).await;

                Json(AuditImageCacheResponse {
                    success: true,
                    original_url: req.url,
                    cdn_url: Some(new_cdn_url),
                    was_accessible: false,
                    re_uploaded: true,
                    message: "Not cached previously, successfully uploaded".to_string(),
                })
            },
            Err(e) => Json(AuditImageCacheResponse {
                success: false,
                original_url: req.url,
                cdn_url: None,
                was_accessible: false,
                re_uploaded: false,
                message: format!("Not cached previously, and upload failed: {}", e),
            }),
        }
    }
}

/// Register routes for this endpoint
pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route("/api/proxy/image-cache", axum::routing::post(image_cache))
        .route("/api/proxy/image-cache", axum::routing::delete(delete_image_cache))
        .route("/api/proxy/image-cache/batch", axum::routing::post(image_cache_batch))
        .route("/api/proxy/image-cache/audit", axum::routing::post(audit_image_cache))
}