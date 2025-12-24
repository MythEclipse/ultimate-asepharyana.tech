//! Handler for the detail endpoint.

use crate::fetch_with_proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::urls::get_komik_url;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{Response},
    routing::{get},
    Json, Router,
};
use backoff::{future::retry, ExponentialBackoff};
use deadpool_redis::redis::AsyncCommands;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/detail";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves details for a specific komik by ID.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_detail";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Chapter {
    pub chapter: String,
    pub date: String,
    pub chapter_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailData {
    pub title: String,
    pub poster: String,
    pub description: String,
    pub status: String,
    pub r#type: String,
    pub release_date: String,
    pub author: String,
    pub total_chapter: String,
    pub updated_on: String,
    pub genres: Vec<String>,
    pub chapters: Vec<Chapter>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
    pub status: bool,
    pub data: DetailData,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct KomikDetailRequest {
    pub komik_id: String,
    pub chapter_id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub enum KomikDetailEvent {
    Chapter(Chapter),
    Detail(DetailData),
    Error(String),
    EndOfStream,
}

#[derive(Deserialize, ToSchema)]
pub struct DetailQuery {
    /// The unique identifier for the komik (typically the slug or URL path)
    pub komik_id: Option<String>,
}

// Define selectors using once_cell::sync::Lazy for efficient initialization
static TITLE_SELECTOR: Lazy<String> =
    Lazy::new(|| r#"div#Judul h1 span[itemprop="name"]"#.to_string());
static INFO_ROW_SELECTOR: Lazy<String> = Lazy::new(|| "table.inftable tr".to_string());
static POSTER_SELECTOR: Lazy<String> = Lazy::new(|| r#"section#Informasi .ims img"#.to_string());
static DESCRIPTION_SELECTOR: Lazy<String> = Lazy::new(|| "p.desc".to_string());
static GENRE_SELECTOR: Lazy<String> = Lazy::new(|| "ul.genre li a".to_string());
static CHAPTER_LIST_SELECTOR: Lazy<String> = Lazy::new(|| "tbody#daftarChapter tr".to_string());
static CHAPTER_LINK_SELECTOR: Lazy<String> = Lazy::new(|| "td.judulseries a".to_string());
static DATE_LINK_SELECTOR: Lazy<String> = Lazy::new(|| "td.tanggalseries".to_string());
static JUDUL2_SELECTOR: Lazy<String> = Lazy::new(|| "div.judul2".to_string());
static CHAPTER_TITLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(?:chapter|ch\.?)\s*([\d\.]+)").unwrap());
static CHAPTER_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([\d\.]+)").unwrap());

// Helper function to find table rows containing specific text
fn find_table_row_with_text<'a>(
    info_rows: &[scraper::ElementRef<'a>], // Changed to accept a slice
    text_fragments: &[&str],
) -> Option<String> {
    let lower_text_fragments: Vec<String> =
        text_fragments.iter().map(|&s| s.to_lowercase()).collect();

    info_rows
        .iter() // Iterate over the slice
        .find(|row| {
            let row_text = row.text().collect::<String>();
            lower_text_fragments
                .iter()
                .any(|fragment| row_text.to_lowercase().contains(fragment))
        })
        .and_then(|row| {
            row.select(&Selector::parse("td:last-child").unwrap())
                .next()
                .map(|cell| cell.text().collect::<String>().trim().to_string())
        })
}

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("komik_id" = Option<String>, Query, description = "Comic/manga identifier", example = "sample_value")
    ),
    path = "/api/komik/detail",
    tag = "komik",
    operation_id = "komik_detail",
    responses(
        (status = 200, description = "Retrieves details for a specific komik by ID.", body = DetailData),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn detail(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<DetailQuery>,
) -> Result<Json<DetailResponse>, (StatusCode, String)> {
    let start_time = std::time::Instant::now();
    let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
    info!("Handling request for komik detail: {}", komik_id);

    let cache_key = format!("komik:detail:{}", komik_id);
    let mut conn = app_state.redis_pool.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Redis error: {}", e),
        )
    })?;

    // Try to get cached data
    let cached_response: Option<String> = conn.get(&cache_key).await.map_err(|e| {
        error!("Failed to get data from Redis: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Redis error: {}", e),
        )
    })?;

    if let Some(json_data_string) = cached_response {
        info!("Cache hit for key: {}", cache_key);
        let detail_response: DetailResponse =
            serde_json::from_str(&json_data_string).map_err(|e| {
                error!("Failed to deserialize cached data: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Serialization error: {}", e),
                )
            })?;
        return Ok(Json(detail_response));
    }

    match fetch_komik_detail(komik_id.clone()).await {
        Ok(data) => {
            // Validate that we got meaningful data
            let is_valid_response =
                !data.title.is_empty() || !data.chapters.is_empty() || !data.description.is_empty();

            if !is_valid_response {
                error!(
                    "Received empty response for komik_id: {}. All fields are empty.",
                    komik_id
                );
                return Err((
                    StatusCode::NOT_FOUND,
                    "No data found for this komik".to_string(),
                ));
            }

            let detail_response = DetailResponse { status: true, data };
            let json_data = serde_json::to_string(&detail_response).map_err(|e| {
                error!("Failed to serialize response for caching: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Serialization error: {}", e),
                )
            })?;

            // Store in Redis with TTL
            conn.set_ex::<_, _, ()>(&cache_key, json_data, CACHE_TTL)
                .await
                .map_err(|e| {
                    error!("Failed to set data in Redis: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Redis error: {}", e),
                    )
                })?;
            info!("Cache set for key: {}", cache_key);

            let total_duration = start_time.elapsed();
            info!(
                "Successfully processed request for komik_id: {} in {:?}",
                komik_id, total_duration
            );
            Ok(Json(detail_response))
        }
        Err(e) => {
            let total_duration = start_time.elapsed();
            error!(
                "Failed to process request for komik_id: {} after {:?}, error: {:?}",
                komik_id, total_duration, e
            );

            // Provide more specific error messages
            let error_msg = match e.to_string().as_str() {
                "Empty response" => "No data received from the source website".to_string(),
                "Failed to fetch" => "Failed to connect to the source website".to_string(),
                "Timeout" => "Request timed out while connecting to source website".to_string(),
                _ => format!("Failed to fetch komik data: {}", e),
            };

            Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
        }
    }
}

