use crate::models::anime2::HasPoster;
use crate::routes::AppState;
use std::sync::Arc;

/// Cache poster URLs for a collection of anime items
/// This is a fire-and-forget operation that triggers lazy background caching
pub async fn cache_posters<T: HasPoster>(
    app_state: &Arc<AppState>,
    items: &[T],
) {
    let posters: Vec<String> = items.iter().map(|item| item.poster().to_string()).collect();
    
    if posters.is_empty() {
        return;
    }

    let db = app_state.db.clone();
    let redis = app_state.redis_pool.clone();
    
    crate::helpers::image_cache::cache_image_urls_batch_lazy(
        db,
        &redis,
        posters,
        Some(app_state.image_processing_semaphore.clone()),
    )
    .await;
}

/// Cache poster URLs and update items with cached URLs
/// Returns the updated items with CDN URLs
pub async fn cache_and_update_posters<T: HasPoster + Clone>(
    app_state: &Arc<AppState>,
    mut items: Vec<T>,
) -> Vec<T> {
    let posters: Vec<String> = items.iter().map(|item| item.poster().to_string()).collect();
    
    if posters.is_empty() {
        return items;
    }

    let db = app_state.db.clone();
    let redis = app_state.redis_pool.clone();
    
    let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
        db,
        &redis,
        posters,
        Some(app_state.image_processing_semaphore.clone()),
    )
    .await;

    // Update items with cached URLs
    for (i, item) in items.iter_mut().enumerate() {
        if let Some(url) = cached_posters.get(i) {
            item.set_poster(url.clone());
        }
    }

    items
}

/// Cache multiple collections of posters and update them
/// Useful when you have different types of items (e.g., ongoing and complete anime)
pub async fn cache_multiple_collections(
    app_state: &Arc<AppState>,
    collections: Vec<Vec<String>>,
) -> Vec<String> {
    let all_posters: Vec<String> = collections.into_iter().flatten().collect();
    
    if all_posters.is_empty() {
        return Vec::new();
    }

    let db = app_state.db.clone();
    let redis = app_state.redis_pool.clone();
    
    crate::helpers::image_cache::cache_image_urls_batch_lazy(
        db,
        &redis,
        all_posters,
        Some(app_state.image_processing_semaphore.clone()),
    )
    .await
}
