// Standard library imports
use std::sync::Arc;
use std::time::Duration;

// External crate imports
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use backoff::{future::retry, ExponentialBackoff};
use deadpool_redis::redis::AsyncCommands;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

// Internal imports
use crate::fetch_with_proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::urls::OTAKUDESU_BASE_URL;

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

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

// Pre-compiled CSS selectors for performance
lazy_static! {
    static ref ITEM_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref EPISODE_SELECTOR: Selector = Selector::parse(".epz").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".pagenavix .page-numbers:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector = Selector::parse(".pagenavix .next.page-numbers").unwrap();
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
    let start = std::time::Instant::now();
    info!("Starting request for complete_anime slug: {}", slug);

    let cache_key = format!("anime:complete:{}", slug);
    let mut conn = app_state.redis_pool.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Redis error: {}", e),
        )
    })?;

    // Check cache first
    let cached_response: Option<String> = conn.get(&cache_key).await.map_err(|e| {
        error!("Failed to get data from Redis: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Redis error: {}", e),
        )
    })?;

    if let Some(json_data_string) = cached_response {
        info!("Cache hit for key: {}", cache_key);
        let list_response: ListResponse = serde_json::from_str(&json_data_string).map_err(|e| {
            error!("Failed to deserialize cached data: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Serialization error: {}", e),
            )
        })?;
        return Ok(Json(list_response).into_response());
    }

    let url = format!("{}/complete-anime/page/{}/", OTAKUDESU_BASE_URL, slug);

    // Fetch and parse HTML
    match fetch_html_with_retry(&url).await {
        Ok(html) => {
            // Parse HTML in a blocking task to avoid blocking the async runtime
            let html_clone = html.clone();
            let slug_clone = slug.clone();
            let parse_result =
                tokio::task::spawn_blocking(move || parse_anime_page(&html_clone, &slug_clone))
                    .await;

            let (anime_list, pagination) = match parse_result {
                Ok(inner_result) => match inner_result {
                    Ok(data) => data,
                    Err(e) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Error parsing anime page: {}", e),
                        ));
                    }
                },
                Err(join_err) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Blocking task join error: {}", join_err),
                    ));
                }
            };

            // Build response
            let list_response = ListResponse {
                message: "Success".to_string(),
                data: anime_list.clone(),
                total: Some(anime_list.len() as i64),
                pagination: Some(pagination),
            };

            // Serialize and cache the response
            let json_data = serde_json::to_string(&list_response).map_err(|e| {
                error!("Failed to serialize response for caching: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Serialization error: {}", e),
                )
            })?;

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

            let duration = start.elapsed();
            info!(
                "Fetched and parsed for slug: {}, duration: {:?}",
                slug, duration
            );
            Ok(Json(list_response).into_response())
        }
        Err(e) => {
            let duration = start.elapsed();
            error!(
                "Error fetching for slug: {}, error: {:?}, duration: {:?}",
                slug, e, duration
            );
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)))
        }
    }
}

/// Fetches HTML content with exponential backoff retry mechanism
async fn fetch_html_with_retry(
    url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
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
                Err(backoff::Error::transient(
                    Box::new(e) as Box<dyn std::error::Error + Send + Sync>
                ))
            }
        }
    };

    retry(backoff, fetch_operation).await
}

/// Parses HTML document to extract anime items and pagination information
fn parse_anime_page(
    html: &str,
    slug: &str,
) -> Result<(Vec<CompleteAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut anime_list = Vec::new();

    // Extract anime items
    for element in document.select(&ITEM_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let slug = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| {
                SLUG_REGEX
                    .captures(href)
                    .and_then(|cap| cap.get(1))
                    .map(|m| m.as_str())
            })
            .unwrap_or("")
            .to_string();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let episode_count = element
            .select(&EPISODE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

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
        .select(&PAGINATION_SELECTOR)
        .next_back()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
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

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}