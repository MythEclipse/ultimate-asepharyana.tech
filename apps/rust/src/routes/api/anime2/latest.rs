use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, Cache};
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::Router;

use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

// Import shared models and parsers
use crate::models::anime2::{LatestAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;
use crate::helpers::anime2_cache as cache_utils;


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
        (status = 200, description = "Get latest anime2 updates with pagination", body = ApiResponse<Vec<LatestAnimeItem>>),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn latest(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<LatestQuery>,
) -> ApiResult<Vec<LatestAnimeItem>> {
    let page = params.page.unwrap_or(1);
    info!("anime2 latest request, page: {}", page);

    let cache_key = format!("anime2:latest:{}", page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (data, pagination) = fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Use shared cache utility for poster caching
            let updated_data = cache_utils::cache_and_update_posters(&app_state, data).await;

            Ok(ApiResponse::success_with_meta(
                updated_data,
                json!({ "pagination": pagination }),
            ))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_latest_anime(
    page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://alqanime.si/anime/page/{}/?status=&type=&order=latest",
        page
    );

    let html = fetch_html_with_retry(&url)
        .await
        .map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_latest_page(&html, page)).await??;

    Ok((anime_list, pagination))
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
    router
}