// Standard library imports
use std::sync::Arc;

// External crate imports
use crate::helpers::{
    internal_err, Cache, fetch_html_with_retry, text_from_or, attr_from_or, extract_slug,
    parse_html, selector
};
use crate::routes::AppState;
use crate::scraping::urls::OTAKUDESU_BASE_URL;
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

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/complete-anime/{slug}";
pub const ENDPOINT_DESCRIPTION: &str =
    "Handles GET requests for the anime/complete-anime/slug endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_complete_anime_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ListResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompleteAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<u32>,
    pub has_previous_page: bool,
    pub previous_page: Option<u32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ListResponse {
    pub message: String,
    pub data: Vec<CompleteAnimeItem>,
    pub total: Option<i64>,
    pub pagination: Option<Pagination>,
}

// Cache configuration
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime/complete-anime/{slug}",
    tag = "anime",
    operation_id = "anime_complete_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/complete-anime/slug endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _start = std::time::Instant::now();
    info!("Starting request for complete_anime slug: {}", slug);

    let cache_key = format!("anime:complete:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let url = format!("{}/complete-anime/page/{}/", OTAKUDESU_BASE_URL, slug);

            let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

            let (anime_list, pagination) =
                tokio::task::spawn_blocking(move || parse_anime_page(&html, &slug))
                    .await
                    .map_err(|e| e.to_string())?
                    .map_err(|e| e.to_string())?;

            let total = anime_list.len() as i64;
            Ok(ListResponse {
                message: "Success".to_string(),
                data: anime_list,
                total: Some(total),
                pagination: Some(pagination),
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    return Ok(Json(response).into_response());
}
/// Parses HTML document to extract anime items and pagination information
fn parse_anime_page(
    html: &str,
    slug: &str,
) -> Result<(Vec<CompleteAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut anime_list = Vec::new();

    let item_selector = selector(".venz ul li").unwrap();
    let title_selector = selector(".thumbz h2.jdlflm").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epz").unwrap();
    let pagination_selector = selector(".pagenavix .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagenavix .next.page-numbers").unwrap();

    // Extract anime items
    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");
        let anime_url = attr_from_or(&element, &link_selector, "href", "");
        let slug = extract_slug(&anime_url);
        let poster = attr_from_or(&element, &img_selector, "src", "");
        let episode_count = text_from_or(&element, &episode_selector, "N/A");

        if !title.is_empty() {
            anime_list.push(CompleteAnimeItem {
                title,
                slug,
                poster,
                episode_count,
                anime_url,
            });
        }
    }

    // Extract pagination information
    let current_page = slug.parse::<u32>().unwrap_or(1);
    let last_visible_page = document
        .select(&pagination_selector)
        .next_back()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();
    let next_page = if has_next_page {
        Some(current_page + 1)
    } else {
        None
    };
    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some(current_page - 1)
    } else {
        None
    };

    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    };

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}