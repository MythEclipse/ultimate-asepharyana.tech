use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use reqwest;
use scraper::{Html, Selector};
use regex::Regex;
use rust_lib::config::CONFIG_MAP;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/search";
pub const ENDPOINT_DESCRIPTION: &str = "Searches for komik based on query parameters.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_search";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaItem {
    pub title: String,
    pub image: String,
    pub chapter: String,
    pub score: String,
    pub date: String,
    pub r#type: String,
    pub komik_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SearchResponse {
    pub data: Vec<MangaItem>,
    pub prev_page: bool,
    pub next_page: bool,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/api/komik/search",
    tag = "komik",
    operation_id = "komik_search",
    responses(
        (status = 200, description = "Searches for komik based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    let query = params.q.unwrap_or_default();
    let base_url = CONFIG_MAP
        .get("KOMIK_BASE_URL")
        .cloned()
        .unwrap_or_else(|| "https://komikindo.id".to_string());

    let url = if query.is_empty() {
        format!("{}/page/1/", base_url)
    } else {
        format!("{}/page/1/?s={}", base_url, urlencoding::encode(&query))
    };

    match fetch_and_parse_search(&url).await {
        Ok(response) => Json(response),
        Err(_) => Json(SearchResponse {
            data: vec![],
            prev_page: false,
            next_page: false,
        }),
    }
}

async fn fetch_and_parse_search(url: &str) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let animposx_selector = Selector::parse(".animposx").unwrap();
    let title_selector = Selector::parse(".tt h4").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let chapter_selector = Selector::parse(".lsch a").unwrap();
    let score_selector = Selector::parse("i").unwrap();
    let date_selector = Selector::parse(".datech").unwrap();
    let type_selector = Selector::parse(".typeflag").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    let mut data = Vec::new();

    for element in document.select(&animposx_selector) {
        let title = element
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let mut image = element
            .select(&img_selector)
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();
        if let Some(pos) = image.find('?') {
            image = image[..pos].to_string();
        }

        let chapter_text = element
            .select(&chapter_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let chapter = Regex::new(r"\d+(\.\d+)?")
            .unwrap()
            .find(&chapter_text)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        let score = element
            .select(&score_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let date = element
            .select(&date_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let r#type = element
            .select(&type_selector)
            .next()
            .and_then(|e| e.value().attr("class"))
            .and_then(|class| class.split_whitespace().nth(1))
            .unwrap_or("")
            .to_string();

        let komik_id = element
            .select(&link_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();

        if !title.is_empty() {
            data.push(MangaItem {
                title,
                image,
                chapter,
                score,
                date,
                r#type,
                komik_id,
            });
        }
    }

    let pagination = parse_pagination(&document);

    Ok(SearchResponse {
        data,
        prev_page: pagination.has_previous_page,
        next_page: pagination.has_next_page,
    })
}

fn parse_pagination(document: &Html) -> Pagination {
    let current_selector = Selector::parse(".pagination .current").unwrap();
    let page_selectors = Selector::parse(".pagination a:not(.next)").unwrap();
    let next_selector = Selector::parse(".pagination .next").unwrap();
    let prev_selector = Selector::parse(".pagination .prev").unwrap();

    let current_page = document
        .select(&current_selector)
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(1);

    let mut last_visible_page = current_page;
    for element in document.select(&page_selectors) {
        if let Ok(page) = element.text().collect::<String>().trim().parse::<u32>() {
            if page > last_visible_page {
                last_visible_page = page;
            }
        }
    }

    let has_next_page = document.select(&next_selector).next().is_some();
    let has_previous_page = document.select(&prev_selector).next().is_some();

    Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        has_previous_page,
    }
}

#[derive(Debug)]
struct Pagination {
    current_page: u32,
    last_visible_page: u32,
    has_next_page: bool,
    has_previous_page: bool,
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}