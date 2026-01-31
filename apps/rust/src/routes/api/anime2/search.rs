use crate::helpers::{internal_err, parse_html, Cache, fetch_html_with_retry};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

// Import shared models and parsers
use crate::models::anime2::{SearchAnimeItem, PaginationWithStringPages};
use crate::scraping::anime2 as parsers;
use crate::helpers::anime2_cache as cache_utils;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/search";
pub const ENDPOINT_DESCRIPTION: &str = "Searches for anime2 based on query parameters.";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_search";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

const CACHE_TTL: u64 = 300; // 5 minutes

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SearchResponse {
    pub status: String,
    pub data: Vec<SearchAnimeItem>,
    pub pagination: PaginationWithStringPages,
}

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
        (status = 200, description = "Searches for anime2 based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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

            Ok(SearchResponse {
                status: "Ok".to_string(),
                data,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
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
    router.route(ENDPOINT_PATH, get(search))
}