async fn fetch_komik_detail(
    komik_id: String,
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    let base_url = get_komik_url();
    let url = format!("{}/manga/{}/", base_url, komik_id); // Correct URL format for komiku.org

    // Retry logic with exponential backoff
    let backoff = ExponentialBackoff {
        initial_interval: Duration::from_millis(500),
        max_interval: Duration::from_secs(10),
        multiplier: 2.0,
        max_elapsed_time: Some(Duration::from_secs(30)),
        ..Default::default()
    };

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => {
                let duration = start_time.elapsed();
                info!("Successfully fetched URL: {} in {:?}", url, duration);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch URL: {}, error: {:?}", url, e);
                Err(backoff::Error::transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;

    tokio::task::spawn_blocking(move || parse_komik_detail_document(&Html::parse_document(&html)))
        .await?
}

fn parse_komik_detail_document(
    document: &Html,
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    info!("Starting to parse komik detail document");

    // Helper function to clean and format extracted text
    fn clean_text(text: String) -> String {
        text.replace(['\n', '\t'], " ").trim().to_string()
    }

    // Helper function to extract text after a keyword or colon
    fn extract_value_after_keyword(
        full_text: &str,
        keywords: &[&str],
        default_index: usize,
    ) -> String {
        let lower_text = full_text.to_lowercase();
        for keyword in keywords {
            if let Some(pos) = lower_text.find(&format!("{}:", keyword)) {
                return clean_text(full_text[pos + keyword.len() + 1..].to_string());
            } else if let Some(pos) = lower_text.find(&format!("{} ", keyword)) {
                return clean_text(full_text[pos + keyword.len() + 1..].to_string());
            }
        }
        if let Some(colon_pos) = full_text.find(':') {
            return clean_text(full_text[colon_pos + 1..].to_string());
        }
        clean_text(full_text[default_index..].to_string())
    }

    // Improved title extraction with fallback options
    let title = document
        .select(&Selector::parse(&TITLE_SELECTOR).unwrap())
        .next()
        .map(|e| {
            let text = clean_text(e.text().collect::<String>());
            // Remove common prefixes/suffixes that might be included
            text.replace("Komik ", "")
                .replace("Manga ", "")
                .replace("Manhua ", "")
                .replace("Manhwa ", "")
                .trim()
                .to_string()
        })
        .or_else(|| {
            // Fallback to try to extract title from h1 elements
            document
                .select(&Selector::parse("h1").unwrap())
                .next()
                .map(|e| clean_text(e.text().collect::<String>()))
        })
        .or_else(|| {
            // Final fallback to document title
            document
                .select(&Selector::parse("title").unwrap())
                .next()
                .map(|e| {
                    let text = clean_text(e.text().collect::<String>());
                    if text.contains("Komik ") {
                        text.replace("Komik ", "").trim().to_string()
                    } else {
                        text.trim().to_string()
                    }
                })
        })
        .unwrap_or_default();

    let info_rows_vec: Vec<scraper::ElementRef> = document
        .select(&Selector::parse(&INFO_ROW_SELECTOR).unwrap())
        .collect();
    let info_rows = &info_rows_vec[..]; // Create a slice from the Vec

    let status = info_rows
        .iter()
        .find_map(|&row| {
            let full_text = row.text().collect::<String>();
            if full_text.to_lowercase().contains("status") {
                Some(
                    extract_value_after_keyword(&full_text, &["status"], 0)
                        .replace("Status", "")
                        .replace("Jenis Komik", "")
                        .replace("Type", "")
                        .trim()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_default();

    let r#type = info_rows
        .iter()
        .find_map(|&row| {
            let full_text = row.text().collect::<String>();
            if full_text.to_lowercase().contains("jenis komik")
                || full_text.to_lowercase().contains("type")
            {
                Some(
                    extract_value_after_keyword(&full_text, &["jenis komik", "type"], 0)
                        .replace("Jenis Komik", "")
                        .replace("Type", "")
                        .trim()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_default();

    let author = info_rows
        .iter()
        .find_map(|&row| {
            let full_text = row.text().collect::<String>();
            if full_text.to_lowercase().contains("pengarang")
                || full_text.to_lowercase().contains("author")
                || full_text.to_lowercase().contains("artist")
            {
                Some(
                    extract_value_after_keyword(&full_text, &["pengarang", "author", "artist"], 0)
                        .replace("Pengarang", "")
                        .replace("Author", "")
                        .replace("pengarang", "")
                        .replace("author", "")
                        .replace("Artist", "")
                        .replace("artist", "")
                        .trim()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_default();

    let poster = document
        .select(&Selector::parse(&POSTER_SELECTOR).unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .map(|s| s.split('?').next().unwrap_or(s).to_string())
        .unwrap_or_default();

    let description = document
        .select(&Selector::parse(&DESCRIPTION_SELECTOR).unwrap())
        .map(|e| clean_text(e.text().collect::<String>()))
        .filter(|t| t.len() > 50) // avoid tiny fragments
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_string();

    // Extract release date, total chapter, and updated_on using specific selectors first
    let release_date = find_table_row_with_text(info_rows, &["tanggal rilis", "release date"])
        .map(clean_text)
        .unwrap_or_else(|| {
            // Fallback to last chapter date if no specific release date found
            document
                .select(&Selector::parse(&CHAPTER_LIST_SELECTOR).unwrap())
                .next_back()
                .and_then(|last| {
                    last.select(&Selector::parse(&DATE_LINK_SELECTOR).unwrap())
                        .next()
                })
                .map(|e| clean_text(e.text().collect::<String>()))
                .unwrap_or_default()
        });

    let total_chapter = find_table_row_with_text(info_rows, &["total chapter", "total chapters"])
        .unwrap_or_else(|| {
            // Fallback to chapter count if no specific total found
            let count = document
                .select(&Selector::parse(&CHAPTER_LIST_SELECTOR).unwrap())
                .count();
            if count > 0 {
                count.to_string()
            } else {
                String::new()
            }
        });

    let updated_on = find_table_row_with_text(info_rows, &["diperbarui", "updated"])
        .or_else(|| {
            // Look for updated date in the "judul2" class which contains "pembaca • X waktu lalu"
            document
                .select(&Selector::parse(&JUDUL2_SELECTOR).unwrap())
                .next()
                .map(|e| {
                    let text = clean_text(e.text().collect::<String>());
                    // Extract the part after "• " which contains the time information
                    text.split("• ").nth(1).unwrap_or("").trim().to_string()
                })
        })
        .unwrap_or_else(|| {
            // Fallback to first chapter date if no specific updated date found
            document
                .select(&Selector::parse(&CHAPTER_LIST_SELECTOR).unwrap())
                .next()
                .and_then(|first| {
                    first
                        .select(&Selector::parse(&DATE_LINK_SELECTOR).unwrap())
                        .next()
                })
                .map(|e| clean_text(e.text().collect::<String>()))
                .unwrap_or_default()
        });

    let mut genres = Vec::new();
    for element in document.select(&Selector::parse(&GENRE_SELECTOR).unwrap()) {
        let genre = clean_text(element.text().collect::<String>());
        if !genre.is_empty() {
            genres.push(genre);
        }
    }

    // Optimized chapter parsing with refined selectors
    let raw_chapter_data: Vec<(String, String, String)> = document
        .select(&Selector::parse(&CHAPTER_LIST_SELECTOR).unwrap())
        .filter_map(|el| {
            let chapter_link_element = el
                .select(&Selector::parse(&CHAPTER_LINK_SELECTOR).unwrap())
                .next();
            let date_element = el
                .select(&Selector::parse(&DATE_LINK_SELECTOR).unwrap())
                .next();

            let chapter_text = chapter_link_element
                .as_ref()
                .map(|e| clean_text(e.text().collect::<String>()))
                .unwrap_or_default();

            let date_text = date_element
                .map(|e| clean_text(e.text().collect::<String>()))
                .unwrap_or_default();

            let href_text = chapter_link_element
                .and_then(|e| e.value().attr("href"))
                .map(|s| s.to_string())
                .unwrap_or_default();

            if !chapter_text.is_empty() || !date_text.is_empty() || !href_text.is_empty() {
                Some((chapter_text, date_text, href_text))
            } else {
                None
            }
        })
        .collect();

    let chapters: Vec<Chapter> = raw_chapter_data
        .par_iter()
        .filter_map(|(chapter_text, date_text, href_text)| {
            let chapter = {
                let trimmed_chapter_text = chapter_text.trim();
                // Try to find "Chapter N", "Ch. N", or just "N"
                if let Some(captures) = CHAPTER_TITLE_REGEX.captures(trimmed_chapter_text) {
                    captures
                        .get(1)
                        .map_or(trimmed_chapter_text.to_string(), |m| m.as_str().to_string())
                } else if let Some(captures) = CHAPTER_NUMBER_REGEX.captures(trimmed_chapter_text) {
                    captures
                        .get(1)
                        .map_or(trimmed_chapter_text.to_string(), |m| m.as_str().to_string())
                } else {
                    trimmed_chapter_text.to_string()
                }
            };

            let date = date_text.trim().to_string();

            let chapter_id = href_text
                .split('/')
                .filter(|s| !s.is_empty())
                .next_back()
                .unwrap_or("")
                .to_string();

            if !chapter_id.is_empty() {
                Some(Chapter {
                    chapter,
                    date,
                    chapter_id,
                })
            } else {
                None
            }
        })
        .collect();

    let duration = start_time.elapsed();
    info!("Parsed komik detail document in {:?}", duration);

    Ok(DetailData {
        title,
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
    })
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(app_state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

async fn handle_socket(mut socket: WebSocket, _: Arc<AppState>) {
    info!("WebSocket connection established.");

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            error!("WebSocket client disconnected with error.");
            return;
        };

        match msg {
            Message::Text(text) => {
                info!("Received WebSocket text message: {}", text);
                let request: Result<KomikDetailRequest, serde_json::Error> =
                    serde_json::from_str(&text);

                match request {
                    Ok(req) => {
                        // Removed chapter image streaming logic
                        info!("Processing detail request for komik_id: {}", req.komik_id);
                        let komik_id = req.komik_id.clone();

                        // Fetch detail data
                        match fetch_komik_detail(komik_id).await {
                            Ok(mut detail_data) => {
                                // Send initial detail data (without chapters)
                                let chapters_to_send =
                                    detail_data.chapters.drain(..).collect::<Vec<_>>(); // Temporarily remove chapters
                                let initial_detail_event = KomikDetailEvent::Detail(detail_data);
                                if socket
                                    .send(Message::Text(
                                        serde_json::to_string(&initial_detail_event)
                                            .unwrap()
                                            .into(),
                                    ))
                                    .await
                                    .is_err()
                                {
                                    warn!("Client disconnected during initial detail send.");
                                    return;
                                }

                                // Send chapters one by one
                                for chapter in chapters_to_send {
                                    let chapter_event = KomikDetailEvent::Chapter(chapter);
                                    if socket
                                        .send(Message::Text(
                                            serde_json::to_string(&chapter_event).unwrap().into(),
                                        ))
                                        .await
                                        .is_err()
                                    {
                                        warn!("Client disconnected during chapter stream.");
                                        return;
                                    }
                                    // Small delay to simulate real-time parsing and prevent overwhelming the client
                                    tokio::time::sleep(Duration::from_millis(50)).await;
                                }

                                // Signal end of stream
                                if socket
                                    .send(Message::Text(
                                        serde_json::to_string(&KomikDetailEvent::EndOfStream)
                                            .unwrap()
                                            .into(),
                                    ))
                                    .await
                                    .is_err()
                                {
                                    warn!("Client disconnected before EndOfStream.");
                                    return;
                                }
                            }
                            Err(e) => {
                                error!("Failed to fetch komik detail for WS: {:?}", e);
                                let error_event = KomikDetailEvent::Error(format!(
                                    "Failed to fetch detail: {}",
                                    e
                                ));
                                if socket
                                    .send(Message::Text(
                                        serde_json::to_string(&error_event).unwrap().into(),
                                    ))
                                    .await
                                    .is_err()
                                {
                                    warn!("Client disconnected during error send.");
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse WebSocket request: {:?}", e);
                        let error_event =
                            KomikDetailEvent::Error(format!("Invalid request format: {}", e));
                        if socket
                            .send(Message::Text(
                                serde_json::to_string(&error_event).unwrap().into(),
                            ))
                            .await
                            .is_err()
                        {
                            warn!("Client disconnected during invalid request error send.");
                            return;
                        }
                    }
                }
            }
            Message::Binary(_) => {
                warn!("Received unexpected binary message.");
                if socket
                    .send(Message::Text(
                        "Error: Binary messages not supported.".to_string().into(),
                    ))
                    .await
                    .is_err()
                {
                    warn!("Client disconnected during binary message error.");
                    return;
                }
            }
            Message::Ping(ping) => {
                info!("Received ping message.");
                if socket.send(Message::Pong(ping)).await.is_err() {
                    warn!("Failed to send pong message, client likely disconnected.");
                    return;
                }
            }
            Message::Pong(_) => {
                info!("Received pong message.");
            }
            Message::Close(cf) => {
                info!("Received close message: {:?}", cf);
                if let Some(close_frame) = cf {
                    warn!(
                        "WebSocket connection closed with code: {}",
                        close_frame.code
                    );
                } else {
                    warn!("WebSocket connection closed.");
                }
                return;
            }
        }
    }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route(ENDPOINT_PATH, get(detail))
        .route("/ws/komik/detail", get(ws_handler))
}

