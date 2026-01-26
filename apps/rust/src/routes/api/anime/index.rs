use crate::helpers::{default_backoff, internal_err, parse_html, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;
use lazy_static::lazy_static;
use scraper::Selector;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_index";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<AnimeResponse>";

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
pub struct AnimeData {
    pub ongoing_anime: Vec<OngoingAnimeItem>,
    pub complete_anime: Vec<CompleteAnimeItem>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeResponse {
    pub status: String,
    pub data: AnimeData,
}

lazy_static! {
    pub static ref VENZ_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
    pub static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    pub static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    pub static ref EPISODE_SELECTOR: Selector = Selector::parse(".epz").unwrap();
}
use crate::helpers::cache_ttl::CACHE_TTL_VERY_SHORT;
const CACHE_TTL: u64 = CACHE_TTL_VERY_SHORT; // 5 minutes

#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "anime",
    operation_id = "anime_index",
    responses(
        (status = 200, description = "Handles GET requests for the anime endpoint.", body = AnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn anime(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let start_time = std::time::Instant::now();
    info!("Handling request for anime index");

    let cache = Cache::new(&app_state.redis_pool);

    // Clean caching with get_or_set pattern
    let response = cache
        .get_or_set("anime:index", CACHE_TTL, || async {
            let mut data = fetch_anime_data()
                .await
                .map_err(|e| format!("Fetch error: {}", e))?;

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
                db.clone(),
                &redis,
                all_posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // Update ongoing anime posters
            for (i, item) in data.ongoing_anime.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            // Update complete anime posters
            for (i, item) in data.complete_anime.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(ongoing_len + i) {
                    item.poster = url.clone();
                }
            }

            Ok(AnimeResponse {
                status: "Ok".to_string(),
                data,
            })
        })
        .await
        .map_err(internal_err)?;

    info!("Anime index completed in {:?}", start_time.elapsed());
    Ok(Json(response).into_response())
}

async fn fetch_anime_data() -> Result<AnimeData, Box<dyn std::error::Error + Send + Sync>> {
    let ongoing_url = format!("{}/ongoing-anime/", get_otakudesu_url());
    let complete_url = format!("{}/complete-anime/", get_otakudesu_url());

    let (ongoing_html, complete_html) = tokio::join!(
        fetch_html_with_retry(&ongoing_url),
        fetch_html_with_retry(&complete_url)
    );

    let ongoing_html = ongoing_html?;
    let complete_html = complete_html?;

    let ongoing_anime =
        tokio::task::spawn_blocking(move || parse_ongoing_anime(&ongoing_html)).await??;
    let complete_anime =
        tokio::task::spawn_blocking(move || parse_complete_anime(&complete_html)).await??;

    Ok(AnimeData {
        ongoing_anime,
        complete_anime,
    })
}

async fn fetch_html_with_retry(
    url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let backoff = default_backoff(); // Use helper instead of manual config

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(url).await {
            Ok(response) => {
                info!("Successfully fetched URL: {}", url);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch URL: {}, error: {:?}", url, e);
                Err(transient(e)) // Use helper
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;
    Ok(html)
}

fn parse_ongoing_anime(
    html: &str,
) -> Result<Vec<OngoingAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut ongoing_anime = Vec::new();

    for element in document.select(&VENZ_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let slug = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let current_episode = element
            .select(&EPISODE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

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

    for element in document.select(&VENZ_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let slug = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let episode_count = element
            .select(&EPISODE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

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
    router.route(ENDPOINT_PATH, get(anime))
}