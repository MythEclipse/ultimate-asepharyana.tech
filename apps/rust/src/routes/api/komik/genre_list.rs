use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, attr_from};
use crate::routes::AppState;
use crate::scraping::urls::get_komik_api_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json, Router};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;


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
    let html = fetch_html_with_retry(&url).await?;

    tokio::task::spawn_blocking(move || parse_genres(&html))
        .await?
}

fn parse_genres(html: &str) -> Result<Vec<Genre>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut genres = Vec::new();

    let genre_selector = selector("#Genre .ls3, section#Genre .ls3, .ls3").unwrap();
    let genre_name_selector = selector(".ls3p h4, h4").unwrap();
    let genre_link_selector = selector("a[href*='/genre/']").unwrap();
    let slug_regex = Regex::new(r"/genre/([^/]+)").unwrap();

    // Each .ls3 contains a genre with image, name in h4, and link
    for element in document.select(&genre_selector) {
        // Get name from h4 inside ls3p
        let name = text_from_or(&element, &genre_name_selector, "");

        // Get link from a with /genre/ in href
        let href = attr_from(&element, &genre_link_selector, "href").unwrap_or_default();

        let slug = slug_regex
            .captures(&href)
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
    router
}