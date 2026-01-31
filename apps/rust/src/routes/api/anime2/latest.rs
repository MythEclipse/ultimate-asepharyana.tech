use crate::helpers::{internal_err, Cache, fetch_html_with_retry};
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

// Import shared models and parsers
use crate::models::anime2::{LatestAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;
use crate::helpers::anime2_cache as cache_utils;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/latest";
pub const ENDPOINT_DESCRIPTION: &str = "Get latest anime2 updates with pagination";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_latest";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LatestAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LatestAnimeResponse {
    pub status: String,
    pub data: Vec<LatestAnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct LatestQuery {
    pub page: Option<u32>,
}

const CACHE_TTL: u64 = 120;

#[utoipa::path(
    get,
    params(
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/anime2/latest",
    tag = "anime2",
    operation_id = "anime2_latest",
    responses(
        (status = 200, description = "Get latest anime2 updates with pagination", body = LatestAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn latest(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<LatestQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    info!("anime2 latest request, page: {}", page);

    let cache_key = format!("anime2:latest:{}", page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let response = fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Use shared cache utility for poster caching
            let updated_data = cache_utils::cache_and_update_posters(&app_state, response.data).await;

            Ok(LatestAnimeResponse {
                status: "Ok".to_string(),
                data: updated_data,
                pagination: response.pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_latest_anime(
    page: u32,
) -> Result<LatestAnimeResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://alqanime.si/anime/page/{}/?status=&type=&order=latest",
        page
    );

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_latest_page(&html, page)).await??;

    Ok(LatestAnimeResponse {
        status: "Ok".to_string(),
        data: anime_list,
        pagination,
    })
}

fn parse_latest_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = crate::helpers::parse_html(html);
    
    // Use shared parser for anime items
    let anime_list = parsers::parse_latest_anime(html)?;
    
    // Use shared parser for pagination
    let pagination = parsers::parse_pagination(&document, current_page);

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(latest))
}