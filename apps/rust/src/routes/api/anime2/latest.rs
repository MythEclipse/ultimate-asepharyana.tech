use crate::helpers::{default_backoff, get_cached_or_original, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
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

lazy_static! {
    static ref ITEM_SELECTOR: Selector = Selector::parse(".listupd .bs").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".ntitle").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref EPISODE_SELECTOR: Selector = Selector::parse(".epx").unwrap();
    static ref SCORE_SELECTOR: Selector = Selector::parse(".numscore").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

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
            let (mut anime_list, pagination) =
                fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs (returns original + background cache)
            for item in &mut anime_list {
                if !item.poster.is_empty() {
                    item.poster = get_cached_or_original(&app_state.db, &app_state.redis_pool, &item.poster).await;
                }
            }

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
        "https://alqanime.net/advanced-search/page/{}/?order=latest",
        page
    );

    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => Ok(response.data),
            Err(e) => {
                warn!("Failed: {:?}", e);
                Err(transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;

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

    for element in document.select(&ITEM_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let current_episode = element
            .select(&EPISODE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let score = element
            .select(&SCORE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let slug = SLUG_REGEX
            .captures(&anime_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

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