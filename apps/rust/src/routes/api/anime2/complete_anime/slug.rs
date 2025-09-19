use axum::{ extract::Path, response::IntoResponse, routing::get, Json, Router };
use axum::http::StatusCode;
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use crate::fetch_with_proxy::fetch_with_proxy;
use lazy_static::lazy_static;
use backoff::{ future::retry, ExponentialBackoff };
use std::time::Duration;
use tracing::{ info, warn, error };
use regex::Regex;
use once_cell::sync::Lazy;
use axum::extract::State;
use deadpool_redis::redis::AsyncCommands;

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
pub struct CompleteAnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub episode_count: String,
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
  pub message: String,
  pub data: Vec<CompleteAnimeItem>,
  pub total: Option<i64>,
}

// Pre-compiled CSS selectors for performance
lazy_static! {
  pub static ref ITEM_SELECTOR: Selector = Selector::parse(".listupd article.bs").unwrap();
  pub static ref TITLE_SELECTOR: Selector = Selector::parse(".ntitle").unwrap();
  pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  pub static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  pub static ref EPISODE_SELECTOR: Selector = Selector::parse(".epx").unwrap();
  pub static ref PAGINATION_SELECTOR: Selector = Selector::parse(
    ".pagination .page-numbers:not(.next)"
  ).unwrap();
  pub static ref NEXT_SELECTOR: Selector = Selector::parse(
    ".pagination .next.page-numbers"
  ).unwrap();
}

// Pre-compiled regex for slug extraction
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime2/complete-anime/{slug}",
    tag = "anime2",
    operation_id = "anime2_complete_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/complete-anime/slug endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
  State(app_state): State<Arc<AppState>>,
  Path(slug): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let start_time = std::time::Instant::now();
  info!("Handling request for complete_anime slug: {}", slug);

  let cache_key = format!("anime2:complete:{}", slug);
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
    let list_response: ListResponse = serde_json::from_str(&json_data_string).map_err(|e| {
      error!("Failed to deserialize cached data: {:?}", e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
    })?;
    return Ok(Json(list_response).into_response());
  }

  let url =
    format!("https://alqanime.net/advanced-search/page/{}/?status=completed&order=update", slug);

  let html = fetch_html_with_retry(&url).await.map_err(|e| {
    error!("Failed to fetch HTML for slug: {}, error: {:?}", slug, e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching HTML: {}", e))
  })?;

  let html_clone = html.clone();
  let slug_clone = slug.clone();
  let (anime_list, _pagination) = tokio::task
    ::spawn_blocking(move || parse_anime_page(&html_clone, &slug_clone)).await
    .map_err(|e| {
      error!("Failed to spawn blocking task for slug: {}, error: {:?}", slug, e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Error spawning blocking task: {}", e))
    })
    ? // Handle JoinError
    .map_err(|e| {
      error!("Failed to parse anime page for slug: {}, error: {:?}", slug, e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Error parsing anime page: {}", e))
    })?; // Handle parsing error

  let list_response = ListResponse {
    message: "Success".to_string(),
    data: anime_list.clone(),
    total: Some(anime_list.len() as i64),
  };
  let json_data = serde_json::to_string(&list_response).map_err(|e| {
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
  info!("Successfully processed request for slug: {} in {:?}", slug, total_duration);
  Ok(Json(list_response).into_response())
}

async fn fetch_html_with_retry(
  url: &str
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
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
  Ok(html)
}

fn parse_anime_page(
  html: &str,
  slug: &str
) -> Result<(Vec<CompleteAnimeItem>, Pagination), String> {
  let start_time = std::time::Instant::now();
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
      .and_then(|href|
        SLUG_REGEX.captures(href)
          .and_then(|cap| cap.get(1))
          .map(|m| m.as_str())
      )
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("data-src"))
      .unwrap_or("")
      .to_string();

    let episode_count = element
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

    anime_list.push(CompleteAnimeItem {
      title,
      slug,
      poster,
      episode_count,
      anime_url,
    });
  }

  let current_page = slug.parse::<u32>().unwrap_or(1);
  let last_visible_page = document
    .select(&PAGINATION_SELECTOR)
    .next_back()
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

  Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}