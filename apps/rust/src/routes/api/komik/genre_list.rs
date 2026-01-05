use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
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
pub const ENDPOINT_PATH: &str = "/api/komik/genres";
pub const ENDPOINT_DESCRIPTION: &str = "Get list of all available komik genres";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_genres";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<GenresResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub count: Option<u32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GenresResponse {
    pub status: String,
    pub data: Vec<Genre>,
}

lazy_static! {
    // Komiku genres are in ls3 sections with ls3p content
    static ref GENRE_SELECTOR: Selector =
        Selector::parse("#Genre .ls3, section#Genre .ls3, .ls3").unwrap();
    static ref GENRE_NAME_SELECTOR: Selector = Selector::parse(".ls3p h4, h4").unwrap();
    static ref GENRE_LINK_SELECTOR: Selector = Selector::parse("a[href*='/genre/']").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/genre/([^/]+)").unwrap());

const CACHE_TTL: u64 = 3600;

#[utoipa::path(
    get,
    path = "/api/komik/genres",
    tag = "komik",
    operation_id = "komik_genres",
    responses(
        (status = 200, description = "Get list of all available komik genres", body = GenresResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn genres(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Handling request for komik genres");

    let cache_key = "komik:genres:list:v3";
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(cache_key, CACHE_TTL, || async {
            let genres = fetch_genres().await.map_err(|e| e.to_string())?;
            Ok(GenresResponse {
                status: "Ok".to_string(),
                data: genres,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_genres() -> Result<Vec<Genre>, Box<dyn std::error::Error + Send + Sync>> {
    // Try homepage which typically lists genres in sidebar
    let url = get_komik_api_url();

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
    let genres = tokio::task::spawn_blocking(move || parse_genres(&html)).await??;
    Ok(genres)
}

fn parse_genres(html: &str) -> Result<Vec<Genre>, Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut genres = Vec::new();

    // Each .ls3 contains a genre with image, name in h4, and link
    for element in document.select(&GENRE_SELECTOR) {
        // Get name from h4 inside ls3p
        let name = element
            .select(&GENRE_NAME_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        // Get link from a with /genre/ in href
        let href = element
            .select(&GENRE_LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("");

        let slug = SLUG_REGEX
            .captures(href)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        if !name.is_empty() && !slug.is_empty() {
            genres.push(Genre {
                name,
                slug,
                count: None,
            });
        }
    }

    info!("Parsed {} genres", genres.len());
    Ok(genres)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(genres))
}