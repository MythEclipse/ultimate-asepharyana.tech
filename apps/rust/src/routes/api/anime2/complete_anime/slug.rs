// Standard library imports
use std::sync::Arc;

// External crate imports
use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;

// Internal imports
use crate::routes::AppState;

// Import shared models and parsers
use crate::models::anime2::{CompleteAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/complete-anime/{slug}";
pub const ENDPOINT_DESCRIPTION: &str =
    "Handles GET requests for the anime2/complete-anime/slug endpoint.";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_complete_anime_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<CompleteAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompleteAnimeResponse {
    pub status: String,
    pub data: Vec<CompleteAnimeItem>,
    pub pagination: Pagination,
}

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
        (status = 200, description = "Handles GET requests for the anime2/complete-anime/slug endpoint.", body = CompleteAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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

            crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // Return original data explicitly for speed
            Ok(CompleteAnimeResponse {
                status: "Ok".to_string(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
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
    router.route(ENDPOINT_PATH, get(slug))
}