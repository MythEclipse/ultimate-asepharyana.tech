// Handler for GET /api/komik/chapter
// Fetches and parses manga chapter, returning JSON with title, next/prev chapter IDs, images, and list_chapter.

use axum::{
    extract::Query,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ChapterParams {
    pub chapter_url: Option<String>,
}

#[derive(Serialize)]
pub struct MangaChapter {
    pub title: String,
    pub next_chapter_id: String,
    pub prev_chapter_id: String,
    pub images: Vec<String>,
    pub list_chapter: String,
}

pub async fn handler(Query(params): Query<ChapterParams>) -> impl IntoResponse {
    let chapter_url = match &params.chapter_url {
        Some(url) if !url.is_empty() => url.clone(),
        _ => return Json(error_response("Missing chapter_url parameter")).into_response(),
    };
    let base_url = "https://komikcast.site";
    let url = format!("{}/chapter/{}", base_url, chapter_url);

    let client = Client::new();
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(_) => return Json(error_response("Failed to fetch data")).into_response(),
    };
    let body = match resp.text().await {
        Ok(b) => b,
        Err(_) => return Json(error_response("Failed to read response body")).into_response(),
    };

    let document = Html::parse_document(&body);

    // Title
    let title = select_text(&document, ".entry-title");

    // Previous chapter ID
    let prev_chapter_id = {
        let sel = Selector::parse(".nextprev a[rel=\"prev\"]").unwrap();
        document.select(&sel).next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(3))
            .unwrap_or("")
            .to_string()
    };

    // Next chapter ID
    let next_chapter_id = {
        let sel = Selector::parse(".nextprev a[rel=\"next\"]").unwrap();
        document.select(&sel).next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(3))
            .unwrap_or("")
            .to_string()
    };

    // List chapter
    let list_chapter = {
        let sel = Selector::parse(".nextprev a:has(.icol.daftarch)").unwrap();
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

    Json(MangaChapter {
        title,
        next_chapter_id,
        prev_chapter_id,
        images,
        list_chapter,
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

fn error_response(msg: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    map.insert("status", "false");
    map.insert("message", msg);
    map
}
