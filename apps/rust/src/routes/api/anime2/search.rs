use crate::helpers::{default_backoff, get_cached_or_original, internal_err, parse_html, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;
use lazy_static::lazy_static;
use scraper::Selector;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/search";
pub const ENDPOINT_DESCRIPTION: &str = "Searches for anime2 based on query parameters.";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_search";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub description: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub rating: String,
    pub r#type: String,
    pub season: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<String>,
    pub has_previous_page: bool,
    pub previous_page: Option<String>,
}

lazy_static! {
    static ref ITEM_SELECTOR: Selector = Selector::parse(".listupd .bs").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".ntitle").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref DESC_SELECTOR: Selector = Selector::parse(".data .typez").unwrap();
    static ref GENRE_SELECTOR: Selector = Selector::parse(".genres a").unwrap();
    static ref RATING_SELECTOR: Selector = Selector::parse(".score").unwrap();
    static ref TYPE_SELECTOR: Selector = Selector::parse(".typez").unwrap();
    static ref SEASON_SELECTOR: Selector = Selector::parse(".season").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

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

#[utoipa::path(
    get,
    params(
        ("q" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value")
    ),
    path = "/api/anime2/search",
    tag = "anime2",
    operation_id = "anime2_search",
    responses(
        (status = 200, description = "Searches for anime2 based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query = params.q.unwrap_or_else(|| "one".to_string());
    info!("Starting search for query: {}", query);

    let cache_key = format!("anime2:search:{}", query);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let url = format!("https://alqanime.net/?s={}", urlencoding::encode(&query));
            let (mut data, pagination) = fetch_and_parse_search(&url)
                .await
                .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs (returns original + background cache)
            for item in &mut data {
                if !item.poster.is_empty() {
                    item.poster = get_cached_or_original(&app_state.db, &app_state.redis_pool, &item.poster).await;
                }
            }

            Ok(SearchResponse {
                status: "Ok".to_string(),
                data,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_and_parse_search(
    url: &str,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let html = fetch_html_with_retry(url).await?;
    let (data, pagination) = tokio::task::spawn_blocking(move || {
        let document = parse_html(&html);
        parse_search_document(&document)
    })
    .await??;

    Ok((data, pagination))
}

async fn fetch_html_with_retry(
    url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(url).await {
            Ok(response) => {
                info!("Successfully fetched URL: {}", url);
                Ok(response.data)
            }
            Err(e) => Err(transient(e)),
        }
    };

    let html = retry(backoff, fetch_operation).await?;
    Ok(html)
}

fn parse_search_document(
    document: &scraper::Html,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let mut data = Vec::new();

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
            .and_then(|href| href.split('/').nth(3))
            .unwrap_or("")
            .to_string();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let description = element
            .select(&DESC_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let genres = element
            .select(&GENRE_SELECTOR)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect();

        let rating = element
            .select(&RATING_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let r#type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let season = element
            .select(&SEASON_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        if !title.is_empty() {
            data.push(AnimeItem {
                title,
                slug,
                poster,
                description,
                anime_url,
                genres,
                rating,
                r#type,
                season,
            });
        }
    }

    // Pagination logic
    let current_page = document
        .select(&PAGINATION_SELECTOR)
        .find(|e| e.value().attr("class").unwrap_or("").contains("current"))
        .and_then(|e| e.text().collect::<String>().trim().parse().ok())
        .unwrap_or(1);

    let last_visible_page = document
        .select(&PAGINATION_SELECTOR)
        .last()
        .and_then(|e| e.text().collect::<String>().trim().parse().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();

    let next_page = if has_next_page {
        document
            .select(&NEXT_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split("/page/").nth(1))
            .and_then(|s| s.split('/').next())
            .map(|s| s.to_string())
    } else {
        None
    };

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some((current_page - 1).to_string())
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