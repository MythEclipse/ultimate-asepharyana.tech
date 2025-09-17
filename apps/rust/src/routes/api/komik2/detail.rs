//! Handler for the detail endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use tracing::{ info, error, warn };
use lazy_static::lazy_static;
use axum::extract::State;
use axum::http::StatusCode;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::urls::get_komik2_url;
use backoff::{ future::retry, ExponentialBackoff };
use std::time::Duration;
use deadpool_redis::redis::AsyncCommands;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik2/detail";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves details for a specific komik2 by ID.";
pub const ENDPOINT_TAG: &str = "komik2";
pub const OPERATION_ID: &str = "komik2_detail";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailData>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Chapter {
  pub chapter: String,
  pub date: String,
  pub chapter_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailData {
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
  pub chapters: Vec<Chapter>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
  pub status: bool,
  pub data: DetailData,
}

#[derive(Deserialize, ToSchema)]
pub struct DetailQuery {
  /// The unique identifier for the komik (typically the slug or URL path)
  pub komik_id: Option<String>,
}

lazy_static! {
  static ref TITLE_SELECTOR: Selector = Selector::parse("h1#Judul, h1.entry-title").unwrap();
  // The site often uses definition lists or spans with labels; we select info container rows
  static ref INFO_ROW_SELECTOR: Selector = Selector::parse(
    ".spe span, .inftable tr, .infos .infox .spe span"
  ).unwrap();
  static ref SCORE_SELECTOR: Selector = Selector::parse(".spe span").unwrap();
  static ref POSTER_SELECTOR: Selector = Selector::parse("#Imgnovel, div.ims img, .thumb img").unwrap();
  static ref DESCRIPTION_SELECTOR: Selector = Selector::parse(
    "article section p, .entry-content p, .desc p"
  ).unwrap();
  static ref GENRE_SELECTOR: Selector = Selector::parse(".genre a, ul.genre li a").unwrap();
  // Chapter meta often listed in list; keep generic selectors
  static ref CHAPTER_LIST_SELECTOR: Selector = Selector::parse(
    "table#Daftar_Chapter tbody#daftarChapter tr, #chapter_list li, .eplister ul li"
  ).unwrap();
  static ref CHAPTER_LINK_SELECTOR: Selector = Selector::parse(
    "td.judulseries a, a.chapter, a"
  ).unwrap();
  static ref DATE_LINK_SELECTOR: Selector = Selector::parse(
    "td.tanggalseries, .rightarea .date, .epcontent .date, .udate"
  ).unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("komik_id" = Option<String>, Query, description = "Comic/manga identifier", example = "sample_value")
    ),
    path = "/api/komik2/detail",
    tag = "komik2",
    operation_id = "komik2_detail",
    responses(
        (status = 200, description = "Retrieves details for a specific komik2 by ID.", body = DetailData),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
#[axum::debug_handler]
pub async fn detail(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<DetailQuery>
) -> Result<Json<DetailResponse>, (StatusCode, String)> {
  let start_time = std::time::Instant::now();
  let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
  info!("Handling request for komik2 detail: {}", komik_id);

  let cache_key = format!("komik2:detail:{}", komik_id);
  let mut conn = app_state.redis_pool.get().await.map_err(|e| {
    error!("Failed to get Redis connection: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
  })?;

  // Try to get cached data
  let cached_response: Option<String> = conn.get(&cache_key).await.map_err(|e| {
    error!("Failed to get data from Redis: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
  })?;

  if let Some(json_data_string) = cached_response {
    info!("Cache hit for key: {}", cache_key);
    let detail_response: DetailResponse = serde_json::from_str(&json_data_string).map_err(|e| {
      error!("Failed to deserialize cached data: {:?}", e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
    })?;
    return Ok(Json(detail_response));
  }

  match fetch_komik_detail(komik_id.clone()).await {
    Ok(data) => {
      let detail_response = DetailResponse { status: true, data };
      let json_data = serde_json::to_string(&detail_response).map_err(|e| {
        error!("Failed to serialize response for caching: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
      })?;

      // Store in Redis with TTL
      conn.set_ex::<_, _, ()>(&cache_key, json_data, CACHE_TTL).await.map_err(|e| {
        error!("Failed to set data in Redis: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
      })?;
      info!("Cache set for key: {}", cache_key);

      let total_duration = start_time.elapsed();
      info!("Successfully processed request for komik_id: {} in {:?}", komik_id, total_duration);
      Ok(Json(detail_response))
    }
    Err(e) => {
      let total_duration = start_time.elapsed();
      error!(
        "Failed to process request for komik_id: {} after {:?}, error: {:?}",
        komik_id,
        total_duration,
        e
      );
      Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
  }
}

async fn fetch_komik_detail(
  komik_id: String
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  let base_url = get_komik2_url();
  let url = format!("{}/manga/{}", base_url, komik_id);

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

  tokio::task::spawn_blocking(move ||
    parse_komik_detail_document(&Html::parse_document(&html), &komik_id)
  ).await?
}

fn parse_komik_detail_document(
  document: &Html,
  _komik_id: &str
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  info!("Starting to parse komik2 detail document");

  let title = document
    .select(&TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Extract labeled fields from info rows
  let mut alternative_title = String::new();
  let mut status = String::new();
  let mut r#type = String::new();
  let mut author = String::new();
  for row in document.select(&INFO_ROW_SELECTOR) {
    let txt = row.text().collect::<String>().trim().to_string();
    let lower = txt.to_lowercase();
    if
      alternative_title.is_empty() &&
      (lower.contains("judul alternatif") || lower.contains("alternative"))
    {
      alternative_title = txt
        .replace("Judul Alternatif:", "")
        .replace("Alternative:", "")
        .trim()
        .to_string();
    }
    if status.is_empty() && lower.contains("status") {
      status = txt.replace("Status:", "").trim().to_string();
    }
    if r#type.is_empty() && (lower.contains("jenis komik") || lower.contains("type")) {
      // sometimes the type anchor text is within the span
      r#type = txt.replace("Jenis Komik:", "").replace("Type:", "").trim().to_string();
    }
    if author.is_empty() && (lower.contains("pengarang") || lower.contains("author")) {
      author = txt.replace("Pengarang:", "").replace("Author:", "").trim().to_string();
    }
  }

  let score = document
    .select(&SCORE_SELECTOR)
    .find(|e| e.text().collect::<String>().to_lowercase().contains("rating"))
    .map(|e| {
      e.text()
        .collect::<String>()
        .replace("Rating:", "")
        .trim()
        .to_string()
    })
    .unwrap_or_default();

  let poster = document
    .select(&POSTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .map(|s| s.split('?').next().unwrap_or(s).to_string())
    .unwrap_or_default();

  let description = document
    .select(&DESCRIPTION_SELECTOR)
    .map(|e| e.text().collect::<String>())
    .filter(|t| t.len() > 50) // avoid tiny fragments
    .collect::<Vec<String>>()
    .join("\n")
    .trim()
    .to_string();

  // Release date, total chapter, updated_on are often within the chapter list meta; try to infer
  let mut release_date = String::new();
  let mut total_chapter = String::new();
  let mut updated_on = String::new();
  // Try read first and last items
  if let Some(first) = document.select(&CHAPTER_LIST_SELECTOR).next() {
    // updated_on or latest date
    updated_on = first
      .select(&DATE_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();
  }
  if let Some(last) = document.select(&CHAPTER_LIST_SELECTOR).last() {
    release_date = last
      .select(&DATE_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();
  }
  // Count chapters
  let count = document.select(&CHAPTER_LIST_SELECTOR).count();
  if count > 0 {
    total_chapter = count.to_string();
  }

  let mut genres = Vec::new();
  for element in document.select(&GENRE_SELECTOR) {
    genres.push(element.text().collect::<String>().trim().to_string());
  }

  let mut chapters = Vec::new();
  for el in document.select(&CHAPTER_LIST_SELECTOR) {
    let chapter = el
      .select(&CHAPTER_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();
    let date = el
      .select(&DATE_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();
    let chapter_id = el
      .select(&CHAPTER_LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .map(|href| {
        let parts: Vec<&str> = href
          .split('/')
          .filter(|s| !s.is_empty())
          .collect();
        parts.last().cloned().unwrap_or("").to_string()
      })
      .unwrap_or_default();
    chapters.push(Chapter { chapter, date, chapter_id });
  }

  let duration = start_time.elapsed();
  info!("Parsed komik2 detail document in {:?}", duration);

  Ok(DetailData {
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
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(detail))
}