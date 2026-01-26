use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
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
pub const ENDPOINT_PATH: &str = "/api/anime2/genres";
pub const ENDPOINT_DESCRIPTION: &str = "Get list of all available anime2 genres";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_genres";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<GenresResponse>";

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

lazy_static! {
    // Genre labels are next to checkboxes in the dropdown
    static ref GENRE_LABEL_SELECTOR: Selector = Selector::parse("label[for^=\"genre-\"]").unwrap();
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
    // Fetch advanced search page to get genres
    let url = "https://alqanime.si/anime/";

    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching genres from: {}", url);
        match fetch_with_proxy(url).await {
            Ok(response) => {
                info!("Successfully fetched genres page");
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch genres: {:?}", e);
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

    // Parse genre labels from advanced search page
    for element in document.select(&GENRE_LABEL_SELECTOR) {
        let name = element.text().collect::<String>().trim().to_string();
        let for_attr = element.value().attr("for").unwrap_or("");

        // Extract slug from the 'for' attribute (format: "genre-{slug}")
        let slug = SLUG_REGEX
            .captures(for_attr)
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
    router.route(ENDPOINT_PATH, get(genres))
}