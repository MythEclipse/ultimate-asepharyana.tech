// Standard library imports
use std::sync::Arc;

// External crate imports
use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, parse_html, Cache};
use axum::{
    extract::{Path, State},
    Router,
};

use serde_json::json;
use tracing::info;

// Internal imports
use crate::routes::AppState;

// Import shared models and parsers
use crate::models::anime2::{CompleteAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;


const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime2/complete-anime/{slug}",
    tag = "anime2",
    operation_id = "anime2_complete_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/complete-anime/slug endpoint.", body = ApiResponse<Vec<CompleteAnimeItem>>),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> ApiResult<Vec<CompleteAnimeItem>> {
    let _start_time = std::time::Instant::now();
    info!("Handling request for complete_anime slug: {}", slug);

    let cache_key = format!("anime2:complete:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let url = format!(
                "https://alqanime.si/anime/page/{}/?status=completed&order=update",
                slug
            );

            let html = fetch_html_with_retry(&url)
                .await
                .map_err(|e| e.to_string())?;

            let slug_clone = slug.clone();
            let (anime_list, pagination) =
                tokio::task::spawn_blocking(move || parse_anime_page(&html, &slug_clone))
                    .await
                    .map_err(|e: tokio::task::JoinError| e.to_string())?
                    .map_err(|e: String| e.to_string())?;

            // Store posters in a separate vector to avoid borrow checker issues
            let posters: Vec<String> = anime_list.iter().map(|i| i.poster.clone()).collect();
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            let mut final_data = anime_list;
            for (i, item) in final_data.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            let meta = json!({
                "pagination": pagination,
                "status": "Ok"
            });

            Ok(ApiResponse::success_with_meta(final_data, meta))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

fn parse_anime_page(
    html: &str,
    slug: &str,
) -> Result<(Vec<CompleteAnimeItem>, Pagination), String> {
    let start_time = std::time::Instant::now();
    info!("Starting to parse anime page for slug: {}", slug);

    let document = parse_html(html);
    
    // Parse anime items using shared parser
    let anime_list = parsers::parse_complete_anime(html)
        .map_err(|e| format!("Failed to parse anime items: {}", e))?;

    // Parse pagination using shared parser
    let current_page = slug.parse::<u32>().unwrap_or(1);
    let pagination = parsers::parse_pagination(&document, current_page);

    let duration = start_time.elapsed();
    info!("Parsed {} anime items in {:?}", anime_list.len(), duration);

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}