use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text, attr};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json, Router};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GenresResponse {
    pub status: String,
    pub data: Vec<Genre>,
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"genre-(.+)$").unwrap());

const CACHE_TTL: u64 = 3600; // 1 hour

#[utoipa::path(
    get,
    path = "/api/anime2/genres",
    tag = "anime2",
    operation_id = "anime2_genres",
    responses(
        (status = 200, description = "Get list of all available anime2 genres", body = GenresResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn genres(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Handling request for anime2 genres");

    let cache_key = "anime2:genres:list:v3";
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
    let url = "https://alqanime.si/anime/";

    let html = fetch_html_with_retry(url).await?;

    let genres = tokio::task::spawn_blocking(move || parse_genres(&html)).await??;

    Ok(genres)
}

fn parse_genres(html: &str) -> Result<Vec<Genre>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut genres = Vec::new();
    let genre_label_selector = selector("label[for^=\"genre-\"]").unwrap();

    // Parse genre labels from advanced search page
    for element in document.select(&genre_label_selector) {
        let name = text(&element).trim().to_string();
        let for_attr = attr(&element, "for").unwrap_or_default();

        // Extract slug from the 'for' attribute (format: "genre-{slug}")
        let slug = SLUG_REGEX
            .captures(&for_attr)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        if !name.is_empty() && !slug.is_empty() {
            genres.push(Genre { name, slug });
        }
    }

    info!("Parsed {} genres", genres.len());
    Ok(genres)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}