use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
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

lazy_static! {
    static ref ITEM_SELECTOR: Selector = Selector::parse("article, .ls4, .ls2").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse("h3 a, h4 a").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img.lazy, img").unwrap();
    static ref CHAPTER_SELECTOR: Selector =
        Selector::parse(".ls4s a, .ls24, .ls2l a, .new1 a").unwrap();
    static ref SCORE_SELECTOR: Selector = Selector::parse(".up, .numscore, .epx").unwrap();
    static ref TYPE_SELECTOR: Selector = Selector::parse(".ls3p, .type").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("h3 a, h4 a, a").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".paging a, .pagination a:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector =
        Selector::parse(".paging a.next, .pagination .next").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

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
            let (komik_list, pagination) = fetch_genre_komik(&genre_slug, page)
                .await
                .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let posters: Vec<String> = komik_list.iter().map(|i| i.poster.clone()).collect();
            crate::helpers::image_cache::cache_image_urls_batch_lazy(&db, &redis, posters);

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

    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => Ok(response.data),
            Err(e) => {
                warn!("Failed: {:?}", e);
                Err(transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;
    let (komik_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((komik_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<KomikItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut komik_list = Vec::new();

    for element in document.select(&ITEM_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("data-src").or(e.value().attr("src")))
            .unwrap_or("")
            .to_string();

        let chapter = element
            .select(&CHAPTER_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let score = element
            .select(&SCORE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let komik_type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("Unknown".to_string());

        let komik_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let slug = SLUG_REGEX
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
