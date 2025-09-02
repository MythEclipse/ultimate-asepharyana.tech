// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/komik/detail/{komik_id}";
const ENDPOINT_DESCRIPTION: &str = "Fetches and parses manga detail, returning JSON with all fields.";
const ENDPOINT_TAG: &str = "komik";
const SUCCESS_RESPONSE_BODY: &str = "MangaDetailResponse";
const KOMIK_ID_DESCRIPTION: &str = "ID of the manga to fetch details for (e.g., 'one-piece').";
// --- AKHIR METADATA ---

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use scraper::{Html, Selector};
use utoipa::ToSchema;
use axum::http::StatusCode;

#[derive(Deserialize, ToSchema)]
pub struct DetailParams {
    pub komik_id: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MangaDetailData {
    pub title: String,
    pub alternative_title: String,
    pub score: String,
    pub poster: String,
    pub description: String,
    pub status: String,
    pub r#type: String,
    pub release_date: String,
    pub author: String,
    pub total_chapter: String,
    pub updated_on: String,
    pub genres: Vec<String>,
    pub chapters: Vec<ChapterInfo>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChapterInfo {
    pub chapter: String,
    pub date: String,
    pub chapter_id: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MangaDetailResponse {
    pub status: &'static str,
    pub data: MangaDetailData,
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

pub async fn detail_handler(Path(komik_id): Path<String>) -> Response {
    let base_url = "https://komikcast.site";
    let url = format!("{}/komik/{}", base_url, komik_id);

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
    let title = select_text(&document, "h1.entry-title");

    // Alternative Title
    let alternative_title = select_text(&document, ".spe span:contains('Judul Alternatif:')")
        .replace("Judul Alternatif:", "")
        .trim()
        .to_string();

    // Score
    let score = select_text(&document, ".rtg > div > i");

    // Poster
    let poster = select_attr(&document, ".thumb img", "src")
        .split('?')
        .next()
        .unwrap_or("")
        .to_string();

    // Description
    let description = select_text(&document, "#sinopsis > section > div > div.entry-content.entry-content-single > p");

    // Status
    let status = select_text(&document, ".spe span:contains('Status:')")
        .replace("Status:", "")
        .trim()
        .to_string();

    // Genres
    let mut genres = Vec::new();
    if let Ok(sel) = Selector::parse(".genre-info a") {
        for el in document.select(&sel) {
            genres.push(el.text().collect::<String>().trim().to_string());
        }
    }

    // Release Date (not always available)
    let release_date = select_text(&document, "#chapter_list > ul > li:last-child > span.dt");

    // Author
    let author = select_text(&document, ".spe span:contains('Pengarang:')")
        .replace("Pengarang:", "")
        .trim()
        .to_string();

    // Type
    let r#type = select_text(&document, ".spe span:contains('Jenis Komik:') a");

    // Total Chapter
    let total_chapter = select_text(&document, "#chapter_list > ul > li:nth-child(1) > span.lchx");

    // Updated On
    let updated_on = select_text(&document, "#chapter_list > ul > li:nth-child(1) > span.dt");

    // Chapters
    let mut chapters = Vec::new();
    if let Ok(sel) = Selector::parse("#chapter_list ul li") {
        for el in document.select(&sel) {
            let chapter = select_text_el(&el, ".lchx a");
            let date = select_text_el(&el, ".dt a");
            let chapter_id = select_attr_el(&el, ".lchx a", "href")
                .split('/')
                .nth(3)
                .unwrap_or("")
                .to_string();
            chapters.push(ChapterInfo { chapter, date, chapter_id });
        }
    }

    Json(MangaDetailResponse {
        status: "Ok",
        data: MangaDetailData {
            title,
            alternative_title,
            score,
            poster,
            description,
            status,
            r#type,
            release_date,
            author,
            total_chapter,
            updated_on,
            genres,
            chapters,
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

// Utility: select attribute from document
fn select_attr(document: &Html, selector: &str, attr: &str) -> String {
    if let Ok(sel) = Selector::parse(selector) {
        document.select(&sel).next()
            .and_then(|e| e.value().attr(attr))
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

// Utility: select text from element
fn select_text_el(element: &scraper::ElementRef, selector: &str) -> String {
    if let Ok(sel) = Selector::parse(selector) {
        element.select(&sel).next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

// Utility: select attribute from element
fn select_attr_el(element: &scraper::ElementRef, selector: &str, attr: &str) -> String {
    if let Ok(sel) = Selector::parse(selector) {
        element.select(&sel).next()
            .and_then(|e| e.value().attr(attr))
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}
