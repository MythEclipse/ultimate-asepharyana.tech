use crate::helpers::{internal_err, parse_html, Cache, fetch_html_with_retry, text_from_or, attr_from_or, selector, extract_slug};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime2 endpoint.";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_index";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<Anime2Response>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompleteAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

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

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

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

            let all_posters = [ongoing_posters, complete_posters].concat();
            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db,
                &redis,
                all_posters,
                Some(app_state.image_processing_semaphore.clone()),
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
        tokio::task::spawn_blocking(move || parse_ongoing_anime(&ongoing_html)).await??;
    let complete_anime =
        tokio::task::spawn_blocking(move || parse_complete_anime(&complete_html)).await??;

    Ok(Anime2Data {
        ongoing_anime,
        complete_anime,
    })
}

fn parse_ongoing_anime(
    html: &str,
) -> Result<Vec<OngoingAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut ongoing_anime = Vec::new();

    let item_selector = selector("article.bs").unwrap();
    let title_selector = selector(".tt h2").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epx").unwrap();

    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let href = attr_from_or(&element, &link_selector, "href", "");
        let slug = extract_slug(&href);

        let poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let current_episode = text_from_or(&element, &episode_selector, "N/A");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        if !title.is_empty() {
            ongoing_anime.push(OngoingAnimeItem {
                title,
                slug,
                poster,
                current_episode,
                anime_url,
            });
        }
    }
    Ok(ongoing_anime)
}

fn parse_complete_anime(
    html: &str,
) -> Result<Vec<CompleteAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut complete_anime = Vec::new();

    let item_selector = selector("article.bs").unwrap();
    let title_selector = selector(".tt h2").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epx").unwrap();

    for element in document.select(&item_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let href = attr_from_or(&element, &link_selector, "href", "");
        let slug = extract_slug(&href);

        let poster = element
            .select(&img_selector)
            .next()
            .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let episode_count = text_from_or(&element, &episode_selector, "N/A");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        if !title.is_empty() {
            complete_anime.push(CompleteAnimeItem {
                title,
                slug,
                poster,
                episode_count,
                anime_url,
            });
        }
    }
    Ok(complete_anime)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(anime2))
}