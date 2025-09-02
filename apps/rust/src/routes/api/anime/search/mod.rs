// Handler for GET /api/anime/search. Fetches search results from otakudesu.cloud and returns them as JSON.
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

#[derive(Serialize, utoipa::ToSchema)]
pub struct AnimeItem {
    title: String,
    slug: String,
    poster: String,
    episode: String,
    anime_url: String,
    genres: Vec<String>,
    status: String,
    rating: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct Pagination {
    current_page: usize,
    last_visible_page: usize,
    has_next_page: bool,
    next_page: Option<usize>,
    has_previous_page: bool,
    previous_page: Option<usize>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct SearchResponse {
    status: &'static str,
    data: Vec<AnimeItem>,
    pagination: Pagination,
}

#[utoipa::path(
    get,
    path = "/api/anime/search",
    summary = "Search anime",
    description = "Searches for anime on otakudesu.cloud using the provided query string.",
    params(
        ("q" = String, Query, description = "Search query for anime title")
    ),
    responses(
        (status = 200, description = "Successfully retrieved anime search results", body = SearchResponse),
        (status = 500, description = "Internal server error", body = String)
    ),
    tag = "Anime"
)]
pub async fn handler(Query(params): Query<HashMap<String, String>>) -> Response {
    let q = params.get("q").map(|s| s.as_str()).unwrap_or("one");
    let url = format!("https://otakudesu.cloud/?s={}&post_type=anime", q);

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
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch data",
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let (anime_list, pagination) = parse_anime_data(&html, q);

    let response = SearchResponse {
        status: "Ok",
        data: anime_list,
        pagination,
    };

    Json(response).into_response()
}

fn parse_anime_data(html: &str, q: &str) -> (Vec<AnimeItem>, Pagination) {
    let document = Html::parse_document(html);
    let li_selector = Selector::parse("#venkonten .chivsrc li").unwrap();
    let mut anime_list = Vec::new();

    for li in document.select(&li_selector) {
        let title = li
            .select(&Selector::parse("h2 a").unwrap())
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let anime_url = li
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let slug = anime_url
            .split('/')
            .nth(4)
            .unwrap_or("")
            .to_string();

        let poster = li
            .select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let episode_text = li
            .select(&Selector::parse("h2 a").unwrap())
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or_default();

        let episode = {
            let re = regex::Regex::new(r"\(([^)]+)\)").unwrap();
            re.captures(&episode_text)
                .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                .unwrap_or_else(|| "Ongoing".to_string())
        };

        let genres = li
            .select(&Selector::parse(".set b").unwrap())
            .filter(|b| b.text().any(|t| t.contains("Genres")))
            .flat_map(|b| {
                b.parent()
                    .and_then(|parent| {
                        Some(
                            parent
                                .children()
                                .filter_map(|child| {
                                    if let Some(_a) = child.value().as_element().filter(|e| e.name() == "a") {
                                        Some(child.children()
                                            .filter_map(|c| c.value().as_text())
                                            .map(|t| t.as_ref() as &str)
                                            .collect::<String>())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                                .into_iter(),
                        )
                    })
                    .into_iter()
                    .flatten()
            })
            .collect::<Vec<_>>();

        let status = li
            .select(&Selector::parse(".set b").unwrap())
            .filter(|b| b.text().any(|t| t.contains("Status")))
            .next()
            .and_then(|b| b.parent())
            .map(|parent| {
                parent
                    .children()
                    .filter_map(|child| child.value().as_text().map(|t| t.to_string()))
                    .collect::<String>()
                    .replace("Status :", "")
                    .trim()
                    .to_string()
            })
            .unwrap_or_default();

        let rating = li
            .select(&Selector::parse(".set b").unwrap())
            .filter(|b| b.text().any(|t| t.contains("Rating")))
            .next()
            .and_then(|b| b.parent())
            .map(|parent| {
                parent
                    .children()
                    .filter_map(|child| child.value().as_text().map(|t| t.to_string()))
                    .collect::<String>()
                    .replace("Rating :", "")
                    .trim()
                    .to_string()
            })
            .unwrap_or_default();

        anime_list.push(AnimeItem {
            title,
            slug,
            poster,
            episode,
            anime_url,
            genres,
            status,
            rating,
        });
    }

    let page_num = q.parse::<usize>().unwrap_or(1);
    let has_next_page = document.select(&Selector::parse(".hpage .r").unwrap()).next().is_some();
    let pagination = Pagination {
        current_page: page_num,
        last_visible_page: 57,
        has_next_page,
        next_page: if has_next_page { Some(page_num + 1) } else { None },
        has_previous_page: page_num > 1,
        previous_page: if page_num > 1 { Some(page_num - 1) } else { None },
    };

    (anime_list, pagination)
}
