// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/komik/search";
const ENDPOINT_DESCRIPTION: &str = "Fetches and parses manga search results, returning JSON with data and pagination.";
const ENDPOINT_TAG: &str = "komik";
const SUCCESS_RESPONSE_BODY: &str = "SearchResponse";
const QUERY_DESCRIPTION: &str = "Query for manga search.";
const PAGE_DESCRIPTION: &str = "Page number for pagination.";
// --- AKHIR METADATA ---

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use utoipa::ToSchema;
use axum::http::StatusCode;

#[derive(Deserialize, ToSchema)]
pub struct SearchParams {
    pub query: Option<String>,
    pub page: Option<u32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MangaData {
    pub title: String,
    pub poster: String,
    pub chapter: String,
    pub score: String,
    pub date: String,
    pub r#type: String,
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<u32>,
    pub has_previous_page: bool,
    pub previous_page: Option<u32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub data: Vec<MangaData>,
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

pub async fn search_handler(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let query = params.query.unwrap_or_default();
    let page = params.page.unwrap_or(1);

    // TODO: Replace with actual base URL logic if needed
    let base_url = "https://komikcast.site";
    let url = format!("{}/page/{}/?s={}", base_url, page, urlencoding::encode(&query));

    let client = Client::new();
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => return ErrorResponse {
            message: "Failed to fetch data".to_string(),
            error: e.to_string(),
        }.into_response(),
    };
    let body = match resp.text().await {
        Ok(b) => b,
        Err(e) => return ErrorResponse {
            message: "Failed to read response body".to_string(),
            error: e.to_string(),
        }.into_response(),
    };

    let document = Html::parse_document(&body);
    let selector = Selector::parse(".animposx").unwrap();

    let mut data = Vec::new();
    for element in document.select(&selector) {
        let title = element
            .select(&Selector::parse(".tt h4").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let poster = element
            .select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|e| e.value().attr("src"))
            .map(|s| s.split('?').next().unwrap_or("").to_string())
            .unwrap_or_default();

        let chapter = element
            .select(&Selector::parse(".lsch a").unwrap())
            .next()
            .map(|e| {
                let txt = e.text().collect::<String>().trim().replace("Ch.", "");
                txt.split_whitespace().next().unwrap_or("").to_string()
            })
            .unwrap_or_default();

        let score = element
            .select(&Selector::parse("i").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let date = element
            .select(&Selector::parse(".datech").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let r#type = element
            .select(&Selector::parse(".typeflag").unwrap())
            .next()
            .and_then(|e| e.value().attr("class"))
            .and_then(|c| c.split_whitespace().nth(1))
            .unwrap_or("")
            .to_string();

        let slug = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();

        data.push(MangaData {
            title,
            poster,
            chapter,
            score,
            date,
            r#type,
            slug,
        });
    }

    // Pagination extraction
    let pagination = extract_pagination(&document, page);

    Json(SearchResponse { data, pagination }).into_response()
}

fn extract_pagination(document: &Html, current_page: u32) -> Pagination {
    let selector_current = Selector::parse(".pagination .current").unwrap();
    let selector_last = Selector::parse(".pagination a:not(.next):last-child").unwrap();
    let selector_next = Selector::parse(".pagination .next").unwrap();
    let selector_prev = Selector::parse(".pagination .prev").unwrap();

    let current = document
        .select(&selector_current)
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse().ok())
        .unwrap_or(current_page);

    let last = document
        .select(&selector_last)
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse().ok())
        .unwrap_or(current);

    let has_next = document.select(&selector_next).next().is_some();
    let has_prev = document.select(&selector_prev).next().is_some();

    Pagination {
        current_page: current,
        last_visible_page: last,
        has_next_page: has_next,
        next_page: if has_next && current < last { Some(current + 1) } else { None },
        has_previous_page: has_prev,
        previous_page: if has_prev && current > 1 { Some(current - 1) } else { None },
    }
}
