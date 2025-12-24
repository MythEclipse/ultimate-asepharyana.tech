use crate::fetch_with_proxy::fetch_with_proxy;
use crate::routes::AppState;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use backoff::{future::retry, ExponentialBackoff};
use deadpool_redis::redis::AsyncCommands;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/search";
pub const ENDPOINT_DESCRIPTION: &str = "Searches for komik based on query parameters.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_search";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaItem {
    pub title: String,
    pub poster: String,
    pub chapter: String,
    pub score: String,
    pub date: String,
    pub r#type: String,
    pub slug: String,
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
    pub data: Vec<MangaItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct SearchQuery {
    /// Search query string to filter komik results
    pub query: Option<String>,
    /// Page number for pagination (defaults to 1)
    pub page: Option<u32>,
}

use axum::extract::State;

lazy_static! {
  pub static ref ANIMPOST_SELECTOR: Selector = Selector::parse("div.bge, .listupd .bge").unwrap();
  pub static ref TITLE_SELECTOR: Selector = Selector::parse(
    "div.kan h3, div.kan a h3, .tt h3"
  ).unwrap();
  pub static ref IMG_SELECTOR: Selector = Selector::parse("div.bgei img").unwrap();
  pub static ref CHAPTER_SELECTOR: Selector = Selector::parse(
    "div.new1 a span:last-child, .new1 span, .lch"
  ).unwrap();
  pub static ref SCORE_SELECTOR: Selector = Selector::parse(".up, .epx, .numscore").unwrap(); // broader match
  pub static ref DATE_SELECTOR: Selector = Selector::parse(
    "div.kan span.judul2, .mdis .date"
  ).unwrap();
  pub static ref TYPE_SELECTOR: Selector = Selector::parse(
    "div.tpe1_inf b, .tpe1_inf span.type, .mdis .type"
  ).unwrap();
  pub static ref LINK_SELECTOR: Selector = Selector::parse("div.bgei a, div.kan a").unwrap();
  pub static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
  pub static ref CURRENT_SELECTOR: Selector = Selector::parse(
    ".pagination > .current, .pagination > span.page-numbers.current, .hpage .current"
  ).unwrap();
  pub static ref PAGE_SELECTORS: Selector = Selector::parse(
    ".pagination > a, .pagination > .page-numbers:not(.next):not(.prev), .hpage a"
  ).unwrap();
  pub static ref NEXT_SELECTOR: Selector = Selector::parse(
    ".pagination > a.next, .pagination > .next.page-numbers, .hpage .next"
  ).unwrap();
  pub static ref PREV_SELECTOR: Selector = Selector::parse(
    ".pagination > a.prev, .pagination > .prev.page-numbers, .hpage .prev"
  ).unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("query" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/search",
    tag = "komik",
    operation_id = "komik_search",
    responses(
        (status = 200, description = "Searches for komik based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let start_time = std::time::Instant::now();
    let query = params.query.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    info!(
        "Starting komik search for query: '{}', page: {}",
        query, page
    );

    let cache_key = format!("komik:search:{}:{}", query, page);
    let mut conn = app_state.redis_pool.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Redis error: {}", e),
        )
    })?;

    // Try to get cached data
    let cached_response: Option<String> = conn.get(&cache_key).await.map_err(|e| {
        error!("Failed to get data from Redis: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Redis error: {}", e),
        )
    })?;

    if let Some(json_data_string) = cached_response {
        info!("Cache hit for key: {}", cache_key);
        let search_response: SearchResponse =
            serde_json::from_str(&json_data_string).map_err(|e| {
                error!("Failed to deserialize cached data: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Serialization error: {}", e),
                )
            })?;
        return Ok(Json(search_response));
    }

    let url = format!(
        "https://api.komiku.org/?post_type=manga&s={}",
        urlencoding::encode(&query)
    );

    let (data, pagination) = fetch_and_parse_search(&url, page).await.map_err(|e| {
        error!(
            "Failed to process komik search for query: '{}', page: {} after {:?}, error: {:?}",
            query,
            page,
            start_time.elapsed(),
            e
        );
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
    })?;

    let search_response = SearchResponse { data, pagination };
    let json_data = serde_json::to_string(&search_response).map_err(|e| {
        error!("Failed to serialize response for caching: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialization error: {}", e),
        )
    })?;

    // Store in Redis with TTL
    conn.set_ex::<_, _, ()>(&cache_key, json_data, CACHE_TTL)
        .await
        .map_err(|e| {
            error!("Failed to set data in Redis: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Redis error: {}", e),
            )
        })?;
    info!("Cache set for key: {}", cache_key);

    let total_duration = start_time.elapsed();
    info!(
        "Successfully processed komik search for query: '{}', page: {} in {:?}",
        query, page, total_duration
    );
    Ok(Json(search_response))
}

