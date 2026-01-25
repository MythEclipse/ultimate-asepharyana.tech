use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/genre/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Filter anime by genre with pagination";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_genre_filter";
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
}

lazy_static! {
    static ref VENZ_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref EP_SELECTOR: Selector = Selector::parse(".epz").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("genre_slug" = String, Path, description = "Parameter for resource identification", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/anime/genre/{slug}",
    tag = "anime",
    operation_id = "anime_genre_filter",
    responses(
        (status = 200, description = "Filter anime by genre with pagination", body = GenreAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(genre_slug): Path<String>,
    Query(params): Query<GenreQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    info!("Handling request for genre: {}, page: {}", genre_slug, page);

    let cache_key = format!("anime:genre:{}:{}", genre_slug, page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (anime_list, pagination) = fetch_genre_anime(&genre_slug, page)
                .await
                .map_err(|e| e.to_string())?;

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
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = if page == 1 {
        format!("{}/genres/{}/", get_otakudesu_url(), genre_slug)
    } else {
        format!(
            "{}/genres/{}/page/{}/",
            get_otakudesu_url(),
            genre_slug,
            page
        )
    };

    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching genre page: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => {
                info!("Successfully fetched genre page");
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch genre page: {:?}", e);
                Err(transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut anime_list = Vec::new();

    for element in document.select(&VENZ_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let score = element
            .select(&EP_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let slug = SLUG_REGEX
            .captures(&anime_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        // Try to determine status from score text
        let status_text = score.to_lowercase();
        let status = if status_text.contains("ongoing") {
            "Ongoing".to_string()
        } else if status_text.contains("completed") || status_text.contains("eps") {
            "Completed".to_string()
        } else {
            "Unknown".to_string()
        };

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
        .select(&PAGINATION_SELECTOR)
        .next_back()
        .map(|e| {
            e.text()
                .collect::<String>()
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        })
        .unwrap_or(1);

    let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
    let next_page = if has_next_page {
        Some(current_page + 1)
    } else {
        None
    };

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some(current_page - 1)
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

    info!("Parsed {} anime items", anime_list.len());
    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}
