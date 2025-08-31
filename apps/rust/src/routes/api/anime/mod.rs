use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod search;
pub mod detail;
pub mod episode;
pub mod anime_service; // otakudesu
pub mod anime_dto;     // otakudesu
pub mod alqanime_service;
pub mod alqanime_dto;

// NOTE: The /search endpoint now supports ?status=complete or ?status=ongoing for filtering.

pub mod anime_detail_dto;

// Functions from complete-anime/mod.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::routes::api::komik::manga_dto::Pagination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCompleteData {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
}

async fn fetch_with_proxy(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

async fn fetch_anime_page_complete(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://otakudesu.cloud/complete-anime/page/{}/", slug);
    let response = fetch_with_proxy(&url).await?;
    Ok(response)
}

fn parse_anime_page_complete(html: &str, slug: &str) -> (Vec<AnimeCompleteData>, Pagination) {
    let document = Html::parse_document(html);

    let mut anime_list: Vec<AnimeCompleteData> = Vec::new();

    let pagination = {
        let current_page = slug.parse::<u32>().unwrap_or(1);
        let last_visible_page_selector = Selector::parse(".pagenavix .page-numbers:not(.next):last").unwrap();
        let last_visible_page = document.select(&last_visible_page_selector)
            .next()
            .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
            .unwrap_or(1);

        let has_next_page = document.select(&Selector::parse(".pagenavix .next.page-numbers").unwrap()).next().is_some();
        let next_page = if has_next_page { Some(current_page + 1) } else { None };
        let previous_page = if current_page > 1 { Some(current_page - 1) } else { None };

        Pagination {
            current_page,
            last_visible_page,
            has_next_page,
            next_page,
            previous_page,
        }
    };

    let anime_selector = Selector::parse(".venz ul li").unwrap();
    for element in document.select(&anime_selector) {
        let title = element.select(&Selector::parse(".thumbz h2.jdlflm").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let slug_val = element.select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4).map(|s| s.to_string()))
            .unwrap_or_default();
        let poster = element.select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|e| e.value().attr("src").map(|s| s.to_string()))
            .unwrap_or_default();
        let episode = element.select(&Selector::parse(".epz").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let anime_url = element.select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|e| e.value().attr("href").map(|s| s.to_string()))
            .unwrap_or_default();

        anime_list.push(AnimeCompleteData {
            title,
            slug: slug_val,
            poster,
            episode,
            anime_url,
        });
    }

    (anime_list, pagination)
}

pub async fn complete_anime_handler(
    Path(slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    match fetch_anime_page_complete(&slug).await {
        Ok(html) => {
            let (anime_list, pagination) = parse_anime_page_complete(&html, &slug);
            (
                StatusCode::OK,
                Json(json!({
                    "status": "Ok",
                    "data": anime_list,
                    "pagination": pagination,
                })),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("Complete anime error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to process request: {}", e) })),
            )
                .into_response()
        }
    }
}

// Functions from ongoing-anime/mod.rs
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OngoingAnimeResponse {
    pub status: String,
    pub data: Vec<AnimeItem>,
    pub pagination: Pagination,
}

async fn fetch_anime_page_ongoing(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://otakudesu.cloud/ongoing-anime/page/{}/", slug);
    let response = fetch_with_proxy(&url).await?;
    Ok(response)
}

fn parse_anime_page_ongoing(html: &str, slug: &str) -> (Vec<AnimeItem>, Pagination) {
    let document = Html::parse_document(html);

    let mut anime_list: Vec<AnimeItem> = Vec::new();
    let anime_selector = Selector::parse(".venz ul li").unwrap();
    let title_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epz_selector = Selector::parse(".epz").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    for element in document.select(&anime_selector) {
        let title = element.select(&title_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let anime_url = element.select(&a_selector).next().and_then(|e| e.value().attr("href")).unwrap_or_default().to_string();
        let item_slug = anime_url.split('/').nth(4).unwrap_or_default().to_string();
        let poster = element.select(&img_selector).next().and_then(|e| e.value().attr("src")).unwrap_or_default().to_string();
        let episode = element.select(&epz_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or("Ongoing".to_string());

        anime_list.push(AnimeItem {
            title,
            slug: item_slug,
            poster,
            episode,
            anime_url,
        });
    }

    let pagination_selector = Selector::parse(".pagination .page-numbers:not(.next):last").unwrap();
    let next_selector = Selector::parse(".pagination .next").unwrap();

    let current_page_int = slug.parse::<u32>().unwrap_or(1);
    let last_visible_page_text = document.select(&pagination_selector).next().map(|e| e.text().collect::<String>()).unwrap_or("1".to_string());
    let last_visible_page_int = last_visible_page_text.parse::<u32>().unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();
    let next_page = if has_next_page { Some(current_page_int + 1) } else { None };
    let previous_page = if current_page_int > 1 { Some(current_page_int - 1) } else { None };

    let pagination = Pagination {
        current_page: current_page_int,
        last_visible_page: last_visible_page_int,
        has_next_page,
        next_page,
        previous_page,
    };

    (anime_list, pagination)
}

pub async fn get_ongoing_anime(
    Path(slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    match fetch_anime_page_ongoing(&slug).await {
        Ok(html) => {
            let (anime_list, pagination) = parse_anime_page_ongoing(&html, &slug);
            (
                StatusCode::OK,
                Json(json!({
                    "status": "Ok",
                    "data": anime_list,
                    "pagination": pagination,
                })),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("Error fetching anime page: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to fetch anime data: {}", e) })),
            )
                .into_response()
        }
    }
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/complete-anime/:slug", axum::routing::get(complete_anime_handler))
        .route("/ongoing-anime/:slug", axum::routing::get(get_ongoing_anime))
}
