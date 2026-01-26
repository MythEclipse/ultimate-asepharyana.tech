//! Handler for the komik chapter endpoint.

use crate::helpers::{
    cache_image_urls_batch_lazy, default_backoff, internal_err, transient, Cache,
};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_komik_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, routing::get, Json, Router};
use backoff::future::retry;

use lazy_static::lazy_static;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/chapter";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves chapter data for a specific komik chapter.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_chapter";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ChapterResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChapterData {
    pub title: String,
    pub next_chapter_id: String,
    pub prev_chapter_id: String,
    pub list_chapter: String,
    pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChapterResponse {
    pub message: String,
    pub data: ChapterData,
}

#[derive(Deserialize, ToSchema)]
pub struct ChapterQuery {
    /// URL-friendly identifier for the chapter (typically the chapter slug or URL path)
    pub chapter_url: Option<String>,
}

lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse("title").unwrap();
    static ref PREV_CHAPTER_SELECTOR: Selector =
        Selector::parse(".nxpr a:not(.rl):not([href*='#Chapter']), .chprev a, a.prev").unwrap();
    static ref LIST_CHAPTER_SELECTOR: Selector =
        Selector::parse("table#Daftar_Chapter tbody tr").unwrap();
    static ref NEXT_CHAPTER_SELECTOR: Selector =
        Selector::parse(".nxpr a.rl, .nxpr a.next, .chnext a, a.next").unwrap();
    static ref IMAGE_SELECTOR: Selector = Selector::parse("#Baca_Komik img").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("chapter_url" = Option<String>, Query, description = "Chapter-specific identifier", example = "sample_value")
    ),
    path = "/api/komik/chapter",
    tag = "komik",
    operation_id = "komik_chapter",
    responses(
        (status = 200, description = "Retrieves chapter data for a specific komik chapter.", body = ChapterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn chapter(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<ChapterQuery>,
) -> Result<Json<ChapterResponse>, (StatusCode, String)> {
    let chapter_url = params.chapter_url.unwrap_or_default();
    info!("Handling request for komik chapter: {}", chapter_url);

    let cache_key = format!("komik:chapter:{}", chapter_url);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let mut data = fetch_komik_chapter(chapter_url.clone())
                .await
                .map_err(|e| e.to_string())?;

            // Cache all images in background (lazy)
            // This returns original URLs immediately but triggers caching for next time
            data.images =
                cache_image_urls_batch_lazy(app_state.db.clone(), &app_state.redis_pool, data.images);

            Ok(ChapterResponse {
                message: "Ok".to_string(),
                data,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response))
}

pub async fn fetch_komik_chapter(
    chapter_url: String,
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
    let base_url = get_komik_url();
    let url = format!("{}/{}", base_url, chapter_url); // Keep as-is since chapter URLs might already have correct format

    // Retry logic with exponential backoff
    let backoff = default_backoff();

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(&url).await {
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

    let html = retry(backoff, fetch_operation).await?;

    tokio::task::spawn_blocking(move || {
        parse_komik_chapter_document(&Html::parse_document(&html), &chapter_url)
    })
    .await?
}

fn parse_komik_chapter_document(
    document: &Html,
    chapter_url: &str,
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
    let _start_time = std::time::Instant::now();
    info!("Starting to parse komik chapter document");

    let title = document
        .select(&TITLE_SELECTOR)
        .next()
        .map(|e| {
            let full_title = e.text().collect::<String>();
            // Extract series title from "Chapter XX | Komik TITLE - Komiku"
            if let Some(start) = full_title.find("Komik ") {
                if let Some(end) = full_title.find(" - Komiku") {
                    full_title[start + 6..end].trim().to_string()
                } else {
                    full_title
                }
            } else {
                full_title
            }
        })
        .unwrap_or_default();

    let next_chapter_id = document
        .select(&NEXT_CHAPTER_SELECTOR)
        .next()
        .and_then(|e| e.value().attr("href"))
        .map(|href| {
            href.trim_end_matches('/')
                .split('/')
                .filter(|s| !s.is_empty())
                .next_back()
                .unwrap_or("")
                .to_string()
        })
        .unwrap_or_default();

    // Function to extract and decrement chapter number from URL for any series
    fn get_previous_chapter_id(chapter_url: &str) -> String {
        // Pattern: "*-chapter-XX" where XX is chapter number
        const CHAPTER_PATTERN: &str = "chapter-";

        // Look for "chapter-" pattern in the URL
        if let Some(pattern_pos) = chapter_url.rfind(CHAPTER_PATTERN) {
            let prefix = &chapter_url[0..pattern_pos];
            let suffix = &chapter_url[pattern_pos + CHAPTER_PATTERN.len()..];

            // Try to extract chapter number (handles 1, 2, or more digit numbers)
            let chapter_num = suffix
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            if let Ok(num) = chapter_num.parse::<u32>() {
                // Decrement chapter number, but not below 1 (not 0 as requested)
                let prev_num = num.saturating_sub(1);

                // Format with same number of digits as original (preserve leading zeros if any)
                let formatted_num = if chapter_num.starts_with('0') {
                    // Keep leading zeros if original had them
                    format!("{:0width$}", prev_num, width = chapter_num.len())
                } else {
                    // No leading zeros needed
                    prev_num.to_string()
                };

                // Reconstruct with previous chapter number
                return format!("{}{}{}", prefix, CHAPTER_PATTERN, formatted_num);
            }
        }

        // If we can't determine a previous chapter from URL, return empty string
        String::new()
    }

    // Get previous chapter ID - first try URL pattern, then fall back to HTML extraction
    let prev_chapter_id_from_url = get_previous_chapter_id(chapter_url);
    let prev_chapter_id = if !prev_chapter_id_from_url.is_empty() {
        prev_chapter_id_from_url
    } else {
        // Fall back to HTML parsing if URL pattern doesn't match
        document
            .select(&PREV_CHAPTER_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                href.trim_end_matches('/')
                    .split('/')
                    .filter(|s| !s.is_empty())
                    .next_back()
                    .unwrap_or("")
                    .to_string()
            })
            .unwrap_or_default()
    };

    fn get_list_chapter_from_url(chapter_url: &str) -> String {
        let re = regex::Regex::new(r"-chapter-\d+").unwrap();
        re.replace_all(chapter_url, "").to_string()
    }

    let list_chapter = get_list_chapter_from_url(chapter_url);

    let mut images = Vec::new();
    let forbidden_images = [
        "https://flagcdn.com/32x24/jp.png",
        "https://flagcdn.com/32x24/kr.png",
        "https://flagcdn.com/32x24/cn.png",
        "https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/google.svg",
        "https://www.gravatar.com/avatar/?d=mp&s=80",
        "/asset/img/komikuplus2.jpg",
        "https://komiku.org/asset/img/Loading.gif",
    ];
    for el in document.select(&IMAGE_SELECTOR) {
        if let Some(src) = el
            .value()
            .attr("src")
            .or_else(|| el.value().attr("data-src"))
            .or_else(|| el.value().attr("data-lazy-src"))
            .or_else(|| {
                el.value()
                    .attr("srcset")
                    .and_then(|s| s.split_whitespace().next())
            })
        {
            if !forbidden_images.contains(&src) {
                images.push(src.to_string());
            }
        }
    }

    Ok(ChapterData {
        title,
        next_chapter_id,
        prev_chapter_id,
        list_chapter,
        images,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(chapter))
}