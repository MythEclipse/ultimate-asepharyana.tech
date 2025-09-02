// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/komik/chapter";
const ENDPOINT_DESCRIPTION: &str = "Fetches and parses manga chapter, returning JSON with title, next/prev chapter IDs, images, and list_chapter.";
const ENDPOINT_TAG: &str = "komik";
const SUCCESS_RESPONSE_BODY: &str = "MangaChapterResponse";
const CHAPTER_URL_DESCRIPTION: &str = "Chapter URL to fetch (e.g., 'isekai-ojisan-chapter-1').";
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
pub struct ChapterParams {
    pub chapter_url: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapter {
    pub title: String,
    pub next_chapter_id: String,
    pub prev_chapter_id: String,
    pub images: Vec<String>,
    pub list_chapter: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapterResponse {
    pub status: &'static str,
    pub data: MangaChapter,
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

pub async fn chapter_handler(Query(params): Query<ChapterParams>) -> Response {
    let chapter_url = match &params.chapter_url {
        Some(url) if !url.is_empty() => url.clone(),
        _ => return ErrorResponse {
            message: "Missing chapter_url parameter".to_string(),
            error: "Missing parameter".to_string(),
        }.into_response(),
    };
    let base_url = "https://komikcast.site";
    let url = format!("{}/chapter/{}", base_url, chapter_url);

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

    // Title
    let title = select_text(&document, ".entry-title");

    // Previous chapter ID
    let prev_chapter_id = {
        let sel = Selector::parse(".nextprev a[rel=\"prev\"]").expect("Failed to parse selector for previous chapter");
        document.select(&sel).next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(3))
            .unwrap_or("")
            .to_string()
    };

    // Next chapter ID
    let next_chapter_id = {
        let sel = Selector::parse(".nextprev a[rel=\"next\"]").expect("Failed to parse selector for next chapter");
        document.select(&sel).next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(3))
            .unwrap_or("")
            .to_string()
    };

    // List chapter
    let list_chapter = {
        let sel = Selector::parse(".nextprev a:has(.icol.daftarch)").expect("Failed to parse selector for list chapter");
        document.select(&sel).next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string()
    };

    // Images
    let mut images = Vec::new();
    if let Ok(sel) = Selector::parse("#chimg-auh img") {
        for el in document.select(&sel) {
            if let Some(src) = el.value().attr("src") {
                images.push(src.to_string());
            }
        }
    }

    Json(MangaChapterResponse {
        status: "Ok",
        data: MangaChapter {
            title,
            next_chapter_id,
            prev_chapter_id,
            images,
            list_chapter,
        },
    }).into_response()
}

// Utility: select text from document
fn select_text(document: &Html, selector: &str) -> String {
    if let Ok(sel) = Selector::parse(selector) {
        document.select(&sel).next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}
