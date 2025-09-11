//! Handler for the chapter endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::komik_base_url::get_cached_komik_base_url;
use tracing::{ info, error };
use lazy_static::lazy_static;
use std::time::Instant;
use tokio::time::{ sleep, Duration };
use rust_lib::chromiumoxide::BrowserPool;
use axum::extract::State;

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
  static ref PREV_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".nextprev a[rel=\"prev\"]"
  ).unwrap();
  static ref NEXT_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".nextprev a[rel=\"next\"]"
  ).unwrap();
  static ref IMAGE_SELECTOR: Selector = Selector::parse("#chimg-auh img").unwrap();
}

async fn fetch_with_retry(
  browser_pool: &BrowserPool,
  url: &str,
  max_retries: u32
) -> Result<String, Box<dyn std::error::Error>> {
  let mut attempt = 0;
  loop {
    match fetch_with_proxy(url, browser_pool).await {
      Ok(response) => {
        return Ok(response.data);
      }
      Err(e) => {
        attempt += 1;
        if attempt > max_retries {
          error!("Failed to fetch {} after {} attempts: {:?}", url, max_retries, e);
          return Err(Box::new(e));
        }
        let delay = Duration::from_millis((2u64).pow(attempt) * 100);
        info!("Retrying fetch for {} in {:?}", url, delay);
        sleep(delay).await;
      }
    }
  }
}

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
pub async fn chapter(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<ChapterQuery>
) -> impl IntoResponse {
  let chapter_url = params.chapter_url.unwrap_or_default();
  let start = Instant::now();
  info!("Starting chapter request for chapter_url {}", chapter_url);

  match get_cached_komik_base_url(&app_state.browser_pool, false).await {
    Ok(base_url) => {
      match fetch_and_parse_chapter(&app_state.browser_pool, &chapter_url, &base_url).await {
        Ok(data) => {
          info!("[komik][chapter] Success for chapter_url: {}", chapter_url);
          info!("Chapter request completed in {:?}", start.elapsed());
          Json(ChapterResponse {
            message: "Success".to_string(),
            data,
          })
        }
        Err(e) => {
          error!("[komik][chapter] Error parsing chapter for {}: {:?}", chapter_url, e);
          info!("Chapter request completed in {:?}", start.elapsed());
          Json(ChapterResponse {
            message: "Error parsing chapter".to_string(),
            data: ChapterData {
              title: "".to_string(),
              next_chapter_id: "".to_string(),
              prev_chapter_id: "".to_string(),
              images: vec![],
            },
          })
        }
      }
    }
    Err(e) => {
      error!("[komik][chapter] Error getting base URL: {:?}", e);
      info!("Chapter request completed in {:?}", start.elapsed());
      Json(ChapterResponse {
        message: "Error parsing chapter".to_string(),
        data: ChapterData {
          title: "".to_string(),
          next_chapter_id: "".to_string(),
          prev_chapter_id: "".to_string(),
          images: vec![],
        },
      })
    }
  }
}

async fn fetch_and_parse_chapter(
  browser_pool: &BrowserPool,
  chapter_url: &str,
  base_url: &str
) -> Result<ChapterData, Box<dyn std::error::Error>> {
  let start = Instant::now();
  let url = format!("{}/chapter/{}", base_url, chapter_url);
  info!("[fetch_and_parse_chapter] Fetching URL: {}", url);

  let html = fetch_with_retry(browser_pool, &url, 3).await?;
  let document = Html::parse_document(&html);

  let title = document
    .select(&*TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let prev_chapter_element = document.select(&*PREV_CHAPTER_SELECTOR).next();
  let prev_chapter_id = if let Some(element) = prev_chapter_element {
    element
      .value()
      .attr("href")
      .and_then(|href| href.split('/').nth(3))
      .unwrap_or("")
      .to_string()
  } else {
    "".to_string()
  };

  let next_chapter_element = document.select(&*NEXT_CHAPTER_SELECTOR).next();
  let next_chapter_id = if let Some(element) = next_chapter_element {
    element
      .value()
      .attr("href")
      .and_then(|href| href.split('/').nth(3))
      .unwrap_or("")
      .to_string()
  } else {
    "".to_string()
  };

  let mut images = Vec::new();
  for element in document.select(&*IMAGE_SELECTOR) {
    if let Some(src) = element.value().attr("src") {
      images.push(src.to_string());
    }
  }

  info!("[fetch_and_parse_chapter] Successfully parsed chapter for {}", chapter_url);
  info!("Fetched and parsed chapter in {:?}", start.elapsed());
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