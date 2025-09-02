// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/komik/manhwa";
const ENDPOINT_DESCRIPTION: &str = "Fetches manhwa list, parses HTML, and returns JSON response.";
const ENDPOINT_TAG: &str = "komik";
const SUCCESS_RESPONSE_BODY: &str = "ManhwaListResponse";
const PAGE_DESCRIPTION: &str = "Page number.";
// --- AKHIR METADATA ---

use axum::{extract::Query, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};
use utoipa::ToSchema;
use axum::http::StatusCode;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct ManhwaData {
    pub title: String,
    pub poster: String,
    pub chapter: String,
    pub date: String,
    pub score: String,
    pub r#type: String,
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<u32>,
    pub has_previous_page: bool,
    pub previous_page: Option<u32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct ManhwaListResponse {
    pub data: Vec<ManhwaData>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct Params {
    pub page: Option<u32>,
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

pub async fn manhwa_handler(Query(params): Query<Params>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let url = format!("https://komikcast.site/manhwa/page/{}/", page);

    let html = match reqwest::get(&url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => return ErrorResponse {
                message: "Failed to read response body".to_string(),
                error: e.to_string(),
            }.into_response(),
        },
        Err(e) => return ErrorResponse {
            message: "Failed to fetch data".to_string(),
            error: e.to_string(),
        }.into_response(),
    };

    let document = Html::parse_document(&html);
    let item_selector = Selector::parse(".animposx").unwrap();
    let title_selector = Selector::parse(".tt h4").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let chapter_selector = Selector::parse(".lsch a").unwrap();
    let score_selector = Selector::parse("i").unwrap();
    let date_selector = Selector::parse(".datech").unwrap();
    let typeflag_selector = Selector::parse(".typeflag").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut data = Vec::new();

    for item in document.select(&item_selector) {
        let title = item.select(&title_selector).next().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let poster = item.select(&img_selector).next().and_then(|n| n.value().attr("src")).map(|s| s.split('?').next().unwrap_or("").to_string()).unwrap_or_default();
        let chapter = item.select(&chapter_selector).next().map(|n| {
            let txt = n.text().collect::<String>().replace("Ch.", "");
            txt.trim().split_whitespace().next().unwrap_or("").to_string()
        }).unwrap_or_default();
        let score = item.select(&score_selector).next().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let date = item.select(&date_selector).next().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let r#type = item.select(&typeflag_selector).next().and_then(|n| n.value().attr("class")).map(|c| c.split_whitespace().nth(1).unwrap_or("").to_string()).unwrap_or_default();
        let slug = item.select(&a_selector).next().and_then(|n| n.value().attr("href")).map(|href| href.split('/').nth(4).unwrap_or("").to_string()).unwrap_or_default();
        data.push(ManhwaData {
            title,
            poster,
            chapter,
            date,
            score,
            r#type,
            slug,
        });
    }

    // Pagination parsing
    let pagination_selector = Selector::parse(".pagination").unwrap();
    let current_selector = Selector::parse(".current").unwrap();
    let next_selector = Selector::parse(".next").unwrap();
    let prev_selector = Selector::parse(".prev").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let pagination = if let Some(pagination_node) = document.select(&pagination_selector).next() {
        let current_page = pagination_node.select(&current_selector).next().and_then(|n| n.text().collect::<String>().trim().parse().ok()).unwrap_or(1);
        let last_visible_page = pagination_node.select(&a_selector).filter(|n| !n.value().classes().any(|c| c == "next")).last().and_then(|n| n.text().collect::<String>().trim().parse().ok()).unwrap_or(current_page);
        let has_next_page = pagination_node.select(&next_selector).next().is_some();
        let has_previous_page = pagination_node.select(&prev_selector).next().is_some();
        let next_page = if has_next_page && current_page < last_visible_page { Some(current_page + 1) } else { None };
        let previous_page = if has_previous_page && current_page > 1 { Some(current_page - 1) } else { None };
        Pagination {
            current_page,
            last_visible_page,
            has_next_page,
            next_page,
            has_previous_page,
            previous_page,
        }
    } else {
        Pagination {
            current_page: 1,
            last_visible_page: 1,
            has_next_page: false,
            next_page: None,
            has_previous_page: false,
            previous_page: None,
        }
    };

    Json(ManhwaListResponse { data, pagination }).into_response()
}
