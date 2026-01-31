use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, attr_from_or, extract_slug, text, attr};
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/genre/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Filter anime2 by genre with advanced options";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_genre_filter";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<GenreAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub score: String,
    pub status: String,
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
pub struct GenreAnimeResponse {
    pub status: String,
    pub genre: String,
    pub data: Vec<AnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct GenreQuery {
    pub page: Option<u32>,
    pub status: Option<String>,
    pub order: Option<String>,
}

const CACHE_TTL: u64 = 300;

#[utoipa::path(
    get,
    params(
        ("genre_slug" = String, Path, description = "Parameter for resource identification", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1),
        ("status" = Option<String>, Query, description = "Status filter (active, inactive, pending, etc.)", example = "sample_value"),
        ("order" = Option<String>, Query, description = "Sort direction (ascending or descending)", example = "sample_value")
    ),
    path = "/api/anime2/genre/{slug}",
    tag = "anime2",
    operation_id = "anime2_genre_filter",
    responses(
        (status = 200, description = "Filter anime2 by genre with advanced options", body = GenreAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(genre_slug): Path<String>,
    Query(params): Query<GenreQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let status = params.status.clone().unwrap_or_default();
    let order = params.order.clone().unwrap_or("update".to_string());

    info!(
        "anime2 genre request: {}, page: {}, status: {}, order: {}",
        genre_slug, page, status, order
    );

    let cache_key = format!("anime2:genre:{}:{}:{}:{}", genre_slug, page, status, order);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (anime_list, pagination) =
                fetch_genre_anime(&genre_slug, page, &status, &order)
                    .await
                    .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = anime_list.iter().map(|i| i.poster.clone()).collect();
            crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // Return original data explicitly for speed
            Ok(GenreAnimeResponse {
                status: "Ok".to_string(),
                genre: genre_slug.clone(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_genre_anime(
    genre_slug: &str,
    page: u32,
    status: &str,
    order: &str,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let mut url = if page > 1 {
        format!(
            "https://alqanime.si/anime/page/{}/?genre[]={}",
            page, genre_slug
        )
    } else {
        format!("https://alqanime.si/anime/?genre[]={}", genre_slug)
    };

    if !status.is_empty() {
        url.push_str(&format!("&status={}", status));
    }
    url.push_str(&format!("&order={}", order));

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut anime_list = Vec::new();

    let item_selector = selector("article.bs").unwrap();
    let title_selector = selector(".tt h2").unwrap();
    let img_selector = selector("img").unwrap();
    let score_selector = selector(".numscore").unwrap();
    let status_selector = selector(".status").unwrap();
    let link_selector = selector("a").unwrap();
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagination .next").unwrap();
    
    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| attr(&e, "src").or(attr(&e, "data-src")))
            .unwrap_or_default();

        let score = text_from_or(&element, &score_selector, "N/A");

        let status = text_from_or(&element, &status_selector, "Unknown");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        let slug = extract_slug(&anime_url);

        if !title.is_empty() {
            anime_list.push(AnimeItem {
                title,
                slug,
                poster,
                score,
                status,
                anime_url,
            });
        }
    }

    let last_visible_page = document
        .select(&pagination_selector)
        .next_back()
        .map(|e| {
            text(&e)
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        })
        .unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();
    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page: if has_next_page {
            Some(current_page + 1)
        } else {
            None
        },
        has_previous_page: current_page > 1,
        previous_page: if current_page > 1 {
            Some(current_page - 1)
        } else {
            None
        },
    };

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}