use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, attr_from_or, text, attr};
use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/genre/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Filter komik by genre with pagination";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_genre_filter";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<GenreKomikResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct KomikItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub chapter: String,
    pub score: String,
    pub r#type: String,
    pub komik_url: String,
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
pub struct GenreKomikResponse {
    pub status: String,
    pub genre: String,
    pub data: Vec<KomikItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct GenreQuery {
    pub page: Option<u32>,
}

const CACHE_TTL: u64 = 300;

#[utoipa::path(
    get,
    params(
        ("genre_slug" = String, Path, description = "Parameter for resource identification", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/genre/{slug}",
    tag = "komik",
    operation_id = "komik_genre_filter",
    responses(
        (status = 200, description = "Filter komik by genre with pagination", body = GenreKomikResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(genre_slug): Path<String>,
    Query(params): Query<GenreQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    info!("komik genre request: {}, page: {}", genre_slug, page);

    let cache_key = format!("komik:genre:{}:{}:v2", genre_slug, page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (mut komik_list, pagination) = fetch_genre_komik(&genre_slug, page)
                .await
                .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = komik_list.iter().map(|i| i.poster.clone()).collect();
            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            for (i, item) in komik_list.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            Ok(GenreKomikResponse {
                status: "Ok".to_string(),
                genre: genre_slug.clone(),
                data: komik_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_genre_komik(
    genre_slug: &str,
    page: u32,
) -> Result<(Vec<KomikItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = if page == 1 {
        format!("{}/genre/{}/", get_komik_api_url(), genre_slug)
    } else {
        format!(
            "{}/genre/{}/page/{}/",
            get_komik_api_url(),
            genre_slug,
            page
        )
    };

    let html = fetch_html_with_retry(&url).await?;
    let (komik_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((komik_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<KomikItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut komik_list = Vec::new();

    let item_selector = selector("article, .ls4, .ls2").unwrap();
    let title_selector = selector("h3 a, h4 a").unwrap();
    let img_selector = selector("img.lazy, img").unwrap();
    let chapter_selector = selector(".ls4s a, .ls24, .ls2l a, .new1 a").unwrap();
    let score_selector = selector(".up, .numscore, .epx").unwrap();
    let type_selector = selector(".ls3p, .type").unwrap();
    let link_selector = selector("h3 a, h4 a, a").unwrap();
    let pagination_selector = selector(".paging a, .pagination a:not(.next)").unwrap();
    let next_selector = selector(".paging a.next, .pagination .next").unwrap();
    let slug_regex = Regex::new(r"/([^/]+)/?$").unwrap();

    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| attr(&e, "data-src").or(attr(&e, "src")))
            .unwrap_or_else(|| "".to_string())
            .to_string();

        let chapter = text_from_or(&element, &chapter_selector, "N/A");

        let score = text_from_or(&element, &score_selector, "N/A");

        let komik_type = text_from_or(&element, &type_selector, "Unknown");

        let komik_url = attr_from_or(&element, &link_selector, "href", "");

        let slug = slug_regex
            .captures(&komik_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        if !title.is_empty() {
            komik_list.push(KomikItem {
                title,
                slug,
                poster,
                chapter,
                score,
                r#type: komik_type,
                komik_url,
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

    Ok((komik_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}