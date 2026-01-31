use crate::helpers::{internal_err, Cache, fetch_html_with_retry};
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/genres";
pub const ENDPOINT_DESCRIPTION: &str = "Get list of all available anime genres";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_genres";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<GenresResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GenresResponse {
    pub status: String,
    pub data: Vec<Genre>,
}

lazy_static! {
    static ref GENRE_SELECTOR: Selector = Selector::parse(".genres li a, .genre-list a").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 3600; // 1 hour - genres don't change often

#[utoipa::path(
    get,
    path = "/api/anime/genres",
    tag = "anime",
    operation_id = "anime_genres",
    responses(
        (status = 200, description = "Get list of all available anime genres", body = GenresResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn genres(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Handling request for anime genres");

    let cache_key = "anime:genres:list";
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
    let url = format!("{}/genre-list/", get_otakudesu_url());

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let genres = tokio::task::spawn_blocking(move || parse_genres(&html)).await??;

    Ok(genres)
}

fn parse_genres(html: &str) -> Result<Vec<Genre>, Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut genres = Vec::new();

    for element in document.select(&GENRE_SELECTOR) {
        let name = element.text().collect::<String>().trim().to_string();
        let url = element.value().attr("href").unwrap_or("").to_string();

        let slug = SLUG_REGEX
            .captures(&url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        if !name.is_empty() && !slug.is_empty() {
            genres.push(Genre { name, slug, url });
        }
    }

    info!("Parsed {} genres", genres.len());
    Ok(genres)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(genres))
}