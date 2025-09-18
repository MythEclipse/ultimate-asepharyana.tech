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
use backoff::{ future::retry, ExponentialBackoff };
use std::time::Duration;
use deadpool_redis::redis::AsyncCommands;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/detail";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves details for a specific komik by ID.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_detail";
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
  static ref TITLE_SELECTOR: Selector = Selector::parse("h1.entry-title").unwrap();
  static ref ALTERNATIVE_TITLE_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Judul Alternatif:')"
  ).unwrap();
  static ref SCORE_SELECTOR: Selector = Selector::parse(".rtg > div > i").unwrap();
  static ref POSTER_SELECTOR: Selector = Selector::parse(".thumb img").unwrap();
  static ref DESCRIPTION_SELECTOR: Selector = Selector::parse(
    "#sinopsis > section > div > div.entry-content.entry-content-single > p"
  ).unwrap();
  static ref STATUS_SELECTOR: Selector = Selector::parse(".spe span:contains('Status:')").unwrap();
  static ref GENRE_SELECTOR: Selector = Selector::parse(".genre-info a").unwrap();
  static ref RELEASE_DATE_SELECTOR: Selector = Selector::parse(
    "#chapter_list > ul > li:last-child > span.dt"
  ).unwrap();
  static ref AUTHOR_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Pengarang:')"
  ).unwrap();
  static ref TYPE_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Jenis Komik:') a"
  ).unwrap();
  static ref TOTAL_CHAPTER_SELECTOR: Selector = Selector::parse(
    "#chapter_list > ul > li:nth-child(1) > span.lchx"
  ).unwrap();
  static ref UPDATED_ON_SELECTOR: Selector = Selector::parse(
    "#chapter_list > ul > li:nth-child(1) > span.dt"
  ).unwrap();
  static ref CHAPTER_LIST_SELECTOR: Selector = Selector::parse("#chapter_list ul li").unwrap();
  static ref CHAPTER_LINK_SELECTOR: Selector = Selector::parse(".lchx a").unwrap();
  static ref DATE_LINK_SELECTOR: Selector = Selector::parse(".dt a").unwrap();
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
#[axum::debug_handler]
pub async fn detail(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<DetailQuery>
) -> Result<Json<DetailResponse>, (StatusCode, String)> {
  let start_time = std::time::Instant::now();
  let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
  info!("Handling request for komik detail: {}", komik_id);

  let cache_key = format!("komik:detail:{}", komik_id);
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
  let base_url = "https://komikindo.ch"; // Updated as per user feedback
  let url = format!("{}/komik/{}/", base_url, komik_id);

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
  info!("Starting to parse komik detail document");

  let title = document
    .select(&TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let alternative_title = document
    .select(&ALTERNATIVE_TITLE_SELECTOR)
    .next()
    .map(|e| { e.text().collect::<String>().replace("Judul Alternatif:", "").trim().to_string() })
    .unwrap_or_default();

  let score = document
    .select(&SCORE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let poster = document
    .select(&POSTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .map(|s| s.split('?').next().unwrap_or(s).to_string())
    .unwrap_or_default();

  let description = document
    .select(&DESCRIPTION_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let status = document
    .select(&STATUS_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().replace("Status:", "").trim().to_string())
    .unwrap_or_default();

  let r#type = document
    .select(&TYPE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let release_date = document
    .select(&RELEASE_DATE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let author = document
    .select(&AUTHOR_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().replace("Pengarang:", "").trim().to_string())
    .unwrap_or_default();

  let total_chapter = document
    .select(&TOTAL_CHAPTER_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let updated_on = document
    .select(&UPDATED_ON_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

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
      .and_then(|href| href.split('/').nth(3)) // Adjust index based on actual URL structure
      .unwrap_or("")
      .to_string();
    chapters.push(Chapter { chapter, date, chapter_id });
  }

  let duration = start_time.elapsed();
  info!("Parsed komik detail document in {:?}", duration);

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