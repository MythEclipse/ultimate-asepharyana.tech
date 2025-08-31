// Handler for GET /api/anime2. Fetches ongoing and complete anime lists from alqanime.net and returns them as JSON.
// Uses reqwest for HTTP requests and scraper for HTML parsing.

use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};

mod complete_anime;
use complete_anime::complete_anime_handler;
mod detail;
use detail::detail_handler;
mod search;
mod ongoing_anime;
use ongoing_anime::ongoing_anime_handler;

#[derive(Serialize)]
struct OngoingAnime {
    title: String,
    slug: String,
    poster: String,
    current_episode: String,
    anime_url: String,
}

#[derive(Serialize)]
struct CompleteAnime {
    title: String,
    slug: String,
    poster: String,
    episode_count: String,
    anime_url: String,
}

#[derive(Serialize)]
struct Anime2Response {
    status: &'static str,
    data: Anime2Data,
}

#[derive(Serialize)]
struct Anime2Data {
    ongoing_anime: Vec<OngoingAnime>,
    complete_anime: Vec<CompleteAnime>,
}

// Main router for /api/anime2 endpoints
pub fn router() -> Router {
    Router::new()
        .route("/complete-anime/:slug", get(complete_anime_handler))
        .route("/ongoing-anime/:slug", get(crate::routes::api::anime2::ongoing_anime::ongoing_anime_handler))
        .route("/detail/:slug", get(detail_handler))
        .route("/search", get(search::search_handler))
        // Add other routes here as needed
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
