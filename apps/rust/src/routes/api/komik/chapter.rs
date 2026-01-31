//! Handler for the komik chapter endpoint.

use crate::helpers::{internal_err, Cache, fetch_html_with_retry, cache_image_urls_batch_lazy, parse_html};
use crate::helpers::scraping::{selector, text, attr};
use crate::routes::AppState;
use crate::scraping::urls::get_komik_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, routing::get, Json, Router};

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
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
            data.images = cache_image_urls_batch_lazy(
                app_state.db.clone(),
                &app_state.redis_pool,
                data.images,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

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

    let html = fetch_html_with_retry(&url).await?;

    tokio::task::spawn_blocking(move || {
        parse_komik_chapter_document(&html, &chapter_url)
    })
    .await?
}

fn parse_komik_chapter_document(
    html: &str,
    chapter_url: &str,
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let _start_time = std::time::Instant::now();
    info!("Starting to parse komik chapter document");

    let title_selector = selector("title").unwrap();
    let prev_chapter_selector = selector(".nxpr a:not(.rl):not([href*='#Chapter']), .chprev a, a.prev").unwrap();
    let next_chapter_selector = selector(".nxpr a.rl, .nxpr a.next, .chnext a, a.next").unwrap();
    let image_selector = selector("#Baca_Komik img").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|e| {
            let full_title = text(&e);
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
        .select(&next_chapter_selector)
        .next()
        .and_then(|e| attr(&e, "href"))
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
            .select(&prev_chapter_selector)
            .next()
            .and_then(|e| attr(&e, "href"))
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
        let re = Regex::new(r"-chapter-\d+").unwrap();
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
    for el in document.select(&image_selector) {
        if let Some(src) = attr(&el, "src")
            .or_else(|| attr(&el, "data-src"))
            .or_else(|| attr(&el, "data-lazy-src"))
            .or_else(|| {
                attr(&el, "srcset")
                    .and_then(|s| s.split_whitespace().next().map(|s| s.to_string()))
            })
        {
            if !forbidden_images.contains(&src.as_str()) {
                images.push(src);
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