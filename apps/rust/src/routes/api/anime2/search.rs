use crate::helpers::{
    internal_err, parse_html, Cache, fetch_html_with_retry,
};
use crate::helpers::scraping::{selector, text_from_or, attr_from_or, extract_slug, attr_from, text, attr};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};

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
            let url = format!("https://alqanime.si/?s={}", urlencoding::encode(&query));
            let (data, pagination) = fetch_and_parse_search(&url)
                .await
                .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs concurrently
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();
            
            // Extract posters
            let posters: Vec<String> = data.iter().map(|item| item.poster.clone()).collect();
            
            // Trigger lazy batch caching
            crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db.clone(),
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // We return original data for speed on cold start

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
        parse_search_document(&html)
    })
    .await??;

    Ok((data, pagination))
}

fn parse_search_document(
    html: &str,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut data = Vec::new();

    let item_selector = selector("article.bs").unwrap();
    let title_selector = selector(".tt h2").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let desc_selector = selector(".data .typez").unwrap();
    let genre_selector = selector(".genres a").unwrap();
    let rating_selector = selector(".score").unwrap();
    let type_selector = selector(".typez").unwrap();
    let season_selector = selector(".season").unwrap();
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagination .next").unwrap();

    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let href = attr_from(&element, &link_selector, "href").unwrap_or_default();
        let slug = extract_slug(&href);
            
        let poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| attr(&e, "src").or(attr(&e, "data-src")))
            .unwrap_or_default();

        let description = text_from_or(&element, &desc_selector, "");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        let genres = element
            .select(&genre_selector)
            .map(|e| text(&e))
            .collect();

        let rating = text_from_or(&element, &rating_selector, "");

        let r#type = text_from_or(&element, &type_selector, "");

        let season = text_from_or(&element, &season_selector, "");

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
        .select(&pagination_selector)
        .find(|e| attr(&e, "class").unwrap_or_default().contains("current"))
        .and_then(|e| text(&e).trim().parse().ok())
        .unwrap_or(1);

    let last_visible_page = document
        .select(&pagination_selector)
        .last()
        .and_then(|e| text(&e).trim().parse().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&next_selector).next().is_some();

    let next_page = if has_next_page {
        document
            .select(&next_selector)
            .next()
            .and_then(|e| attr(&e, "href"))
            .and_then(|href| {
                 href.split("/page/").nth(1).map(|s| s.to_string())
            })
            .and_then(|s| s.split('/').next().map(|s| s.to_string()))
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