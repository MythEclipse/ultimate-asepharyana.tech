use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, parse_html, Cache};
use crate::routes::AppState;
use axum::extract::State;
use axum::{extract::Query, Router};

use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

// Import shared models and parsers
use crate::models::anime2::{PaginationWithStringPages, SearchAnimeItem};
use crate::scraping::anime2 as parsers;
use crate::helpers::anime2_cache as cache_utils;


const CACHE_TTL: u64 = 300; // 5 minutes

#[derive(Deserialize, ToSchema)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[utoipa::path(
    get,
    params(
        ("q" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value")
    ),
    path = "/api/anime2/search",
    tag = "anime2",
    operation_id = "anime2_search",
    responses(
        (status = 200, description = "Searches for anime2 based on query parameters.", body = ApiResponse<Vec<SearchAnimeItem>>),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> ApiResult<Vec<SearchAnimeItem>> {
    let query = params.q.unwrap_or_else(|| "one".to_string());
    info!("Starting search for query: {}", query);

    let cache_key = format!("anime2:search:{}", query);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let url = format!("https://alqanime.si/?s={}", urlencoding::encode(&query));
            let (data, pagination) = fetch_and_parse_search(&url)
                .await
                .map_err(|e| e.to_string())?;

            // Trigger lazy batch caching using shared utility
            cache_utils::cache_posters(&app_state, &data).await;

            // We return original data for speed on cold start

            Ok(ApiResponse::success_with_meta(
                data,
                json!({ "pagination": pagination, "status": "Ok" }),
            ))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_and_parse_search(
    url: &str,
) -> Result<(Vec<SearchAnimeItem>, PaginationWithStringPages), Box<dyn std::error::Error + Send + Sync>> {
    let html = fetch_html_with_retry(url).await?;
    let (data, pagination) = tokio::task::spawn_blocking(move || {
        parse_search_document(&html)
    })
    .await??;

    Ok((data, pagination))
}

fn parse_search_document(
    html: &str,
) -> Result<(Vec<SearchAnimeItem>, PaginationWithStringPages), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);

    // Parse anime items using shared parser
    let data = parsers::parse_search_anime(html)?;

    // Parse pagination using shared parser
    let current_page = 1; // Search results always start at page 1
    let pagination = parsers::parse_pagination_with_string(&document, current_page);

    Ok((data, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}