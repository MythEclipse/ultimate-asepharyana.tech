use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use scraper::{Html, Selector};
use std::sync::Arc;

use crate::routes::ChatState;
use rust_lib::utils::fetch_with_proxy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullAnimeData {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
    pub status: String,
    pub release_date: String,
    pub studio: String,
    pub genre: Vec<String>,
    pub synopsis: String,
}

async fn fetch_anime_page_full(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://otakudesu.cloud/anime/{}", slug);
    let response = fetch_with_proxy(&url).await?;
    Ok(response)
}

fn parse_anime_page_full(html: &str, slug: &str) -> FullAnimeData {
    let document = Html::parse_document(html);

    let title_selector = Selector::parse(".jdl h1").unwrap();
    let poster_selector = Selector::parse(".fotoanime img").unwrap();
    let detail_selector = Selector::parse(".infozingle p").unwrap();
    let synopsis_selector = Selector::parse(".sinopc").unwrap();
    let genre_selector = Selector::parse(".genre-info a").unwrap();

    let title = document.select(&title_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let poster = document.select(&poster_selector).next().and_then(|e| e.value().attr("src")).unwrap_or_default().to_string();

    let mut status = String::new();
    let mut release_date = String::new();
    let mut studio = String::new();
    let mut episode = String::new();

    for element in document.select(&detail_selector) {
        let text = element.text().collect::<String>();
        if text.contains("Status:") {
            status = text.replace("Status:", "").trim().to_string();
        } else if text.contains("Rilis:") {
            release_date = text.replace("Rilis:", "").trim().to_string();
        } else if text.contains("Studio:") {
            studio = text.replace("Studio:", "").trim().to_string();
        } else if text.contains("Episode:") {
            episode = text.replace("Episode:", "").trim().to_string();
        }
    }

    let synopsis = document.select(&synopsis_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let genre: Vec<String> = document.select(&genre_selector).map(|e| e.text().collect::<String>().trim().to_string()).collect();

    let anime_url = format!("https://otakudesu.cloud/anime/{}", slug);

    FullAnimeData {
        title,
        slug: slug.to_string(),
        poster,
        episode,
        anime_url,
        status,
        release_date,
        studio,
        genre,
        synopsis,
    }
}

pub async fn full_anime_handler(
    Path(slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
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
