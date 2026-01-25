use crate::helpers::{default_backoff, internal_err, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::OTAKUDESU_BASE_URL;
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use backoff::future::retry;

use lazy_static::lazy_static;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/full/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/full/{slug} endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_full_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<FullResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeInfo {
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EpisodeInfo {
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DownloadLink {
    pub server: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeFullData {
    pub episode: String,
    pub episode_number: String,
    pub anime: AnimeInfo,
    pub has_next_episode: bool,
    pub next_episode: Option<EpisodeInfo>,
    pub has_previous_episode: bool,
    pub previous_episode: Option<EpisodeInfo>,
    pub stream_url: String,
    pub download_urls: std::collections::HashMap<String, Vec<DownloadLink>>,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct FullResponse {
    pub status: String,
    pub data: AnimeFullData,
}

lazy_static! {
    pub static ref EPISODE_TITLE_SELECTOR: Selector = Selector::parse("h1.posttl").unwrap();
    pub static ref IMAGE_SELECTOR: Selector = Selector::parse(".cukder img").unwrap();
    pub static ref STREAM_SELECTOR: Selector = Selector::parse("#embed_holder iframe").unwrap();
    pub static ref DOWNLOAD_ITEM_SELECTOR: Selector = Selector::parse(".download ul li").unwrap();
    pub static ref RESOLUTION_SELECTOR: Selector = Selector::parse("strong").unwrap();
    pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    pub static ref NEXT_EPISODE_SELECTOR: Selector =
        Selector::parse(".flir a[title*='Episode Selanjutnya']").unwrap();
    pub static ref PREVIOUS_EPISODE_SELECTOR: Selector =
        Selector::parse(".flir a[title*='Episode Sebelumnya']").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "naruto-shippuden-episode-1")
    ),
    path = "/api/anime/full/{slug}",
    tag = "anime",
    operation_id = "anime_full_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/full/{slug} endpoint.", body = FullResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _start = std::time::Instant::now();
    info!("Starting request for full slug: {}", slug);

    let cache_key = format!("anime:full:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let data = fetch_anime_full(slug.clone())
                .await
                .map_err(|e| e.to_string())?;
            Ok(FullResponse {
                status: "Ok".to_string(),
                data,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    return Ok(Json(response).into_response());
}

async fn fetch_anime_full(slug: String) -> Result<AnimeFullData, String> {
    let url = format!("{}/episode/{}", OTAKUDESU_BASE_URL, slug);

    let operation = || async {
        let response = fetch_with_proxy(&url).await?;
        Ok(response.data)
    };

    let backoff = default_backoff();
    let html = retry(backoff, operation)
        .await
        .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;

    match tokio::task::spawn_blocking(move || {
        let document = Html::parse_document(&html);
        parse_anime_full_document(&document, &slug)
    })
    .await
    {
        Ok(inner_result) => inner_result.map_err(|e| e.to_string()),
        Err(join_err) => Err(format!("Failed to spawn blocking task: {}", join_err)),
    }
}

fn parse_anime_full_document(
    document: &Html,
    slug: &str,
) -> Result<AnimeFullData, Box<dyn std::error::Error + Send + Sync>> {
    let episode = document
        .select(&EPISODE_TITLE_SELECTOR)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let episode_number = episode
        .split("Episode")
        .nth(1)
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let image_url = document
        .select(&IMAGE_SELECTOR)
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or("")
        .to_string();

    let stream_url = document
        .select(&STREAM_SELECTOR)
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or("")
        .to_string();

    let mut download_urls = std::collections::HashMap::new();

    for element in document.select(&DOWNLOAD_ITEM_SELECTOR) {
        let resolution = element
            .select(&RESOLUTION_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let mut links = Vec::new();
        for link_element in element.select(&LINK_SELECTOR) {
            let server = link_element.text().collect::<String>().trim().to_string();
            let url = link_element.value().attr("href").unwrap_or("").to_string();
            links.push(DownloadLink { server, url });
        }

        if !resolution.is_empty() && !links.is_empty() {
            download_urls.insert(resolution, links);
        }
    }

    let next_episode_element = document.select(&NEXT_EPISODE_SELECTOR).next();

    let previous_episode_element = document.select(&PREVIOUS_EPISODE_SELECTOR).next();

    let next_episode_slug = next_episode_element
        .and_then(|e| e.value().attr("href"))
        .and_then(|href| {
            href.split('/')
                .nth(href.split('/').count().saturating_sub(2))
        })
        .map(|s| s.to_string() + "/");

    let previous_episode_slug = previous_episode_element
        .and_then(|e| e.value().attr("href"))
        .and_then(|href| {
            href.split('/')
                .nth(href.split('/').count().saturating_sub(2))
        })
        .map(|s| s.to_string() + "/");

    Ok(AnimeFullData {
        episode,
        episode_number,
        anime: AnimeInfo {
            slug: slug.to_string(),
        },
        has_next_episode: next_episode_slug.is_some(),
        next_episode: next_episode_slug.map(|s| EpisodeInfo { slug: s }),
        has_previous_episode: previous_episode_slug.is_some(),
        previous_episode: previous_episode_slug.map(|s| EpisodeInfo { slug: s }),
        stream_url,
        download_urls,
        image_url,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}
