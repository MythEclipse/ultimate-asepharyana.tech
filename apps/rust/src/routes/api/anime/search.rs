// Standard library imports
use std::sync::Arc;

// External crate imports
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use backoff::future::retry;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use utoipa::ToSchema;

// Internal imports
use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/search";
pub const ENDPOINT_DESCRIPTION: &str = "Searches for anime based on query parameters.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_search";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub status: String,
    pub rating: String,
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
pub struct SearchResponse {
    pub status: String,
    pub data: Vec<AnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct SearchQuery {
    pub q: Option<String>,
}

lazy_static! {
    pub static ref ITEM_SELECTOR: Selector = Selector::parse("#venkonten .chivsrc li").unwrap();
    pub static ref TITLE_SELECTOR: Selector = Selector::parse("h2 a").unwrap();
    pub static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    pub static ref GENRE_SELECTOR: Selector = Selector::parse(".set a").unwrap();
    pub static ref STATUS_SELECTOR: Selector = Selector::parse(".set").unwrap();
    pub static ref NEXT_SELECTOR: Selector = Selector::parse(".hpage .r").unwrap();
    pub static ref EPISODE_REGEX: Regex = Regex::new(r"\(([^)]+)\)").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("q" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value")
    ),
    path = "/api/anime/search",
    tag = "anime",
    operation_id = "anime_search",
    responses(
        (status = 200, description = "Searches for anime based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let start = std::time::Instant::now();
    let query = params.q.unwrap_or_else(|| "one".to_string());
    info!("Starting search for query: {}", query);

    let cache_key = format!("anime:search:{}", query);
    let cache = Cache::new(&app_state.redis_pool);

    // Use get_or_set pattern - much cleaner!
    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let url = format!(
                "{}/?s={}&post_type=anime",
                get_otakudesu_url(),
                urlencoding::encode(&query)
            );

            let (mut data, pagination) = fetch_and_parse_search(&url)
                .await
                .map_err(|e| format!("Fetch error: {}", e))?;

            // Convert all poster URLs to CDN URLs
            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = data.iter().map(|i| i.poster.clone()).collect();
            crate::helpers::image_cache::cache_image_urls_batch_lazy(&db, &redis, posters);

            Ok(SearchResponse {
                status: "Ok".to_string(),
                data,
                pagination,
            })
        })
        .await
        .map_err(internal_err)?;

    let duration = start.elapsed();
    info!(
        "Search completed for query: {}, duration: {:?}",
        query, duration
    );

    Ok(Json(response).into_response())
}

async fn fetch_and_parse_search(
    url: &str,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let backoff = default_backoff();

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(url).await {
            Ok(response) => {
                info!("Successfully fetched URL: {}", url);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch URL: {}, error: {:?}", url, e);
                Err(transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation)
        .await
        .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;

    match tokio::task::spawn_blocking(move || parse_search_html(&html)).await {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
    }
}

fn parse_search_html(
    html: &str,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
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
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let slug = anime_url
            .split('/')
            .filter(|s| !s.is_empty())
            .last()
            .unwrap_or("")
            .to_string();

        let genres: Vec<String> = element
            .select(&GENRE_SELECTOR)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect();

        let status_text = element
            .select(&STATUS_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let episode = EPISODE_REGEX
            .captures(&status_text)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or("N/A".to_string());

        let status = if status_text.to_lowercase().contains("ongoing") {
            "Ongoing".to_string()
        } else if status_text.to_lowercase().contains("completed") {
            "Completed".to_string()
        } else {
            "Unknown".to_string()
        };

        let rating = "N/A".to_string();

        if !title.is_empty() {
            anime_list.push(AnimeItem {
                title,
                slug,
                poster,
                episode,
                anime_url,
                genres,
                status,
                rating,
            });
        }
    }

    let pagination = Pagination {
        current_page: 1,
        last_visible_page: 1,
        has_next_page: document.select(&NEXT_SELECTOR).next().is_some(),
        next_page: if document.select(&NEXT_SELECTOR).next().is_some() {
            Some(2)
        } else {
            None
        },
        has_previous_page: false,
        previous_page: None,
    };

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}