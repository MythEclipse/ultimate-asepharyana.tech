use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;

use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/manhwa";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the komik/manhwa endpoint.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_manhwa_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ManhwaResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ManhwaItem {
    pub title: String,
    pub poster: String,
    pub chapter: String,
    pub date: String,
    pub reader_count: String,
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
pub struct ManhwaResponse {
    pub data: Vec<ManhwaItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct QueryParams {
    /// Page number for pagination (defaults to 1)
    pub page: Option<u32>,
}

lazy_static! {
    pub static ref ANIMPOST_SELECTOR: Selector = Selector::parse("div.bge, .listupd .bge").unwrap();
    pub static ref TITLE_SELECTOR: Selector =
        Selector::parse(".kan h3, .kan a h3, .tt h3").unwrap();
    pub static ref IMG_SELECTOR: Selector = Selector::parse(".bgei img").unwrap();
    pub static ref CHAPTER_SELECTOR: Selector =
        Selector::parse(".new1 a span:last-child, .new1 span, .lch").unwrap();
    pub static ref DATE_SELECTOR: Selector =
        Selector::parse(".judul2, .kan span.judul2, .mdis .date").unwrap();
    pub static ref TYPE_SELECTOR: Selector =
        Selector::parse(".tpe1_inf b, .tpe1_inf span.type, .mdis .type").unwrap();
    pub static ref LINK_SELECTOR: Selector = Selector::parse(".bgei a, .kan a").unwrap();
    pub static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
    pub static ref NEXT_PAGE_SPAN_SELECTOR: Selector =
        Selector::parse("body > span[hx-get]").unwrap();
    pub static ref PAGE_NUMBER_REGEX: Regex = Regex::new(r"/page/(\d+)/").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/manhwa",
    tag = "komik",
    operation_id = "komik_manhwa_slug",
    responses(
        (status = 200, description = "Handles GET requests for the komik/manhwa endpoint.", body = ManhwaResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn list(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _start_time = std::time::Instant::now();
    let page = params.page.unwrap_or(1);
    info!("Starting manhwa list request for page {}", page);

    let cache_key = format!("komik:manhwa:{}", page);

    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let base_api_url = get_komik_api_url();
            let url = if page == 1 {
                format!("{}/manga/?tipe=manhwa", base_api_url)
            } else {
                format!("{}/manga/page/{}/?tipe=manhwa", base_api_url, page)
            };

            let (data, pagination) = fetch_and_parse_manhwa_list(&url, page)
                .await
                .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = data.iter().map(|i| i.poster.clone()).collect();
            crate::helpers::image_cache::cache_image_urls_batch_lazy(&db, &redis, posters);

            Ok(ManhwaResponse { data, pagination })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_and_parse_manhwa_list(
    url: &str,
    page: u32,
) -> Result<(Vec<ManhwaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
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

    let html_string = retry(backoff, fetch_operation).await?.to_string();

    tokio::task::spawn_blocking(move || {
        let document = Html::parse_document(&html_string);
        parse_manhwa_list_document(&document, page)
    })
    .await?
}

fn parse_manhwa_list_document(
    document: &Html,
    current_page: u32,
) -> Result<(Vec<ManhwaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
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

        let chapter = {
            let mut found_chapter = String::new();
            for chapter_element in element.select(&LINK_SELECTOR) {
                let text = chapter_element
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string();
                if text.contains("Chapter") {
                    let processed_text = text
                        .replace("Terbaru:", "")
                        .replace("Awal:", "")
                        .trim()
                        .to_string();
                    if let Some(captures) = CHAPTER_REGEX.captures(&processed_text) {
                        if let Some(m) = captures.get(0) {
                            found_chapter = format!("Chapter {}", m.as_str());
                            // Prioritize "Terbaru" if found
                            if text.contains("Terbaru") {
                                break;
                            }
                        }
                    }
                }
            }
            found_chapter
        };

        let full_date_string = element
            .select(&DATE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let parts: Vec<&str> = full_date_string.split(" • ").collect();
        let date = parts.get(1).unwrap_or(&"").to_string();
        let pembaca = parts.first().unwrap_or(&"").to_string();

        let r#type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
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

        data.push(ManhwaItem {
            title,
            poster,
            chapter,
            date,
            reader_count: pembaca,
            r#type,
            slug,
        });
    }

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some(current_page - 1)
    } else {
        None
    };

    let mut has_next_page = false;
    let mut next_page: Option<u32> = None;

    if let Some(next_span) = document.select(&NEXT_PAGE_SPAN_SELECTOR).next() {
        if let Some(hx_get_url) = next_span.value().attr("hx-get") {
            if let Some(captures) = PAGE_NUMBER_REGEX.captures(hx_get_url) {
                if let Some(page_str) = captures.get(1) {
                    if let Ok(page_num) = page_str.as_str().parse::<u32>() {
                        has_next_page = true;
                        next_page = Some(page_num);
                    }
                }
            }
        }
    }

    let last_visible_page = next_page.unwrap_or(current_page);

    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    };

    Ok((data, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(list))
}
