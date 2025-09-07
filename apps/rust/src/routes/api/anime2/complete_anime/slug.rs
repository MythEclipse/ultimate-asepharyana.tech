use axum::{ extract::Path, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use lazy_static::lazy_static;
use backoff::{ future::retry, ExponentialBackoff };
use dashmap::DashMap;
use std::time::{ Duration, Instant };
use tracing::{ info, warn, error };
use regex::Regex;
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime2/complete-anime/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str =
  "Handles GET requests for the anime2/complete-anime/slug endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime2";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime2_complete_anime_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ListResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub episode: String,
  pub anime_url: String,
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
pub struct ListResponse {
  pub status: String,
  pub data: Vec<AnimeItem>,
  pub pagination: Pagination,
}

// Pre-compiled CSS selectors for performance
lazy_static! {
  static ref ITEM_SELECTOR: Selector = Selector::parse(".listupd article.bs").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse(".ntitle").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref EPISODE_SELECTOR: Selector = Selector::parse(".epx").unwrap();
  static ref PAGINATION_SELECTOR: Selector = Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next.page-numbers").unwrap();
}

// Pre-compiled regex for slug extraction
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

// Cache for HTML responses with TTL (5 minutes)
lazy_static! {
  static ref HTML_CACHE: DashMap<String, (String, Instant)> = DashMap::new();
}
const CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = Instant::now();

  // Check cache first
  if let Some(entry) = HTML_CACHE.get(url) {
    if entry.1.elapsed() < CACHE_TTL {
      info!("Cache hit for URL: {}", url);
      return Ok(entry.0.clone());
    } else {
      HTML_CACHE.remove(url);
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
    match fetch_with_proxy(url).await {
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
      HTML_CACHE.insert(url.to_string(), (html.clone(), Instant::now()));
      Ok(html)
    }
    Err(e) => {
      error!("Failed to fetch URL after retries: {}, error: {:?}", url, e);
      Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
  }
}

fn parse_anime_page(html: &str, slug: &str) -> (Vec<AnimeItem>, Pagination) {
  let start_time = Instant::now();
  info!("Starting to parse anime page for slug: {}", slug);

  let document = Html::parse_document(html);
  let mut anime_list = Vec::new();

  for element in document.select(&ITEM_SELECTOR) {
    let title = element
      .select(&TITLE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let slug = element
      .select(&LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| SLUG_REGEX.captures(href).and_then(|cap| cap.get(1)).map(|m| m.as_str()))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("data-src"))
      .unwrap_or("")
      .to_string();

    let episode = element
      .select(&EPISODE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_else(|| "N/A".to_string());

    let anime_url = element
      .select(&LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    anime_list.push(AnimeItem {
      title,
      slug,
      poster,
      episode,
      anime_url,
    });
  }

  let current_page = slug.parse::<u32>().unwrap_or(1);
  let last_visible_page = document
    .select(&PAGINATION_SELECTOR)
    .last()
    .and_then(|e| e.text().collect::<String>().parse::<u32>().ok())
    .unwrap_or(1);

  let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
  let next_page = if has_next_page { Some(current_page + 1) } else { None };
  let has_previous_page = current_page > 1;
  let previous_page = if has_previous_page { Some(current_page - 1) } else { None };

  let pagination = Pagination {
    current_page,
    last_visible_page,
    has_next_page,
    next_page,
    has_previous_page,
    previous_page,
  };

  let duration = start_time.elapsed();
  info!("Parsed {} anime items in {:?}", anime_list.len(), duration);

  (anime_list, pagination)
}

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "naruto-shippuden-episode-1")
    ),
    path = "/api/anime2/complete-anime/{slug}",
    tag = "anime2",
    operation_id = "anime2_complete_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/complete-anime/slug endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
  let start_time = Instant::now();
  info!("Handling request for complete_anime slug: {}", slug);

  let url = format!("https://alqanime.net/advanced-search/page/{}/?status=completed&order=update", slug);

  match fetch_html(&url).await {
    Ok(html) => {
      let (anime_list, pagination) = parse_anime_page(&html, &slug);
      let total_duration = start_time.elapsed();
      info!("Successfully processed request for slug: {} in {:?}", slug, total_duration);
      Json(ListResponse {
        status: "Ok".to_string(),
        data: anime_list,
        pagination,
      })
    }
    Err(e) => {
      let total_duration = start_time.elapsed();
      error!("Failed to process request for slug: {} after {:?}, error: {:?}", slug, total_duration, e);
      Json(ListResponse {
        status: "Error".to_string(),
        data: vec![],
        pagination: Pagination {
          current_page: 1,
          last_visible_page: 1,
          has_next_page: false,
          next_page: None,
          has_previous_page: false,
          previous_page: None,
        },
      })
    }
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}