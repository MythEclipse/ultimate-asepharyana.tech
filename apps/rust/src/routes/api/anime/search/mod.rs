// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime/search"; // The path doesn't have a slug
const ENDPOINT_DESCRIPTION: &str = "Search for anime from otakudesu.cloud";
const ENDPOINT_TAG: &str = "anime";
const SUCCESS_RESPONSE_BODY: &str = "SearchResponse";
const SLUG_DESCRIPTION: &str = "Query parameter for anime search (q).";
// --- AKHIR METADATA ---

use axum::{
    extract::Query,
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
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub status: String,
    pub rating: String,
}

#[derive(Serialize, ToSchema)]
pub struct Pagination {
    pub current_page: usize,
    pub has_next_page: bool,
    pub next_page: Option<usize>,
    pub has_previous_page: bool,
    pub previous_page: Option<usize>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub status: &'static str,
    pub data: Vec<AnimeItem>,
    pub pagination: Pagination,
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

pub async fn search_handler(Query(params): Query<HashMap<String, String>>) -> Response {
    let q = params.get("q").map(|s| s.as_str()).unwrap_or("one");
    let url = format!("https://otakudesu.cloud/?s={}&post_type=anime", q);

    let client = Client::new();
    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return ErrorResponse {
                    message: "Failed to fetch data".to_string(),
                    error: e.to_string(),
                }.into_response();
            }
        },
        Err(e) => {
            return ErrorResponse {
                message: "Failed to fetch data".to_string(),
                error: e.to_string(),
            }.into_response();
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
    let li_selector = Selector::parse("#venkonten .chivsrc li").expect("Failed to parse selector for anime list items");
    let mut anime_list = Vec::new();

    for li in document.select(&li_selector) {
        let title = li
            .select(&Selector::parse("h2 a").expect("Failed to parse selector for anime title"))
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let anime_url = li
            .select(&Selector::parse("a").expect("Failed to parse selector for anime URL"))
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
            .select(&Selector::parse("img").expect("Failed to parse selector for anime poster"))
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let episode_text = li
            .select(&Selector::parse("h2 a").expect("Failed to parse selector for episode text"))
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or_default();

        let episode = {
            let re = regex::Regex::new(r"\(([^)]+)\)").expect("Failed to compile regex for episode parsing");
            re.captures(&episode_text)
                .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                .unwrap_or_else(|| "Ongoing".to_string())
        };

        let genres = li
            .select(&Selector::parse(".set b").expect("Failed to parse selector for genres"))
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
            .select(&Selector::parse(".set b").expect("Failed to parse selector for status"))
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
            .select(&Selector::parse(".set b").expect("Failed to parse selector for rating"))
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
    let has_next_page = document.select(&Selector::parse(".hpage .r").expect("Failed to parse selector for next page")).next().is_some();
    let pagination = Pagination {
        current_page: page_num,
        has_next_page,
        next_page: if has_next_page { Some(page_num + 1) } else { None },
        has_previous_page: page_num > 1,
        previous_page: if page_num > 1 { Some(page_num - 1) } else { None },
    };

    (anime_list, pagination)
}
