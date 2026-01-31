use crate::helpers::{internal_err, Cache, fetch_html_with_retry, text_from_or, attr_from_or};
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/latest";
pub const ENDPOINT_DESCRIPTION: &str = "Get latest anime updates with pagination";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_latest";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LatestAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LatestAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub release_time: String,
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
pub struct LatestAnimeResponse {
    pub status: String,
    pub data: Vec<LatestAnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct LatestQuery {
    pub page: Option<u32>,
}

lazy_static! {
    static ref VENZ_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref EP_SELECTOR: Selector = Selector::parse(".epz").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 120; // 2 minutes - latest updates change frequently

#[utoipa::path(
    get,
    params(
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/anime/latest",
    tag = "anime",
    operation_id = "anime_latest",
    responses(
        (status = 200, description = "Get latest anime updates with pagination", body = LatestAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn latest(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<LatestQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    info!("Handling request for latest anime, page: {}", page);

    let cache_key = format!("anime:latest:{}", page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (mut anime_list, pagination) =
                fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = anime_list.iter().map(|i| i.poster.clone()).collect();
            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            for (i, item) in anime_list.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            // Return original URLs immediately.
            // Posters will be cached in background and available on next request.

            Ok(LatestAnimeResponse {
                status: "Ok".to_string(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_latest_anime(
    page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = if page == 1 {
        format!("{}/ongoing-anime/", get_otakudesu_url())
    } else {
        format!("{}/ongoing-anime/page/{}/", get_otakudesu_url(), page)
    };

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_latest_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_latest_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut anime_list = Vec::new();

    for element in document.select(&VENZ_SELECTOR) {
        let title = text_from_or(&element, &TITLE_SELECTOR, "");

        let poster = attr_from_or(&element, &IMG_SELECTOR, "src", "");

        let current_episode = text_from_or(&element, &EP_SELECTOR, "N/A");

        let anime_url = attr_from_or(&element, &LINK_SELECTOR, "href", "");

        let slug = SLUG_REGEX
            .captures(&anime_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        // Extract release time if available
        let release_time = "Recently".to_string(); // Could be enhanced with actual time

        if !title.is_empty() {
            anime_list.push(LatestAnimeItem {
                title,
                slug,
                poster,
                current_episode,
                release_time,
                anime_url,
            });
        }
    }

    let last_visible_page = document
        .select(&PAGINATION_SELECTOR)
        .next_back()
        .map(|e| {
            e.text()
                .collect::<String>()
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        })
        .unwrap_or(1);

    let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
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

    info!("Parsed {} latest anime items", anime_list.len());
    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(latest))
}