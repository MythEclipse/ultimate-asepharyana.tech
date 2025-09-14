use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use axum::http::StatusCode;
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use regex::Regex;
use tracing::{ info, error, warn };
use lazy_static::lazy_static;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use tokio::sync::Mutex as TokioMutex;
use tokio::task;
use backoff::{ future::retry, ExponentialBackoff };
use std::time::{ Duration, Instant };
use dashmap::DashMap;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/komik/search";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Searches for komik based on query parameters.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "komik";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "komik_search";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

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
pub struct SearchResponse {
  pub data: Vec<MangaItem>,
  pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct SearchQuery {
  /// Search query string to filter komik results
  pub query: Option<String>,
  /// Page number for pagination (defaults to 1)
  pub page: Option<u32>,
}

use axum::extract::State;
use rust_lib::config::CONFIG_MAP;

lazy_static! {
  static ref KOMIK_BASE_URL: String = CONFIG_MAP.get("KOMIK_BASE_URL")
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
        ("query" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/search",
    tag = "komik",
    operation_id = "komik_search",
    responses(
        (status = 200, description = "Searches for komik based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
#[axum::debug_handler]
pub async fn search(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<SearchQuery>
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let start_time = Instant::now();
  let query = params.query.unwrap_or_default();
  let page = params.page.unwrap_or(1);
  info!("Starting komik search for query: '{}', page: {}", query, page);

  let url = format!("{}/page/{}/?s={}", *KOMIK_BASE_URL, page, urlencoding::encode(&query));

  let (data, pagination) = fetch_and_parse_search(&Arc::new(TokioMutex::new(())), &url, page)
    .await
    .map_err(|e| {
      error!(
        "Failed to process komik search for query: '{}', page: {} after {:?}, error: {:?}",
        query,
        page,
        start_time.elapsed(),
        e
      );
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
    })?;

  let total_duration = start_time.elapsed();
  info!(
    "Successfully processed komik search for query: '{}', page: {} in {:?}",
    query,
    page,
    total_duration
  );
  Ok(
    Json(SearchResponse {
      data,
      pagination,
    })
  )
}

async fn fetch_and_parse_search(
  client: &Arc<TokioMutex<()>>,
  url: &str,
  page: u32
) -> Result<(Vec<MangaItem>, Pagination), String> {
  // Check cache first
  if let Some(entry) = HTML_CACHE.get(url) {
    if entry.1.elapsed() < CACHE_TTL {
      info!("Cache hit for URL: {}", url);
      let html_string = entry.0.clone(); // Clone the html string for the blocking task
      let parse_result = task::spawn_blocking(move ||
        parse_search_document(html_string, page)
      ).await;
      return match parse_result {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(format!("Blocking task failed: {}", join_err)),
      };
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

  let html = retry(backoff, fetch_operation).await
    .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;
  HTML_CACHE.insert(url.to_string(), (html.clone(), Instant::now()));

  let parse_result = task::spawn_blocking(move || parse_search_document(html, page)).await;
  match parse_result {
    Ok(inner_result) => inner_result,
    Err(join_err) => Err(format!("Blocking task failed: {}", join_err)),
  }
}

fn parse_search_document(
  html_string: String,
  current_page: u32
) -> Result<(Vec<MangaItem>, Pagination), String> {
  let document = Html::parse_document(&html_string);
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
      .and_then(|text| {
        CHAPTER_REGEX
          .captures(&text)
          .and_then(|cap: regex::Captures| cap.get(0))
          .map(|m| m.as_str().to_string())
      })
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
    router.route(ENDPOINT_PATH, get(search))
}