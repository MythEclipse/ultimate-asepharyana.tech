// Standard library imports
use std::sync::Arc;

// External crate imports
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, attr_from_or, extract_slug, text, extract_parentheses};
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;

use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;


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
            let cached_posters = crate::services::images::cache::cache_image_urls_batch_lazy(
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
    let html = fetch_html_with_retry(url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    match tokio::task::spawn_blocking(move || parse_search_html(&html)).await {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
    }
}

fn parse_search_html(
    html: &str,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut anime_list = Vec::new();

    let item_selector = selector("#venkonten .chivsrc li").unwrap();
    let title_selector = selector("h2 a").unwrap();
    let img_selector = selector("img").unwrap();
    let link_selector = selector("a").unwrap();
    let genre_selector = selector(".set a").unwrap();
    let status_selector = selector(".set").unwrap();
    let next_selector = selector(".hpage .r").unwrap();

    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");
        let poster = attr_from_or(&element, &img_selector, "src", "");
        let anime_url = attr_from_or(&element, &link_selector, "href", "");
        let slug = extract_slug(&anime_url);

        let genres: Vec<String> = element
            .select(&genre_selector)
            .map(|e| text(&e))
            .collect();

        let status_text = text_from_or(&element, &status_selector, "");

        // Use extract_parentheses or manual regex if needed, but regex is removed from imports
        // Re-implementing simplified extraction or use helper if available.
        // The original code used regex capture group 1 inside parentheses.
        // Helper `extract_parentheses` does exactly that.
        let episode = extract_parentheses(&status_text).unwrap_or("N/A".to_string());

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

    let has_next_page = document.select(&next_selector).next().is_some();

    let pagination = Pagination {
        current_page: 1,
        last_visible_page: 1,
        has_next_page,
        next_page: if has_next_page {
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
    router
}