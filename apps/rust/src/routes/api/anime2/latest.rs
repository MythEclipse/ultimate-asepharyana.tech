use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, attr_from_or, extract_slug, text, attr};
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/latest";
pub const ENDPOINT_DESCRIPTION: &str = "Get latest anime2 updates with pagination";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_latest";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LatestAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LatestAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub score: String,
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
            let (anime_list, pagination) =
                fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs (returns original + background cache)
            // Trigger background caching for all posters and return immediately
            // This ensures cold start is fast (returning original URLs) while caching happens in background
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            // Extract all poster URLs
            let posters: Vec<String> = anime_list.iter().map(|item| item.poster.clone()).collect();

            // Trigger lazy batch caching
            crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db.clone(),
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // We return `anime_list` as is (with original posters).
            // Effectively 0ms added latency.

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
    let url = format!(
        "https://alqanime.si/anime/page/{}/?status=&type=&order=latest",
        page
    );

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_latest_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_latest_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut anime_list = Vec::new();
    
    let item_selector = selector("article.bs").unwrap();
    let title_selector = selector(".tt h2").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epx").unwrap();
    let score_selector = selector(".numscore").unwrap();
    let link_selector = selector("a").unwrap();
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagination .next").unwrap();

    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| attr(&e, "src").or(attr(&e, "data-src")))
            .unwrap_or_default();

        let current_episode = text_from_or(&element, &episode_selector, "N/A");

        let score = text_from_or(&element, &score_selector, "N/A");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        let slug = extract_slug(&anime_url);

        if !title.is_empty() {
            anime_list.push(LatestAnimeItem {
                title,
                slug,
                poster,
                current_episode,
                score,
                anime_url,
            });
        }
    }

    let last_visible_page = document
        .select(&pagination_selector)
        .next_back()
        .map(|e| {
            text(&e)
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        })
        .unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();
    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page: if has_next_page {
            Some(current_page + 1)
        } else {
            None
        },
        has_previous_page: current_page > 1,
        previous_page: if current_page > 1 {
            Some(current_page - 1)
        } else {
            None
        },
    };

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(latest))
}