use crate::helpers::{internal_err, parse_html, Cache, fetch_html_with_retry, text_from_or, attr_from, attr_from_or};

use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};

use lazy_static::lazy_static;
use regex::Regex;
use scraper::Selector;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
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
    pub query: Option<String>,
    pub page: Option<u32>,
}

use axum::extract::State;

lazy_static! {
    static ref ANIMPOST_SELECTOR: Selector = Selector::parse("div.bge, .listupd .bge").unwrap();
    static ref TITLE_SELECTOR: Selector =
        Selector::parse("div.kan h3, div.kan a h3, .tt h3").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("div.bgei img").unwrap();
    static ref CHAPTER_SELECTOR: Selector =
        Selector::parse("div.new1 a span:last-child, .new1 span, .lch").unwrap();
    static ref SCORE_SELECTOR: Selector = Selector::parse(".up, .epx, .numscore").unwrap();
    static ref DATE_SELECTOR: Selector =
        Selector::parse("div.kan span.judul2, .mdis .date").unwrap();
    static ref TYPE_SELECTOR: Selector =
        Selector::parse("div.tpe1_inf b, .tpe1_inf span.type, .mdis .type").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("div.bgei a, div.kan a").unwrap();
    static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
    static ref CURRENT_SELECTOR: Selector = Selector::parse(
        ".pagination > .current, .pagination > span.page-numbers.current, .hpage .current"
    )
    .unwrap();
    static ref PAGE_SELECTORS: Selector = Selector::parse(
        ".pagination > a, .pagination > .page-numbers:not(.next):not(.prev), .hpage a"
    )
    .unwrap();
    static ref NEXT_SELECTOR: Selector =
        Selector::parse(".pagination > a.next, .pagination > .next.page-numbers, .hpage .next")
            .unwrap();
    static ref PREV_SELECTOR: Selector =
        Selector::parse(".pagination > a.prev, .pagination > .prev.page-numbers, .hpage .prev")
            .unwrap();
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
    let query = params.query.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    info!(
        "Starting komik search for query: '{}', page: {}",
        query, page
    );

    let cache_key = format!("komik:search:{}:{}", query, page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let base_url = get_komik_api_url();
            let url = if page == 1 {
                format!(
                    "{}/?post_type=manga&s={}",
                    base_url,
                    urlencoding::encode(&query)
                )
            } else {
                format!(
                    "{}/page/{}/?post_type=manga&s={}",
                    base_url,
                    page,
                    urlencoding::encode(&query)
                )
            };
            let (mut data, pagination) = fetch_and_parse_search(&url, page)
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

            Ok(SearchResponse { data, pagination })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_and_parse_search(
    url: &str,
    page: u32,
) -> Result<(Vec<MangaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let html = fetch_html_with_retry(url).await?;
    let (data, pagination) = tokio::task::spawn_blocking(move || {
        let document = parse_html(&html);
        parse_search_document(&document, page)
    })
    .await??;

    Ok((data, pagination))
}



fn parse_search_document(
    document: &scraper::Html,
    current_page: u32,
) -> Result<(Vec<MangaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let mut data = Vec::new();

    for element in document.select(&ANIMPOST_SELECTOR) {
        let title = text_from_or(&element, &TITLE_SELECTOR, "");

        let poster = attr_from_or(&element, &IMG_SELECTOR, "src", "");

        let chapter = text_from_or(&element, &CHAPTER_SELECTOR, "N/A");

        let score = text_from_or(&element, &SCORE_SELECTOR, "N/A");

        let date = text_from_or(&element, &DATE_SELECTOR, "N/A");

        let r#type = text_from_or(&element, &TYPE_SELECTOR, "");

        let slug = attr_from(&element, &LINK_SELECTOR, "href")
            .and_then(|href| href.split('/').nth(3).map(String::from))
            .unwrap_or_default();

        if !title.is_empty() {
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
    }

    // Pagination logic
    let last_visible_page = document
        .select(&PAGE_SELECTORS)
        .last()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
    let next_page = if has_next_page {
        Some(current_page + 1)
    } else {
        None
    };

    let has_previous_page = document.select(&PREV_SELECTOR).next().is_some();
    let previous_page = if has_previous_page {
        if current_page > 1 {
            Some(current_page - 1)
        } else {
            None
        }
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

    Ok((data, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}