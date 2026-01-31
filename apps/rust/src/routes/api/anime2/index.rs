use crate::helpers::{internal_err, Cache, fetch_html_with_retry};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

// Import shared models and parsers
use crate::models::anime2::{OngoingAnimeItem, CompleteAnimeItem};
use crate::scraping::anime2 as parsers;
use crate::helpers::anime2_cache as cache_utils;


#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Anime2Data {
    pub ongoing_anime: Vec<OngoingAnimeItem>,
    pub complete_anime: Vec<CompleteAnimeItem>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Anime2Response {
    pub status: String,
    pub data: Anime2Data,
}

const CACHE_KEY: &str = "anime2:index";
const CACHE_TTL: u64 = 300;

#[utoipa::path(
    get,
    path = "/api/anime2",
    tag = "anime2",
    operation_id = "anime2_index",
    responses(
        (status = 200, description = "Handles GET requests for the anime2 endpoint.", body = Anime2Response),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn anime2(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Handling request for anime2 index");

    // Use Cache helper for get_or_set pattern
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(CACHE_KEY, CACHE_TTL, || async {
            let mut data = fetch_anime_data().await.map_err(|e| e.to_string())?;

            // Use shared cache utility for batch poster caching
            let ongoing_posters: Vec<String> = data
                .ongoing_anime
                .iter()
                .map(|i| i.poster.clone())
                .collect();
            let complete_posters: Vec<String> = data
                .complete_anime
                .iter()
                .map(|i| i.poster.clone())
                .collect();

            let ongoing_len = ongoing_posters.len();
            let cached_posters: Vec<String> = cache_utils::cache_multiple_collections(
                &app_state,
                vec![ongoing_posters, complete_posters],
            )
            .await;

            // Update ongoing posters
            for (i, item) in data.ongoing_anime.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            // Update complete posters
            for (i, item) in data.complete_anime.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(ongoing_len + i) {
                    item.poster = url.clone();
                }
            }

            Ok(Anime2Response {
                status: "Ok".to_string(),
                data,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response))
}

async fn fetch_anime_data() -> Result<Anime2Data, Box<dyn std::error::Error + Send + Sync>> {
    let ongoing_url = "https://alqanime.si/anime/?status=ongoing&type=&order=update";
    let complete_url = "https://alqanime.si/anime/?status=completed&type=&order=update";

    let (ongoing_html, complete_html) = tokio::join!(
        fetch_html_with_retry(ongoing_url),
        fetch_html_with_retry(complete_url)
    );

    let ongoing_html = ongoing_html?;
    let complete_html = complete_html?;

    let ongoing_anime =
        tokio::task::spawn_blocking(move || parsers::parse_ongoing_anime(&ongoing_html)).await??;
    let complete_anime =
        tokio::task::spawn_blocking(move || parsers::parse_complete_anime(&complete_html)).await??;

    Ok(Anime2Data {
        ongoing_anime,
        complete_anime,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}