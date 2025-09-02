// Handler for GET /api/anime/full/{slug}.
// Fetches and parses the anime episode page from otakudesu.cloud using reqwest and scraper.

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AnimeInfo {
    slug: String,
}

#[derive(Serialize)]
pub struct EpisodeInfo {
    slug: String,
}

#[derive(Serialize)]
pub struct AnimeData {
    episode: String,
    episode_number: String,
    anime: AnimeInfo,
    has_next_episode: bool,
    next_episode: Option<EpisodeInfo>,
    has_previous_episode: bool,
    previous_episode: Option<EpisodeInfo>,
    stream_url: String,
    download_urls: HashMap<String, Vec<DownloadLink>>,
    image_url: String,
}

#[derive(Serialize)]
pub struct DownloadLink {
    server: String,
    url: String,
}

#[derive(Serialize)]
pub struct AnimeResponse {
    status: &'static str,
    data: AnimeData,
}

pub async fn handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!("https://otakudesu.cloud/episode/{}/", slug);

    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "message": "Failed to fetch episode page",
                        "error": e.to_string()
                    })),
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch episode page",
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let document = Html::parse_document(&html);

    // Episode title and number
    let episode = {
        let sel = Selector::parse("h1.posttl").unwrap();
        document
            .select(&sel)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    };
    let episode_number = episode
        .split_whitespace()
        .last()
        .and_then(|s| s.parse::<u32>().ok().map(|n| n.to_string()))
        .unwrap_or_default();

    // Image URL
    let image_url = {
        let sel = Selector::parse(".cukder img").unwrap();
        document
            .select(&sel)
            .next()
            .and_then(|n| n.value().attr("src"))
            .unwrap_or("")
            .to_string()
    };

    // Stream URL
    let stream_url = {
        let sel = Selector::parse("#embed_holder iframe").unwrap();
        document
            .select(&sel)
            .next()
            .and_then(|n| n.value().attr("src"))
            .unwrap_or("")
            .to_string()
    };

    // Download URLs
    let mut download_urls: HashMap<String, Vec<DownloadLink>> = HashMap::new();
    if let Ok(ul_sel) = Selector::parse(".download ul li") {
        for li in document.select(&ul_sel) {
            let resolution = li
                .select(&Selector::parse("strong").unwrap())
                .next()
                .map(|n| n.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            let mut links = Vec::new();
            for a in li.select(&Selector::parse("a").unwrap()) {
                let server = a.text().collect::<String>().trim().to_string();
                let url = a.value().attr("href").unwrap_or("").to_string();
                links.push(DownloadLink { server, url });
            }
            if !resolution.is_empty() && !links.is_empty() {
                download_urls.insert(resolution, links);
            }
        }
    }

    // Next/Previous episode
    let next_episode = {
        let sel = Selector::parse(".flir a[title=\"Episode Selanjutnya\"]").unwrap();
        document
            .select(&sel)
            .next()
            .and_then(|a| a.value().attr("href"))
            .and_then(|href| {
                let parts: Vec<&str> = href.trim_matches('/').split('/').collect();
                if parts.len() >= 2 {
                    Some(format!("{}/{}", parts[parts.len() - 2], parts[parts.len() - 1]))
                } else {
                    None
                }
            })
            .map(|slug| EpisodeInfo { slug })
    };
    let has_next_episode = next_episode.is_some();

    let previous_episode = {
        let sel = Selector::parse(".flir a[title=\"Episode Sebelumnya\"]").unwrap();
        document
            .select(&sel)
            .next()
            .and_then(|a| a.value().attr("href"))
            .and_then(|href| {
                let parts: Vec<&str> = href.trim_matches('/').split('/').collect();
                if parts.len() >= 2 {
                    Some(format!("{}/{}", parts[parts.len() - 2], parts[parts.len() - 1]))
                } else {
                    None
                }
            })
            .map(|slug| EpisodeInfo { slug })
    };
    let has_previous_episode = previous_episode.is_some();

    let data = AnimeData {
        episode,
        episode_number,
        anime: AnimeInfo { slug },
        has_next_episode,
        next_episode,
        has_previous_episode,
        previous_episode,
        stream_url,
        download_urls,
        image_url,
    };

    let response = AnimeResponse {
        status: "Ok",
        data,
    };

    Json(response).into_response()
}
