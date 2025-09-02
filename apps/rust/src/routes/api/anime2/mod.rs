// Handler for GET /api/anime2. Fetches ongoing and complete anime lists from alqanime.net and returns them as JSON.
// Uses reqwest for HTTP requests and scraper for HTML parsing.
// This module now exposes OpenAPI documentation for all anime2 endpoints.

use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};
use utoipa::OpenApi;
use crate::routes::ChatState;
use std::sync::Arc;

pub mod complete_anime;
pub mod detail;
pub mod search;
pub mod ongoing_anime;

#[derive(Serialize, utoipa::ToSchema)]
pub struct OngoingAnime {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub anime_url: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct CompleteAnime {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct Anime2Response {
    pub status: &'static str,
    pub data: Anime2Data,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct Anime2Data {
    pub ongoing_anime: Vec<OngoingAnime>,
    pub complete_anime: Vec<CompleteAnime>,
}

/// Aggregates OpenAPI docs for all anime2 endpoints.
#[derive(OpenApi)]
#[openapi(
    paths(
        complete_anime::complete_anime_handler,
        ongoing_anime::ongoing_anime_handler,
        detail::detail_handler,
        search::search_handler
    ),
    components(
        schemas(OngoingAnime, CompleteAnime, Anime2Response, Anime2Data, complete_anime::AnimeItem, complete_anime::Pagination, complete_anime::CompleteAnimeResponse, ongoing_anime::AnimeItem, ongoing_anime::Pagination, ongoing_anime::OngoingAnimeResponse, detail::Genre, detail::Recommendation, detail::Link, detail::DownloadGroup, detail::AnimeDetail, detail::AnimeDetailResponse, search::AnimeSearchItem, search::Pagination, search::AnimeSearchResponse),
    ),
    tags(
        (name = "anime2", description = "Anime2 API endpoints")
    )
)]
pub struct Anime2ApiDoc;

// Main router for /api/anime2 endpoints
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(anime2_handler))
        .route("/complete-anime/{slug}", get(complete_anime::complete_anime_handler))
        .route("/ongoing-anime/{slug}", get(ongoing_anime::ongoing_anime_handler))
        .route("/detail/{slug}", get(detail::detail_handler))
        .route("/search", get(search::search_handler))
}

pub async fn anime2_handler() -> Response {
    let client = Client::new();

    let ongoing_html = match client
        .get("https://alqanime.net/advanced-search/?status=ongoing&order=update")
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
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch ongoing anime",
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let complete_html = match client
        .get("https://alqanime.net/advanced-search/?status=completed&order=update")
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
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!( {
                    "message": "Failed to fetch complete anime",
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let ongoing_anime = parse_ongoing_anime(&ongoing_html);
    let complete_anime = parse_complete_anime(&complete_html);

    let response = Anime2Response {
        status: "Ok",
        data: Anime2Data {
            ongoing_anime,
            complete_anime,
        },
    };

    Json(response).into_response()
}

fn parse_ongoing_anime(html: &str) -> Vec<OngoingAnime> {
    let document = Html::parse_document(html);
    let bs_selector = Selector::parse(".listupd .bs").unwrap();
    let ntitle_selector = Selector::parse(".ntitle").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epx_selector = Selector::parse(".epx").unwrap();

    let mut result = Vec::new();

    for bs in document.select(&bs_selector) {
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
            .and_then(|img| img.value().attr("data-src"))
            .unwrap_or("")
            .to_string();
        let current_episode = bs
            .select(&epx_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        result.push(OngoingAnime {
            title,
            slug,
            poster,
            current_episode,
            anime_url,
        });
    }

    result
}

fn parse_complete_anime(html: &str) -> Vec<CompleteAnime> {
    let document = Html::parse_document(html);
    let bs_selector = Selector::parse(".listupd .bs").unwrap();
    let ntitle_selector = Selector::parse(".ntitle").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epx_selector = Selector::parse(".epx").unwrap();

    let mut result = Vec::new();

    for bs in document.select(&bs_selector) {
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
            .and_then(|img| img.value().attr("data-src"))
            .unwrap_or("")
            .to_string();
        let episode_count = bs
            .select(&epx_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        result.push(CompleteAnime {
            title,
            slug,
            poster,
            episode_count,
            anime_url,
        });
    }

    result
}
