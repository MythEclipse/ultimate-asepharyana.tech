// Handler for GET /api/anime2/search. Fetches search results from alqanime.net and returns them as JSON.
// Uses reqwest for HTTP requests and scraper for HTML parsing.

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AnimeSearchItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub description: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub rating: String,
    pub r#type: String,
    pub season: String,
}

#[derive(Serialize)]
pub struct Pagination {
    pub current_page: usize,
    pub last_visible_page: usize,
    pub has_next_page: bool,
    pub next_page: Option<String>,
    pub has_previous_page: bool,
    pub previous_page: Option<String>,
}

#[derive(Serialize)]
pub struct AnimeSearchResponse {
    pub status: &'static str,
    pub data: Vec<AnimeSearchItem>,
    pub pagination: Pagination,
}

pub async fn search_handler(Query(params): Query<HashMap<String, String>>) -> Response {
    let slug = params.get("q").map(|s| s.as_str()).unwrap_or("log");
    let url = format!("https://alqanime.net/?s={}", slug);

    let client = Client::new();
    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "message": "Failed to fetch data",
                        "error": e.to_string()
                    })),
                ).into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch data",
                    "error": e.to_string()
                })),
            ).into_response();
        }
    };

    let (anime_list, pagination) = parse_anime_data(&html);

    let response = AnimeSearchResponse {
        status: "Ok",
        data: anime_list,
        pagination,
    };

    Json(response).into_response()
}

fn parse_anime_data(html: &str) -> (Vec<AnimeSearchItem>, Pagination) {
    let document = Html::parse_document(html);
    let bs_selector = Selector::parse(".listupd article.bs").unwrap();
    let ntitle_selector = Selector::parse(".ntitle").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let h2_selector = Selector::parse("h2").unwrap();
    let numscore_selector = Selector::parse(".numscore").unwrap();
    let typez_selector = Selector::parse(".typez").unwrap();

    let mut anime_list = Vec::new();

    for bs in document.select(&bs_selector) {
        let title = bs.select(&ntitle_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let anime_url = bs.select(&a_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("")
            .to_string();
        let slug = anime_url.split('/').nth(3).unwrap_or("").to_string();
        let poster = bs.select(&img_selector)
            .next()
            .and_then(|img| img.value().attr("data-src"))
            .unwrap_or("")
            .to_string();
        let description = bs.select(&h2_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let genres = Vec::new(); // Not available in HTML
        let rating = bs.select(&numscore_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let r#type = bs.select(&typez_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let season = String::new(); // Not available in HTML

        anime_list.push(AnimeSearchItem {
            title,
            slug,
            poster,
            description,
            anime_url,
            genres,
            rating,
            r#type,
            season,
        });
    }

    // Pagination
    let pagination_selector = Selector::parse(".pagination").unwrap();
    let current_selector = Selector::parse(".current").unwrap();
    let page_numbers_selector = Selector::parse(".page-numbers").unwrap();
    let next_selector = Selector::parse(".next").unwrap();
    let prev_selector = Selector::parse(".prev").unwrap();

    let pagination_root = document.select(&pagination_selector).next();
    let current_page = pagination_root
        .as_ref()
        .and_then(|p| p.select(&current_selector).next())
        .and_then(|n| n.text().collect::<String>().parse::<usize>().ok())
        .unwrap_or(1);

    let last_visible_page = pagination_root
        .as_ref()
        .and_then(|p| {
            let pages = p.select(&page_numbers_selector).collect::<Vec<_>>();
            if pages.len() >= 2 {
                let text = pages[pages.len() - 2].text().collect::<String>();
                text.parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(1);

    let has_next_page = pagination_root
        .as_ref()
        .map(|p| p.select(&next_selector).next().is_some())
        .unwrap_or(false);

    let next_page = pagination_root
        .as_ref()
        .and_then(|p| p.select(&next_selector).next())
        .and_then(|n| n.value().attr("href"))
        .map(|s| s.to_string());

    let has_previous_page = current_page > 1;

    let previous_page = if has_previous_page {
        pagination_root
            .as_ref()
            .and_then(|p| p.select(&prev_selector).next())
            .and_then(|n| n.value().attr("href"))
            .map(|s| s.to_string())
    } else {
        None
    };

    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    };

    (anime_list, pagination)
}
