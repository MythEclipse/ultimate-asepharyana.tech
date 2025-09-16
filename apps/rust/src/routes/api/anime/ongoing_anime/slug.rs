use axum::{ extract::Path, response::IntoResponse, routing::get, Json, Router };
use axum::http::StatusCode;
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use lazy_static::lazy_static;
use backoff::{ future::retry, ExponentialBackoff };
use tracing::{ info, error, warn };
use tokio::sync::Mutex as TokioMutex;
use axum::extract::State;
use deadpool_redis::redis::AsyncCommands;
use regex::Regex;
use once_cell::sync::Lazy;
use std::time::Duration; // Add this import

// Pre-compiled regex for slug extraction
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/ongoing-anime/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str =
  "Handles GET requests for the anime/ongoing-anime/{slug} endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_ongoing_anime_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<OngoingAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub score: String,
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
pub struct OngoingAnimeResponse {
  pub status: String,
  pub data: Vec<OngoingAnimeItem>,
  pub pagination: Pagination,
}

lazy_static! {
  pub static ref VENZ_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
  pub static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
  pub static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  pub static ref EP_SELECTOR: Selector = Selector::parse(".epz").unwrap();
  pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  pub static ref PAGINATION_SELECTOR: Selector = Selector::parse(
    ".pagination .page-numbers:not(.next)"
  ).unwrap();
  pub static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  pub static ref EPISODE_REGEX: regex::Regex = regex::Regex::new(r"\(([^)]+)\)").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime/ongoing-anime/{slug}",
    tag = "anime",
    operation_id = "anime_ongoing_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/ongoing-anime/{slug} endpoint.", body = OngoingAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
  State(app_state): State<Arc<AppState>>,
  Path(slug): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let start = std::time::Instant::now();
  info!("Starting request for ongoing_anime slug: {}", slug);

  let cache_key = format!("anime:ongoing:{}", slug);
  let mut conn = app_state.redis_pool.get().await.map_err(|e| {
    error!("Failed to get Redis connection: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
  })?;

  // Check cache first
  let cached_response: Option<String> = conn.get(&cache_key).await.map_err(|e| {
    error!("Failed to get data from Redis: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
  })?;

  if let Some(json_data_string) = cached_response {
    info!("Cache hit for key: {}", cache_key);
    let ongoing_anime_response: OngoingAnimeResponse = serde_json
      ::from_str(&json_data_string)
      .map_err(|e| {
        error!("Failed to deserialize cached data: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
      })?;
    return Ok(Json(ongoing_anime_response).into_response());
  }

  let result = fetch_ongoing_anime_page(&Arc::new(TokioMutex::new(())), slug.clone()).await;

  match result {
    Ok((anime_list, pagination)) => {
      let response = OngoingAnimeResponse {
        status: "Ok".to_string(),
        data: anime_list,
        pagination,
      };
      let json_data = serde_json::to_string(&response).map_err(|e| {
        error!("Failed to serialize response for caching: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
      })?;

      // Cache the result
      conn.set_ex::<_, _, ()>(&cache_key, json_data, CACHE_TTL).await.map_err(|e| {
        error!("Failed to set data in Redis: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
      })?;
      info!("Cache set for key: {}", cache_key);

      let duration = start.elapsed();
      info!("Fetched and parsed ongoing_anime for slug: {}, duration: {:?}", slug, duration);
      Ok(Json(response).into_response())
    }
    Err(e) => {
      let duration = start.elapsed();
      error!(
        "Error fetching ongoing_anime for slug: {}, error: {:?}, duration: {:?}",
        slug,
        e,
        duration
      );
      Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)))
    }
  }
}

async fn fetch_ongoing_anime_page(
  client: &Arc<TokioMutex<()>>,
  slug: String
) -> Result<(Vec<OngoingAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
  let _start_time = std::time::Instant::now();
  let url = format!("https://otakudesu.cloud/ongoing-anime/page/{}/", slug);

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
        info!("Successfully fetched URL: {}", url);
        Ok(response.data)
      }
      Err(e) => {
        warn!("Failed to fetch URL: {}, error: {:?}", url, e);
        Err(backoff::Error::transient(e))
      }
    }
  };

  let html = retry(backoff, fetch_operation).await.map_err(|e|
    format!("Failed to fetch HTML with retry: {}", e)
  )?;
  let slug_clone = slug.clone();

  match
    tokio::task::spawn_blocking(move ||
      parse_ongoing_anime_document(&Html::parse_document(&html), &slug_clone)
    ).await
  {
    Ok(inner_result) => inner_result,
    Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
  }
}

fn parse_ongoing_anime_document(
  document: &Html,
  slug: &str
) -> Result<(Vec<OngoingAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  info!("Starting to parse ongoing anime document for slug: {}", slug);

  let mut anime_list = Vec::new();

  for element in document.select(&VENZ_SELECTOR) {
    let title = element
      .select(&TITLE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let poster = element
      .select(&IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();

    let score = element
      .select(&EP_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or("N/A".to_string());

    let anime_url = element
      .select(&LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let slug = SLUG_REGEX.captures(&anime_url)
      .and_then(|cap| cap.get(1))
      .map(|m| m.as_str())
      .unwrap_or("")
      .to_string();

    if !title.is_empty() {
      anime_list.push(OngoingAnimeItem {
        title,
        slug,
        poster,
        score,
        anime_url,
      });
    }
  }

  let current_page = slug.parse::<u32>().unwrap_or(1);

  let last_visible_page = document
    .select(&PAGINATION_SELECTOR)
    .next_back()
    .map(|e| e.text().collect::<String>().trim().parse::<u32>().unwrap_or(1))
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
  info!("Parsed {} ongoing anime items in {:?}", anime_list.len(), duration);

  Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}