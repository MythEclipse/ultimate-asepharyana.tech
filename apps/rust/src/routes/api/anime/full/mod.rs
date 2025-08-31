// Annotated with utoipa for OpenAPI generation

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use scraper::{Html, Selector};
use std::{collections::HashMap, error::Error};
use regex::Regex;

use rust_lib::fetch_with_proxy::fetch_with_proxy;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnimeData {
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnimeInfo {
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EpisodeInfo {
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DownloadLink {
    pub server: String,
    pub url: String,
}

async fn fetch_anime_page_full(slug: &str) -> Result<String, Box<dyn Error>> {
    tracing::info!("[DEBUG] full/mod.rs using rust_lib::fetch_with_proxy import");
    let url = format!("https://otakudesu.cloud/episode/{}/", slug);
    let response = fetch_with_proxy(&url).await?;
    tracing::info!("[DEBUG] full/mod.rs fetched body: {} bytes", response.len());
    tracing::debug!("FetchResult (full/mod.rs): {:?}", &response);
    Ok(response.data)
}

fn parse_anime_page_full(html: &str, slug: &str) -> AnimeData {
    let body = html.to_string();
    let document = Html::parse_document(&body);

    let episode = document
        .select(&Selector::parse("h1.posttl").unwrap())
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default();
    let episode_number_re = Regex::new(r"Episode (\d+)").unwrap();
    let episode_number = episode_number_re
        .captures(&episode)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_default();
    let image_url = document
        .select(&Selector::parse(".cukder img").unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or_default()
        .to_string();
    let stream_url = document
        .select(&Selector::parse("#embed_holder iframe").unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or_default()
        .to_string();

    let mut download_urls: HashMap<String, Vec<DownloadLink>> = HashMap::new();

    let download_selector = Selector::parse(".download ul li").unwrap();
    for element in document.select(&download_selector) {
        let resolution = element
            .select(&Selector::parse("strong").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let links: Vec<DownloadLink> = element
            .select(&Selector::parse("a").unwrap())
            .map(|link_element| DownloadLink {
                server: link_element
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string(),
                url: link_element
                    .value()
                    .attr("href")
                    .unwrap_or_default()
                    .to_string(),
            })
            .collect();

        if !resolution.is_empty() && !links.is_empty() {
            download_urls.insert(resolution, links);
        }
    }

    let next_episode_element = document.select(&Selector::parse(".flir a[title=\"Episode Selanjutnya\"]").unwrap()).next();
    let prev_episode_element = document.select(&Selector::parse(".flir a[title=\"Episode Sebelumnya\"]").unwrap()).next();

    let next_episode_url = next_episode_element.and_then(|e| e.value().attr("href"));
    let previous_episode_url = prev_episode_element.and_then(|e| e.value().attr("href"));

    let next_episode_slug = next_episode_url.map(|url| {
        let segments: Vec<&str> = url.split('/').collect();
        if segments.len() >= 2 {
            segments[segments.len() - 2..].join("/")
        } else {
            url.to_string()
        }
    });
    let previous_episode_slug = previous_episode_url.map(|url| {
        let segments: Vec<&str> = url.split('/').collect();
        if segments.len() >= 2 {
            segments[segments.len() - 2..].join("/")
        } else {
            url.to_string()
        }
    });

    AnimeData {
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
    }
}

#[utoipa::path(
    get,
    path = "/api/anime/full/{slug}",
    params(
        ("slug" = String, Path, description = "Anime episode slug")
    ),
    responses(
        (status = 200, description = "Full anime episode data", body = AnimeData),
        (status = 500, description = "Internal server error")
    ),
    tag = "Anime"
)]
pub async fn full_anime_handler(
    Path(slug): Path<String>,
) -> Response {
    match fetch_anime_page_full(&slug).await {
        Ok(html) => {
            let anime_data = parse_anime_page_full(&html, &slug);
            (
                StatusCode::OK,
                Json(json!({
                    "status": "Ok",
                    "data": anime_data,
                })),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("Full anime error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to process request: {}", e) })),
            )
                .into_response()
        }
    }
}
