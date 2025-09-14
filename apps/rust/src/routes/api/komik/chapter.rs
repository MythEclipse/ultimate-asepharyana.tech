//! Handler for the chapter endpoint.
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
use tokio::sync::Mutex as TokioMutex;
use backoff::{ future::retry, ExponentialBackoff };
use dashmap::DashMap;
use std::time::{ Duration, Instant };

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/chapter";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves chapter data for a specific komik chapter.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_chapter";
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
  static ref TITLE_SELECTOR: Selector = Selector::parse(".entry-title").unwrap();
  static ref PREV_CHAPTER_SELECTOR: Selector = Selector::parse(".nextprev a[rel=\"prev\"]").unwrap();
  static ref LIST_CHAPTER_SELECTOR: Selector = Selector::parse(".nextprev a:has(.icol.daftarch)").unwrap();
  static ref NEXT_CHAPTER_SELECTOR: Selector = Selector::parse(".nextprev a[rel=\"next\"]").unwrap();
  static ref IMAGE_SELECTOR: Selector = Selector::parse("#chimg-auh img").unwrap();
  static ref HTML_CACHE: DashMap<String, (String, Instant)> = DashMap::new();
}
const CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

#[utoipa::path(
    get,
    params(
        ("chapter_url" = Option<String>, Query, description = "Chapter-specific identifier", example = "sample_value")
    ),
    path = "/api/komik/chapter",
    tag = "komik",
    operation_id = "komik_chapter",
    responses(
        (status = 200, description = "Retrieves chapter data for a specific komik chapter.", body = ChapterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
#[axum::debug_handler]
pub async fn chapter(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<ChapterQuery>
) -> Result<Json<ChapterResponse>, (StatusCode, String)> {
  let start_time = Instant::now();
  let chapter_url = params.chapter_url.unwrap_or_default();
  info!("Handling request for komik chapter: {}", chapter_url);

  match fetch_komik_chapter(&Arc::new(TokioMutex::new(())), chapter_url.clone()).await {
    Ok(data) => {
      let total_duration = start_time.elapsed();
      info!(
        "Successfully processed request for chapter_url: {} in {:?}",
        chapter_url,
        total_duration
      );
      Ok(
        Json(ChapterResponse {
          message: "Ok".to_string(),
          data,
        })
      )
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

async fn fetch_komik_chapter(
  browser_client: &Arc<TokioMutex<()>>,
  chapter_url: String
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = Instant::now();
  let base_url = "https://komikindo.ch"; // Updated as per user feedback
  let url = format!("{}/chapter/{}", base_url, chapter_url);

  // Check cache first
  if let Some(entry) = HTML_CACHE.get(&url) {
    if entry.1.elapsed() < CACHE_TTL {
      info!("Cache hit for URL: {}", url);
      let entry_0_clone = entry.0.clone();
      let chapter_url_clone = chapter_url.clone();
      return tokio::task::spawn_blocking(move ||
        parse_komik_chapter_document(&Html::parse_document(&entry_0_clone), &chapter_url_clone)
      ).await?;
    } else {
      HTML_CACHE.remove(&url);
    }
  }

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
    match fetch_with_proxy(&url, browser_client).await {
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

  match retry(backoff, fetch_operation).await {
    Ok(html) => {
      // Cache the result
      HTML_CACHE.insert(url.clone(), (html.clone(), Instant::now()));
      let html_clone = html.clone(); // Clone the html string
      let chapter_url_clone = chapter_url.clone();

      tokio::task::spawn_blocking(move ||
        parse_komik_chapter_document(&Html::parse_document(&html_clone), &chapter_url_clone)
      ).await?
    }
    Err(e) => {
      error!("Failed to fetch URL after retries: {}, error: {:?}", url, e);
      Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
  }
}

fn parse_komik_chapter_document(
  document: &Html,
  _chapter_url: &str
) -> Result<ChapterData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = Instant::now();
  info!("Starting to parse komik chapter document");

  let title = document
    .select(&TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let prev_chapter_id = document
    .select(&PREV_CHAPTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("href"))
    .and_then(|href| href.split('/').nth(3))
    .unwrap_or("")
    .to_string();

  let next_chapter_id = document
    .select(&NEXT_CHAPTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("href"))
    .and_then(|href| href.split('/').nth(3))
    .unwrap_or("")
    .to_string();

  let mut images = Vec::new();
  for el in document.select(&IMAGE_SELECTOR) {
    if let Some(src) = el.value().attr("src") {
      images.push(src.to_string());
    }
  }

  let duration = start_time.elapsed();
  info!("Parsed komik chapter document in {:?}", duration);

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