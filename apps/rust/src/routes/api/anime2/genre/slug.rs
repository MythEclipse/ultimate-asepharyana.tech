use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

// Import shared models and parsers
use crate::models::anime2::{GenreAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/genre/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Filter anime2 by genre with advanced options";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_genre_filter";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<GenreAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GenreAnimeResponse {
    pub status: String,
    pub genre: String,
    pub data: Vec<GenreAnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct GenreQuery {
    pub page: Option<u32>,
    pub status: Option<String>,
    pub order: Option<String>,
}

const CACHE_TTL: u64 = 300;

#[utoipa::path(
    get,
    params(
        ("genre_slug" = String, Path, description = "Parameter for resource identification", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1),
        ("status" = Option<String>, Query, description = "Status filter (active, inactive, pending, etc.)", example = "sample_value"),
        ("order" = Option<String>, Query, description = "Sort direction (ascending or descending)", example = "sample_value")
    ),
    path = "/api/anime2/genre/{slug}",
    tag = "anime2",
    operation_id = "anime2_genre_filter",
    responses(
        (status = 200, description = "Filter anime2 by genre with advanced options", body = GenreAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(genre_slug): Path<String>,
    Query(params): Query<GenreQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let status = params.status.clone().unwrap_or_default();
    let order = params.order.clone().unwrap_or("update".to_string());

    info!(
        "anime2 genre request: {}, page: {}, status: {}, order: {}",
        genre_slug, page, status, order
    );

    let cache_key = format!("anime2:genre:{}:{}:{}:{}", genre_slug, page, status, order);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (anime_list, pagination) =
                fetch_genre_anime(&genre_slug, page, &status, &order)
                    .await
                    .map_err(|e: Box<dyn std::error::Error + Send + Sync>| e.to_string())?;

            // Convert all poster URLs to CDN URLs concurrently
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();
            
            // Store posters in a separate vector to avoid borrow checker issues
            let posters: Vec<String> = anime_list.iter().map(|i| i.poster.clone()).collect();
            crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // Return original data explicitly for speed
            Ok(GenreAnimeResponse {
                status: "Ok".to_string(),
                genre: genre_slug.clone(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_genre_anime(
    genre_slug: &str,
    page: u32,
    status: &str,
    order: &str,
) -> Result<(Vec<GenreAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let mut url = if page > 1 {
        format!(
            "https://alqanime.si/anime/page/{}/?genre[]={}",
            page, genre_slug
        )
    } else {
        format!("https://alqanime.si/anime/?genre[]={}", genre_slug)
    };

    if !status.is_empty() {
        url.push_str(&format!("&status={}", status));
    }
    url.push_str(&format!("&order={}", order));

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<GenreAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    
    // Parse anime items using shared parser
    let anime_list = parsers::parse_genre_anime(html)?;

    // Parse pagination using shared parser
    let pagination = parsers::parse_pagination(&document, current_page);

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}