use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use regex::Regex;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::komik_base_url::get_cached_komik_base_url;
use tracing::{ error, info, warn };
use urlencoding;
use lazy_static::lazy_static;
use std::time::Instant;
use tokio::time::{ sleep, Duration };

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

lazy_static! {
  static ref ANIMPOST_SELECTOR: Selector = Selector::parse(".animposx").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse(".tt h4").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref CHAPTER_SELECTOR: Selector = Selector::parse(".lsch a").unwrap();
  static ref SCORE_SELECTOR: Selector = Selector::parse("i").unwrap();
  static ref DATE_SELECTOR: Selector = Selector::parse(".datech").unwrap();
  static ref TYPE_SELECTOR: Selector = Selector::parse(".typeflag").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
  static ref PAGE_SELECTORS: Selector = Selector::parse(".pagination a:not(.next)").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  static ref PREV_SELECTOR: Selector = Selector::parse(".pagination .prev").unwrap();
}

use rust_lib::chromiumoxide::BrowserPool;
use axum::extract::State;

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
pub async fn search(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<SearchQuery>
) -> impl IntoResponse {
  let query = params.query.unwrap_or_default();
  let page = params.page.unwrap_or(1);
  let start = Instant::now();
  info!("Starting search request for query '{}' page {}", query, page);

  let base_url = match get_cached_komik_base_url(&app_state.browser_pool, false).await {
    Ok(url) => url,
    Err(_) => {
      warn!("[search] Failed to get cached base URL, trying refresh");
      match get_cached_komik_base_url(&app_state.browser_pool, true).await {
        Ok(url) => url,
        Err(e) => {
          error!("[search] Failed to get base URL: {:?}", e);
          return Json(SearchResponse {
            data: vec![],
            pagination: Pagination {
              current_page: page,
              last_visible_page: page,
              has_next_page: false,
              next_page: None,
              has_previous_page: false,
              previous_page: None,
            },
          });
        }
      }
    }
  };

  let url = if query.is_empty() {
    format!("{}/page/{}/", base_url, page)
  } else {
    format!("{}/page/{}/?s={}", base_url, page, urlencoding::encode(&query))
  };

  match fetch_and_parse_search(&app_state.browser_pool, &url, &query, page).await {
    Ok(response) => {
      info!("Search request completed in {:?}", start.elapsed());
      Json(response)
    }
    Err(e) => {
      error!("[search] Failed to fetch and parse search: {:?}", e);
      info!("Search request completed in {:?}", start.elapsed());
      Json(SearchResponse {
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

async fn fetch_and_parse_search(
  browser_pool: &BrowserPool,
  url: &str,
  _query: &str,
  page: u32
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
  let start = Instant::now();
  info!("[fetch_and_parse_search] Starting fetch for URL: {}", url);

  let html = match fetch_with_retry(browser_pool, url, 3).await {
    Ok(h) => h,
    Err(e) => {
      error!("[fetch_and_parse_search] Fetch failed in {:?}: {:?}", start.elapsed(), e);
      return Err(e);
    }
  };

  let document = Html::parse_document(&html);

  let animposx_selector = &*ANIMPOST_SELECTOR;
  let title_selector = &*TITLE_SELECTOR;
  let img_selector = &*IMG_SELECTOR;
  let chapter_selector = &*CHAPTER_SELECTOR;
  let score_selector = &*SCORE_SELECTOR;
  let date_selector = &*DATE_SELECTOR;
  let type_selector = &*TYPE_SELECTOR;
  let link_selector = &*LINK_SELECTOR;

  info!("[fetch_and_parse_search] Fetched and parsed search in {:?}", start.elapsed());

  let mut data = Vec::new();

  for element in document.select(&animposx_selector) {
    let title = element
      .select(&title_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let mut poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();
    if let Some(pos) = poster.find('?') {
      poster = poster[..pos].to_string();
    }

    let chapter_text = element
      .select(&chapter_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();
    let chapter = CHAPTER_REGEX.find(&chapter_text)
      .map(|m| m.as_str().to_string())
      .unwrap_or_default();

    let score = element
      .select(&score_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let date = element
      .select(&date_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let r#type = element
      .select(&type_selector)
      .next()
      .and_then(|e| e.value().attr("class"))
      .and_then(|class| class.split_whitespace().nth(1))
      .unwrap_or("")
      .to_string();

    let slug = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    if !title.is_empty() {
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
  }

  let pagination = parse_pagination(&document, page);

  Ok(SearchResponse {
    data,
    pagination,
  })
}

fn parse_pagination(document: &Html, current_page: u32) -> Pagination {
  let start = Instant::now();
  info!("[parse_pagination] Starting pagination parsing");

  let page_selectors = &*PAGE_SELECTORS;
  let next_selector = &*NEXT_SELECTOR;
  let prev_selector = &*PREV_SELECTOR;

  let mut last_visible_page = current_page;
  for element in document.select(&page_selectors) {
    if let Ok(page) = element.text().collect::<String>().trim().parse::<u32>() {
      if page > last_visible_page {
        last_visible_page = page;
      }
    }
  }

  let has_next_page = document.select(&next_selector).next().is_some();
  let next_page = if has_next_page && current_page < last_visible_page {
    Some(current_page + 1)
  } else {
    None
  };

  let has_previous_page = document.select(&prev_selector).next().is_some();
  let previous_page = if has_previous_page && current_page > 1 {
    Some(current_page - 1)
  } else {
    None
  };

  info!("[parse_pagination] Parsed pagination in {:?}", start.elapsed());

  Pagination {
    current_page,
    last_visible_page,
    has_next_page,
    next_page,
    has_previous_page,
    previous_page,
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}