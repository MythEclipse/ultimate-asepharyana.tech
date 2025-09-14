//use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router}; Handler for the komik manga slug endpoint.

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use regex::Regex;
use rust_lib::config::CONFIG_MAP;
use tracing::{ info, error, warn };
use lazy_static::lazy_static;
use axum::extract::State;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use tokio::sync::Mutex as TokioMutex;
use tokio::task;
use backoff::{ future::retry, ExponentialBackoff };
use dashmap::DashMap;
use std::time::{ Duration, Instant };

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/komik/manga";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the komik/manga endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "komik";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "komik_manga_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<MangaResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaItem {
  pub title: String,
  pub poster: String,
  pub chapter: String,
  pub score: String,
  pub date: String,
  pub r#type: String,
  pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Pagination {
  pub current_page: u32,
  pub last_visible_page: u32,
  pub has_next_page: bool,
  pub next_page: Option<u32>,
  pub has_previous_page: bool,
  pub previous_page: Option<u32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaResponse {
  pub data: Vec<MangaItem>,
  pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct QueryParams {
  /// Page number for pagination (defaults to 1)
  pub page: Option<u32>,
}

lazy_static! {
  static ref BASE_URL: String = CONFIG_MAP.get("KOMIK_BASE_URL")
    .cloned()
    .unwrap_or_else(|| "https://komikindo.ch".to_string());
  pub static ref ANIMPOST_SELECTOR: Selector = Selector::parse(".animposx").unwrap();
  pub static ref TITLE_SELECTOR: Selector = Selector::parse(".tt h4").unwrap();
  pub static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  pub static ref CHAPTER_SELECTOR: Selector = Selector::parse(".lsch a").unwrap();
  pub static ref SCORE_SELECTOR: Selector = Selector::parse("i").unwrap();
  pub static ref DATE_SELECTOR: Selector = Selector::parse(".datech").unwrap();
  pub static ref TYPE_SELECTOR: Selector = Selector::parse(".typeflag").unwrap();
  pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  pub static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
  pub static ref CURRENT_SELECTOR: Selector = Selector::parse(".pagination .current").unwrap();
  pub static ref PAGE_SELECTORS: Selector = Selector::parse(".pagination a:not(.next)").unwrap();
  pub static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  pub static ref PREV_SELECTOR: Selector = Selector::parse(".pagination .prev").unwrap();
  pub static ref HTML_CACHE: DashMap<String, (String, Instant)> = DashMap::new();
}
const CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

#[utoipa::path(
    get,
    params(
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/manga",
    tag = "komik",
    operation_id = "komik_manga_slug",
    responses(
        (status = 200, description = "Handles GET requests for the komik/manga endpoint.", body = MangaResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn list(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<QueryParams>
) -> impl IntoResponse {
  let start_time = Instant::now();
  let page = params.page.unwrap_or(1);
  info!("Starting manga list request for page {}", page);

  let url = format!("{}/manga/page/{}/", *BASE_URL, page);

  match fetch_and_parse_manga_list(&Arc::new(TokioMutex::new(())), &url, page).await {
    Ok((data, pagination)) => {
      let total_duration = start_time.elapsed();
      info!("Successfully processed manga list for page: {} in {:?}", page, total_duration);
      Json(MangaResponse {
        data,
        pagination,
      })
    }
    Err(e) => {
      let total_duration = start_time.elapsed();
      error!(
        "Failed to process manga list for page: {} after {:?}, error: {:?}",
        page,
        total_duration,
        e
      );
      Json(MangaResponse {
        data: vec![],
        pagination: Pagination {
          current_page: page,
          last_visible_page: page,
          has_next_page: false,
          next_page: None,
          has_previous_page: false,
          previous_page: None,
        },
      })
    }
  }
}

async fn fetch_and_parse_manga_list(
  client: &Arc<TokioMutex<()>>,
  url: &str,
  page: u32
) -> Result<(Vec<MangaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
  // Check cache first
  if let Some(entry) = HTML_CACHE.get(url) {
    if entry.1.elapsed() < CACHE_TTL {
      info!("Cache hit for URL: {}", url);
      return task::spawn_blocking(move || {
        let document = Html::parse_document(&entry.0);
        parse_manga_list_document(&document, page)
      }).await?;
    } else {
      HTML_CACHE.remove(url);
    }
  }

  let backoff = ExponentialBackoff {
    initial_interval: Duration::from_millis(500),
    max_interval: Duration::from_secs(10),
    multiplier: 2.0,
    max_elapsed_time: Some(Duration::from_secs(30)),
    ..Default::default()
  };

  let fetch_operation = || async {
    info!("Fetching URL: {}", url);
    match fetch_with_proxy(url, client).await {
      Ok(response) => {
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
  HTML_CACHE.insert(url.to_string(), (html.clone(), Instant::now()));

  task::spawn_blocking(move || {
    let document = Html::parse_document(&html);
    parse_manga_list_document(&document, page)
  }).await?
}

fn parse_manga_list_document(
  document: &Html,
  current_page: u32
) -> Result<(Vec<MangaItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
  let mut data = Vec::new();

  for element in document.select(&ANIMPOST_SELECTOR) {
    let title = element
      .select(&TITLE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let mut poster = element
      .select(&IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();
    poster = poster.split('?').next().unwrap_or(&poster).to_string();

    let chapter = element
      .select(&CHAPTER_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .and_then(|text|
        CHAPTER_REGEX.captures(&text)
          .and_then(|cap| cap.get(0))
          .map(|m| m.as_str().to_string())
      )
      .unwrap_or_default();

    let score = element
      .select(&SCORE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let date = element
      .select(&DATE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let r#type = element
      .select(&TYPE_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("class"))
      .map(|class| class.split(' ').nth(1).unwrap_or("").to_string())
      .unwrap_or_default();

    let slug = element
      .select(&LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    data.push(MangaItem {
      title,
      poster,
      chapter,
      score,
      date,
      r#type,
      slug,
    });
  }

  let last_visible_page = document
    .select(&PAGE_SELECTORS)
    .last()
    .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
    .unwrap_or(current_page);

  let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
  let has_previous_page = document.select(&PREV_SELECTOR).next().is_some();

  let pagination = Pagination {
    current_page,
    last_visible_page,
    has_next_page,
    next_page: if has_next_page {
      Some(current_page + 1)
    } else {
      None
    },
    has_previous_page,
    previous_page: if has_previous_page {
      Some(current_page - 1)
    } else {
      None
    },
  };

  Ok((data, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(list))
}