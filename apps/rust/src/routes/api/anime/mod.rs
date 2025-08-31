// Handler for /api/anime endpoints. Fetches anime data from otakudesu.cloud and returns as JSON.

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub mod complete_anime;
pub use complete_anime::complete_anime_handler;
pub mod ongoing_anime;
pub use ongoing_anime::ongoing_anime_handler;
mod full;
pub use full::full_handler;
mod search;
pub use search::search_handler;

pub async fn anime_handler() -> Response {
    let client = Client::new();

    let ongoing_html = match client.get("https://otakudesu.cloud/ongoing-anime/")
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "message": "Failed to fetch ongoing anime",
                        "error": e.to_string()
                    })),
                ).into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch ongoing anime",
                    "error": e.to_string()
                })),
            ).into_response();
        }
    };

    let complete_html = match client.get("https://otakudesu.cloud/complete-anime/")
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "message": "Failed to fetch complete anime",
                        "error": e.to_string()
                    })),
                ).into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch complete anime",
                    "error": e.to_string()
                })),
            ).into_response();
        }
    };

    let ongoing_anime = parse_ongoing_anime(&ongoing_html);
    let complete_anime = parse_complete_anime(&complete_html);

    let response = serde_json::json!({
        "status": "Ok",
        "data": {
            "ongoing_anime": ongoing_anime,
            "complete_anime": complete_anime,
        }
    });

    Json(response).into_response()
}

fn parse_ongoing_anime(html: &str) -> Vec<HashMap<String, String>> {
    let document = Html::parse_document(html);
    let bs_selector = Selector::parse(".page .listupd .bs").unwrap();
    let ntitle_selector = Selector::parse(".ntitle").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epx_selector = Selector::parse(".epx").unwrap();

    let mut result = Vec::new();

    for bs in document.select(&bs_selector) {
        let mut anime = HashMap::new();
        let title = bs
            .select(&ntitle_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let anime_url = bs
            .select(&a_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("")
            .to_string();
        let slug = anime_url
            .split('/')
            .nth(3)
            .unwrap_or("")
            .to_string();
        let poster = bs
            .select(&img_selector)
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("")
            .to_string();
        let current_episode = bs
            .select(&epx_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        anime.insert("title".to_string(), title);
        anime.insert("slug".to_string(), slug);
        anime.insert("poster".to_string(), poster);
        anime.insert("current_episode".to_string(), current_episode);
        anime.insert("anime_url".to_string(), anime_url);

        result.push(anime);
    }

    result
}

fn parse_complete_anime(html: &str) -> Vec<HashMap<String, String>> {
    let document = Html::parse_document(html);
    let bs_selector = Selector::parse(".page .listupd .bs").unwrap();
    let ntitle_selector = Selector::parse(".ntitle").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epx_selector = Selector::parse(".epx").unwrap();

    let mut result = Vec::new();

    for bs in document.select(&bs_selector) {
        let mut anime = HashMap::new();
        let title = bs
            .select(&ntitle_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let anime_url = bs
            .select(&a_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("")
            .to_string();
        let slug = anime_url
            .split('/')
            .nth(3)
            .unwrap_or("")
            .to_string();
        let poster = bs
            .select(&img_selector)
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("")
            .to_string();
        let episode_count = bs
            .select(&epx_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        anime.insert("title".to_string(), title);
        anime.insert("slug".to_string(), slug);
        anime.insert("poster".to_string(), poster);
        anime.insert("episode_count".to_string(), episode_count);
        anime.insert("anime_url".to_string(), anime_url);

        result.push(anime);
    }

    result
}