async fn fetch_and_parse_search(
    url: &str,
    page: u32,
) -> Result<(Vec<MangaItem>, Pagination), String> {
    let backoff = ExponentialBackoff {
        initial_interval: Duration::from_millis(500),
        max_interval: Duration::from_secs(10),
        multiplier: 2.0,
        max_elapsed_time: Some(Duration::from_secs(30)),
        ..Default::default()
    };

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(url).await {
            Ok(response) => {
                info!("Successfully fetched URL: {}", url);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch URL: {}, error: {:?}", url, e);
                Err(backoff::Error::transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation)
        .await
        .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;

    let parse_result = tokio::task::spawn_blocking(move || parse_search_document(html, page)).await;
    match parse_result {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(format!("Blocking task failed: {}", join_err)),
    }
}

fn parse_search_document(
    html_string: String,
    current_page: u32,
) -> Result<(Vec<MangaItem>, Pagination), String> {
    let document = Html::parse_document(&html_string);
    let mut data = Vec::new();

    for element in document.select(&ANIMPOST_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let mut poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| {
                e.value()
                    .attr("src")
                    .or_else(|| e.value().attr("data-src"))
                    .or_else(|| e.value().attr("data-lazy-src"))
                    .or_else(|| {
                        e.value()
                            .attr("srcset")
                            .and_then(|s| s.split_whitespace().next())
                    })
            })
            .unwrap_or("")
            .to_string();
        poster = poster.split('?').next().unwrap_or(&poster).to_string();

        let chapter = element
            .select(&CHAPTER_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .and_then(|text| {
                CHAPTER_REGEX
                    .captures(&text)
                    .and_then(|cap| cap.get(0))
                    .map(|m| m.as_str().to_string())
            })
            .unwrap_or_default();

        let score = element
            .select(&SCORE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let date = element
            .select(&DATE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let r#type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .map(|text| text.split_whitespace().next().unwrap_or("").to_string())
            .unwrap_or_default();

        let slug = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                let parts: Vec<&str> = href.split('/').filter(|s| !s.is_empty()).collect();
                if let Some(pos) = parts
                    .iter()
                    .position(|s| *s == "manga" || *s == "manhua" || *s == "manhwa")
                {
                    parts.get(pos + 1).cloned().unwrap_or("").to_string()
                } else {
                    parts.last().cloned().unwrap_or("").to_string()
                }
            })
            .unwrap_or_default();

        data.push(MangaItem {
            title,
            poster,
            chapter,
            score,
            date,
            r#type,
            slug,
        });
    }

    let last_visible_page = document
        .select(&PAGE_SELECTORS)
        .next_back()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
    let has_previous_page = document.select(&PREV_SELECTOR).next().is_some();

    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page: if has_next_page {
            Some(current_page + 1)
        } else {
            None
        },
        has_previous_page,
        previous_page: if has_previous_page {
            Some(current_page - 1)
        } else {
            None
        },
    };

    Ok((data, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}