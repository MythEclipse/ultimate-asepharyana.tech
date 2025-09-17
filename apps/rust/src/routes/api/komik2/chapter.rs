//! Handler for the komik2 chapter endpoint.
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
pub const ENDPOINT_PATH: &str = "/api/komik2/chapter";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves chapter data for a specific komik2 chapter.";
pub const ENDPOINT_TAG: &str = "komik2";
pub const OPERATION_ID: &str = "komik2_chapter";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ChapterResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChapterData {
  pub title: String,
  pub next_chapter_id: String,
  pub prev_chapter_id: String,
  pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChapterResponse {
  pub message: String,
  pub data: ChapterData,
}

#[derive(Deserialize, ToSchema)]
pub struct ChapterQuery {
  /// URL-friendly identifier for the chapter (typically the chapter slug or URL path)
  pub chapter_url: Option<String>,
}

lazy_static! {
  static ref TITLE_SELECTOR: Selector = Selector::parse("title").unwrap();
  static ref PREV_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".navi-chapter.prev a, .chapter-nav.prev a, .prev-chapter a"
  ).unwrap();
  static ref LIST_CHAPTER_SELECTOR: Selector = Selector::parse("table#Daftar_Chapter tbody tr").unwrap();
  static ref NEXT_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".navi-chapter.next a, .chapter-nav.next a, .next-chapter a"
  ).unwrap();
  static ref IMAGE_SELECTOR: Selector = Selector::parse(
    "img[data-src], img[src], #Baca_Komik img, .entry-content img"
  ).unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("chapter_url" = Option<String>, Query, description = "Chapter-specific identifier", example = "sample_value")
    ),
    path = "/api/komik2/chapter",
    tag = "komik2",
    operation_id = "komik2_chapter",
    responses(
        (status = 200, description = "Retrieves chapter data for a specific komik2 chapter.", body = ChapterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
#[axum::debug_handler]
pub async fn chapter(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<ChapterQuery>
) -> Result<Json<ChapterResponse>, (StatusCode, String)> {
  let start_time = std::time::Instant::now();
  let chapter_url = params.chapter_url.unwrap_or_default();
  info!("Handling request for komik2 chapter: {}", chapter_url);

  let cache_key = format!("komik2:chapter:{}", chapter_url);
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
    let chapter_response: ChapterResponse = serde_json::from_str(&json_data_string).map_err(|e| {
      error!("Failed to deserialize cached data: {:?}", e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
    })?;
    return Ok(Json(chapter_response));
  }

  match fetch_komik2_chapter(chapter_url.clone()).await {
    Ok(data) => {
      let chapter_response = ChapterResponse { message: "Ok".to_string(), data };
      let json_data = serde_json::to_string(&chapter_response).map_err(|e| {
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
      info!(
        "Successfully processed request for chapter_url: {} in {:?}",
        chapter_url,
        total_duration
      );
      Ok(Json(chapter_response))
    }
    Err(e) => {
      let total_duration = start_time.elapsed();
      error!(
        "Failed to process request for chapter_url: {} after {:?}, error: {:?}",
        chapter_url,
        total_duration,
        e
      );
      Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
  }
}

async fn fetch_komik2_chapter(
  chapter_url: String
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  let base_url = get_komik2_url();
  let url = format!("{}/{}", base_url, chapter_url); // Keep as-is since chapter URLs might already have correct format

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
        let _duration = start_time.elapsed();
        info!("Successfully fetched URL: {}", url);
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
    parse_komik2_chapter_document(&Html::parse_document(&html), &chapter_url)
  ).await?
}

fn parse_komik2_chapter_document(
  document: &Html,
  _chapter_url: &str
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  info!("Starting to parse komik2 chapter document");

  let title = document
    .select(&TITLE_SELECTOR)
    .next()
    .map(|e| {
      let full_title = e.text().collect::<String>();
      // Extract series title from "Chapter XX | Komik TITLE - Komiku"
      if let Some(start) = full_title.find("Komik ") {
        if let Some(end) = full_title.find(" - Komiku") {
          full_title[start + 6..end].trim().to_string()
        } else {
          full_title
        }
      } else {
        full_title
      }
    })
    .unwrap_or_default();

  let prev_chapter_id = document
    .select(&PREV_CHAPTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("href"))
    .map(|href|
      href
        .trim_end_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap_or("")
        .to_string()
    )
    .unwrap_or_default();

  let next_chapter_id = document
    .select(&NEXT_CHAPTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("href"))
    .map(|href|
      href
        .trim_end_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap_or("")
        .to_string()
    )
    .unwrap_or_default();

  let mut images = Vec::new();
  for el in document.select(&IMAGE_SELECTOR) {
    if
      let Some(src) = el
        .value()
        .attr("src")
        .or_else(|| el.value().attr("data-src"))
        .or_else(|| el.value().attr("data-lazy-src"))
        .or_else(||
          el
            .value()
            .attr("srcset")
            .and_then(|s| s.split_whitespace().next())
        )
    {
      images.push(src.to_string());
    }
  }

  let _duration = start_time.elapsed();
  info!("Parsed komik2 chapter document in {:?}", _duration);

  Ok(ChapterData {
    title,
    next_chapter_id,
    prev_chapter_id,
    images,
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(chapter))
}