use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use reqwest;
use scraper::{Html, Selector};

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

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "The slug identifier")
    ),
    path = "/api/api/anime/full/{slug}",
    tag = "anime",
    operation_id = "anime_full_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/full/{slug} endpoint.", body = FullResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
    match fetch_anime_full(&slug).await {
        Ok(data) => Json(FullResponse {
            status: "Ok".to_string(),
            data,
        }),
        Err(_) => Json(FullResponse {
            status: "Error".to_string(),
            data: AnimeFullData {
                episode: "".to_string(),
                episode_number: "".to_string(),
                anime: AnimeInfo { slug: "".to_string() },
                has_next_episode: false,
                next_episode: None,
                has_previous_episode: false,
                previous_episode: None,
                stream_url: "".to_string(),
                download_urls: std::collections::HashMap::new(),
                image_url: "".to_string(),
            },
        }),
    }
}

async fn fetch_anime_full(slug: &str) -> Result<AnimeFullData, Box<dyn std::error::Error>> {
    let url = format!("https://otakudesu.cloud/episode/{}", slug);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let episode = document
        .select(&Selector::parse("h1.posttl").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let episode_number = episode
        .split("Episode")
        .nth(1)
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let image_url = document
        .select(&Selector::parse(".cukder img").unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or("")
        .to_string();

    let stream_url = document
        .select(&Selector::parse("#embed_holder iframe").unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or("")
        .to_string();

    let mut download_urls = std::collections::HashMap::new();

    for element in document.select(&Selector::parse(".download ul li").unwrap()) {
        let resolution = element
            .select(&Selector::parse("strong").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let mut links = Vec::new();
        for link_element in element.select(&Selector::parse("a").unwrap()) {
            let server = link_element.text().collect::<String>().trim().to_string();
            let url = link_element.value().attr("href").unwrap_or("").to_string();
            links.push(DownloadLink { server, url });
        }

        if !resolution.is_empty() && !links.is_empty() {
            download_urls.insert(resolution, links);
        }
    }

    let next_episode_element = document
        .select(&Selector::parse(".flir a[title*='Episode Selanjutnya']").unwrap())
        .next();

    let previous_episode_element = document
        .select(&Selector::parse(".flir a[title*='Episode Sebelumnya']").unwrap())
        .next();

    let next_episode_slug = next_episode_element
        .and_then(|e| e.value().attr("href"))
        .and_then(|href| href.split('/').nth(href.split('/').count().saturating_sub(2)))
        .map(|s| s.to_string());

    let previous_episode_slug = previous_episode_element
        .and_then(|e| e.value().attr("href"))
        .and_then(|href| href.split('/').nth(href.split('/').count().saturating_sub(2)))
        .map(|s| s.to_string());

    Ok(AnimeFullData {
        episode,
        episode_number,
        anime: AnimeInfo { slug: slug.to_string() },
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