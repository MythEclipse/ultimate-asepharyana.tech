use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, parse_html, Cache};
use crate::routes::AppState;
use axum::extract::State;
use axum::{extract::Path, Router};

use serde_json::json;
use std::sync::Arc;
use tracing::info;

// Import shared models and parsers
use crate::models::anime2::{OngoingAnimeItemWithScore, Pagination};
use crate::scraping::anime2 as parsers;


const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime2/ongoing-anime/{slug}",
    tag = "anime2",
    operation_id = "anime2_ongoing_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/ongoing-anime/{slug} endpoint.", body = ApiResponse<Vec<OngoingAnimeItemWithScore>>),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> ApiResult<Vec<OngoingAnimeItemWithScore>> {
    let _start_time = std::time::Instant::now();
    info!("Handling request for ongoing_anime slug: {}", slug);

    let cache_key = format!("anime2:ongoing:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (data, pagination) = fetch_ongoing_anime_page(slug.clone())
                .await
                .map_err(|e| e)?;

            // Convert all poster URLs to CDN URLs concurrently
            let posters: Vec<String> = data.iter().map(|i| i.poster.clone()).collect();
            crate::helpers::image_cache::cache_image_urls_batch_lazy(
                app_state.db.clone(),
                &app_state.redis_pool,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            Ok(ApiResponse::success_with_meta(
                data,
                json!({ "pagination": pagination, "status": "Ok" }),
            ))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_ongoing_anime_page(
    slug: String,
) -> Result<(Vec<OngoingAnimeItemWithScore>, Pagination), String> {
    let url = format!(
        "https://alqanime.si/anime/page/{}/?status=ongoing&type=&order=update",
        slug
    );

    let html = fetch_html_with_retry(&url)
        .await
        .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;
    let slug_clone = slug.clone();

    match tokio::task::spawn_blocking(move || {
        parse_ongoing_anime_document(&html, &slug_clone)
    })
    .await
    {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(format!("Failed to spawn blocking task: {}", join_err)),
    }
}

fn parse_ongoing_anime_document(
    html: &str,
    slug: &str,
) -> Result<(Vec<OngoingAnimeItemWithScore>, Pagination), String> {
    let start_time = std::time::Instant::now();
    info!(
        "Starting to parse ongoing anime document for slug: {}",
        slug
    );
    
    let document = parse_html(html);
    
    // Parse anime items using shared parser
    let anime_list = parsers::parse_ongoing_anime_with_score(html)
        .map_err(|e| format!("Failed to parse anime items: {}", e))?;

    // Parse pagination using shared parser
    let current_page = slug.parse::<u32>().unwrap_or(1);
    let pagination = parsers::parse_pagination(&document, current_page);

    let duration = start_time.elapsed();
    info!(
        "Parsed {} ongoing anime items in {:?}",
        anime_list.len(),
        duration
    );

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}