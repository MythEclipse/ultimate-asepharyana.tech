//! Handler for the detail endpoint.

use crate::helpers::{get_cached_or_original, internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::helpers::scraping::{selector, text_from_or, text, attr};
use crate::routes::AppState;
use crate::scraping::urls::get_komik_url;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::Response,
    Json, Router,
};

use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};
use utoipa::ToSchema;

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

static CHAPTER_TITLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(?:chapter|ch\.?)\s*([\d\.]+)").unwrap());
static CHAPTER_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([\d\.]+)").unwrap());

// Helper function to find table rows containing specific text
fn find_table_row_with_text<'a>(
    info_rows: &[scraper::ElementRef<'a>], 
    text_fragments: &[&str],
) -> Option<String> {
    let lower_text_fragments: Vec<String> =
        text_fragments.iter().map(|&s| s.to_lowercase()).collect();
    let td_last_selector = selector("td:last-child").unwrap();

    info_rows
        .iter() 
        .find(|row| {
            let row_text = text(row).to_lowercase();
            lower_text_fragments
                .iter()
                .any(|fragment| row_text.contains(fragment))
        })
        .and_then(|row| {
            text_from_or(row, &td_last_selector, "").trim().to_string().into()
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
    let _start_time = std::time::Instant::now();
    let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
    info!("Handling request for komik detail: {}", komik_id);

    let cache_key = format!("komik:detail:{}", komik_id);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let mut data = fetch_komik_detail(komik_id.clone())
                .await
                .map_err(|e| e.to_string())?;

            // Cache poster image
            if !data.poster.is_empty() {
                data.poster = get_cached_or_original(
                    app_state.db.clone(),
                    &app_state.redis_pool,
                    &data.poster,
                    Some(app_state.image_processing_semaphore.clone()),
                )
                .await;
            }

            Ok(DetailResponse { status: true, data })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response))
}

async fn fetch_komik_detail(
    komik_id: String,
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
    let base_url = get_komik_url();
    let url = format!("{}/manga/{}/", base_url, komik_id); 

    let html = fetch_html_with_retry(&url).await?;

    tokio::task::spawn_blocking(move || parse_komik_detail_document(&html))
        .await?
}

fn parse_komik_detail_document(
    html: &str,
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    info!("Starting to parse komik detail document");
    
    let document = parse_html(html);

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

    let title_selector = selector("div#Judul h1 span[itemprop=\"name\"]").unwrap();
    let h1_selector = selector("h1").unwrap();
    let title_tag_selector = selector("title").unwrap();
    let info_row_selector = selector("table.inftable tr").unwrap();
    let poster_selector = selector("section#Informasi .ims img").unwrap();
    let desc_selector = selector("p.desc").unwrap();
    let chapter_list_selector = selector("tbody#daftarChapter tr").unwrap();
    let date_link_selector = selector("td.tanggalseries").unwrap();
    let judul2_selector = selector("div.judul2").unwrap();
    let genre_selector = selector("ul.genre li a").unwrap();
    let chapter_link_selector = selector("td.judulseries a").unwrap();

    // Improved title extraction with fallback options
    let title = document
        .select(&title_selector)
        .next()
        .map(|e| {
            let text = clean_text(text(&e));
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
                .select(&h1_selector)
                .next()
                .map(|e| clean_text(text(&e)))
        })
        .or_else(|| {
            // Final fallback to document title
            document
                .select(&title_tag_selector)
                .next()
                .map(|e| {
                    let text = clean_text(text(&e));
                    if text.contains("Komik ") {
                        text.replace("Komik ", "").trim().to_string()
                    } else {
                        text.trim().to_string()
                    }
                })
        })
        .unwrap_or_default();

    let info_rows_vec: Vec<scraper::ElementRef> = document
        .select(&info_row_selector)
        .collect();
    let info_rows = &info_rows_vec[..]; 

    let status = info_rows
        .iter()
        .find_map(|&row| {
            let full_text = text(&row);
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
            let full_text = text(&row);
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
            let full_text = text(&row);
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
        .select(&poster_selector)
        .next()
        .and_then(|e| attr(&e, "src"))
        .map(|s| s.split('?').next().unwrap_or(&s).to_string())
        .unwrap_or_default();

    let description = document
        .select(&desc_selector)
        .map(|e| clean_text(text(&e)))
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
                .select(&chapter_list_selector)
                .next_back()
                .and_then(|last| {
                    last.select(&date_link_selector)
                        .next()
                })
                .map(|e| clean_text(text(&e)))
                .unwrap_or_default()
        });

    let total_chapter = find_table_row_with_text(info_rows, &["total chapter", "total chapters"])
        .unwrap_or_else(|| {
            // Fallback to chapter count if no specific total found
            let count = document
                .select(&chapter_list_selector)
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
                .select(&judul2_selector)
                .next()
                .map(|e| {
                    let text_str = clean_text(text(&e));
                    // Extract the part after "• " which contains the time information
                    text_str.split("• ").nth(1).unwrap_or("").trim().to_string()
                })
        })
        .unwrap_or_else(|| {
            // Fallback to first chapter date if no specific updated date found
            document
                .select(&chapter_list_selector)
                .next()
                .and_then(|first| {
                    first
                        .select(&date_link_selector)
                        .next()
                })
                .map(|e| clean_text(text(&e)))
                .unwrap_or_default()
        });

    let mut genres = Vec::new();
    for element in document.select(&genre_selector) {
        let genre = clean_text(text(&element));
        if !genre.is_empty() {
            genres.push(genre);
        }
    }

    // Optimized chapter parsing with refined selectors
    let raw_chapter_data: Vec<(String, String, String)> = document
        .select(&chapter_list_selector)
        .filter_map(|el| {
            let chapter_link_element = el
                .select(&chapter_link_selector)
                .next();
            let date_element = el
                .select(&date_link_selector)
                .next();

            let chapter_text = chapter_link_element
                .as_ref()
                .map(|e| clean_text(text(e)))
                .unwrap_or_default();

            let date_text = date_element
                .map(|e| clean_text(text(&e)))
                .unwrap_or_default();

            let href_text = chapter_link_element
                .and_then(|e| attr(&e, "href"))

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

#[utoipa::path(
    get,
    path = "/api/komik/detail/ws",
    tag = "komik",
    operation_id = "komik_detail_ws",
    responses(
        (status = 101, description = "WebSocket upgrade")
    )
)]
pub async fn ws_handler(ws: WebSocketUpgrade, State(app_state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

async fn handle_socket(mut socket: WebSocket, app_state: Arc<AppState>) {
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
                                // Cache poster image
                                if !detail_data.poster.is_empty() {
                                    detail_data.poster = get_cached_or_original(
                                        app_state.db.clone(),
                                        &app_state.redis_pool,
                                        &detail_data.poster,
                                        Some(app_state.image_processing_semaphore.clone()),
                                    )
                                    .await;
                                }

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
}