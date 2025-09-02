// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime/full/{slug}";
const ENDPOINT_DESCRIPTION: &str = "Fetches and parses the anime episode page from otakudesu.cloud";
const ENDPOINT_TAG: &str = "anime";
const SUCCESS_RESPONSE_BODY: &str = "FullEpisodeResponse";
const SLUG_DESCRIPTION: &str = "Slug for the anime episode (e.g., 'isekai-ojisan-episode-1').";
// --- AKHIR METADATA ---

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use axum::http::StatusCode;

#[derive(Serialize, ToSchema)]
pub struct AnimeInfo {
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
pub struct EpisodeInfo {
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
pub struct DownloadLink {
    pub server: String,
    pub url: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FullEpisodeData {
    pub episode: String,
    pub episode_number: String,
    pub anime: AnimeInfo,
    pub has_next_episode: bool,
    pub next_episode: Option<EpisodeInfo>,
    pub has_previous_episode: bool,
    pub previous_episode: Option<EpisodeInfo>,
    pub stream_url: String,
    pub download_urls: HashMap<String, Vec<DownloadLink>>,
    pub image_url: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FullEpisodeResponse {
    pub status: &'static str,
    pub data: FullEpisodeData,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub error: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

/// Fetches and parses the anime episode page from otakudesu.cloud
#[utoipa::path(get, path = "/api/anime/full/{slug}", tag = "anime", responses((status = 200, description = "Success", body = FullEpisodeResponse), (status = 500, description = "Internal Server Error")), params(("slug" = String, Path, description = "Slug for the anime episode (e.g., 'isekai-ojisan-episode-1').")))]
pub async fn full_episode_handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!("https://otakudesu.cloud/episode/{}/", slug);

    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return ErrorResponse {
                    message: "Failed to fetch episode page".to_string(),
                    error: e.to_string(),
                }.into_response();
            }
        },
        Err(e) => {
            return ErrorResponse {
                message: "Failed to fetch episode page".to_string(),
                error: e.to_string(),
            }.into_response();
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

    let data = FullEpisodeData {
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

    let response = FullEpisodeResponse {
        status: "Ok",
        data,
    };

    Json(response).into_response()
}
