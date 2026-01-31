use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, text, attr};
use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, Json, Router};

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
use utoipa::ToSchema;


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

            let (mut data, pagination) = fetch_and_parse_manhwa_list(&url, page)
                .await
                .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = data.iter().map(|i| i.poster.clone()).collect();
            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            for (i, item) in data.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

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
    let html_string = fetch_html_with_retry(url).await?;

    tokio::task::spawn_blocking(move || {
        parse_manhwa_list_document(&html_string, page)
    })
    .await?
}

fn parse_manhwa_list_document(
    html: &str,
    current_page: u32,
) -> Result<(Vec<ManhwaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut data = Vec::new();

    let animpost_selector = selector("div.bge, .listupd .bge").unwrap();
    let title_selector = selector(".kan h3, .kan a h3, .tt h3").unwrap();
    let img_selector = selector(".bgei img").unwrap();
    let _chapter_selector = selector(".new1 a span:last-child, .new1 span, .lch").unwrap();
    let date_selector = selector(".judul2, .kan span.judul2, .mdis .date").unwrap();
    let type_selector = selector(".tpe1_inf b, .tpe1_inf span.type, .mdis .type").unwrap();
    let link_selector = selector(".bgei a, .kan a").unwrap();
    let chapter_regex = Regex::new(r"\d+(\.\d+)?").unwrap();
    let next_page_span_selector = selector("body > span[hx-get]").unwrap();
    let page_number_regex = Regex::new(r"/page/(\d+)/").unwrap();

    for element in document.select(&animpost_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let mut poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| {
                attr(&e, "src")
                    .or_else(|| attr(&e, "data-src"))
                    .or_else(|| attr(&e, "data-lazy-src"))
                    .or_else(|| {
                        attr(&e, "srcset")
                            .and_then(|s| s.split_whitespace().next().map(|s| s.to_string()))
                    })
            })
            .unwrap_or_default();
        poster = poster.split('?').next().unwrap_or(&poster).to_string();

        let chapter = {
            let mut found_chapter = String::new();
            for chapter_element in element.select(&link_selector) {
                let text = text(&chapter_element);
                if text.contains("Chapter") {
                    let processed_text = text
                        .replace("Terbaru:", "")
                        .replace("Awal:", "")
                        .trim()
                        .to_string();
                    if let Some(captures) = chapter_regex.captures(&processed_text) {
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

        let full_date_string = text_from_or(&element, &date_selector, "");

        let parts: Vec<&str> = full_date_string.split(" • ").collect();
        let date = parts.get(1).unwrap_or(&"").to_string();
        let pembaca = parts.first().unwrap_or(&"").to_string();

        let r#type = text_from_or(&element, &type_selector, "");

        let slug = element
            .select(&link_selector)
            .next()
            .and_then(|e| attr(&e, "href"))
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

    if let Some(next_span) = document.select(&next_page_span_selector).next() {
        if let Some(hx_get_url) = attr(&next_span, "hx-get") {
            if let Some(captures) = page_number_regex.captures(&hx_get_url) {
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
    router
}