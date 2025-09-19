use axum::{ extract::{ Path, State }, response::IntoResponse, routing::get, Json, Router };
use axum::http::StatusCode;
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use crate::fetch_with_proxy::fetch_with_proxy;
use lazy_static::lazy_static;
use backoff::{ future::retry, ExponentialBackoff };
use tracing::{ info, error };
use deadpool_redis::redis::AsyncCommands;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/full/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/full/{slug} endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_full_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<FullResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeInfo {
  pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EpisodeInfo {
  pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DownloadLink {
  pub server: String,
  pub url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeFullData {
  pub episode: String,
  pub episode_number: String,
  pub anime: AnimeInfo,
  pub has_next_episode: bool,
  pub next_episode: Option<EpisodeInfo>,
  pub has_previous_episode: bool,
  pub previous_episode: Option<EpisodeInfo>,
  pub stream_url: String,
  pub download_urls: std::collections::HashMap<String, Vec<DownloadLink>>,
  pub image_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct FullResponse {
  pub status: String,
  pub data: AnimeFullData,
}

lazy_static! {
  pub static ref EPISODE_TITLE_SELECTOR: Selector = Selector::parse("h1.posttl").unwrap();
  pub static ref IMAGE_SELECTOR: Selector = Selector::parse(".cukder img").unwrap();
  pub static ref STREAM_SELECTOR: Selector = Selector::parse("#embed_holder iframe").unwrap();
  pub static ref DOWNLOAD_ITEM_SELECTOR: Selector = Selector::parse(".download ul li").unwrap();
  pub static ref RESOLUTION_SELECTOR: Selector = Selector::parse("strong").unwrap();
  pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  pub static ref NEXT_EPISODE_SELECTOR: Selector = Selector::parse(
    ".flir a[title*='Episode Selanjutnya']"
  ).unwrap();
  pub static ref PREVIOUS_EPISODE_SELECTOR: Selector = Selector::parse(
    ".flir a[title*='Episode Sebelumnya']"
  ).unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "naruto-shippuden-episode-1")
    ),
    path = "/api/anime/full/{slug}",
    tag = "anime",
    operation_id = "anime_full_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/full/{slug} endpoint.", body = FullResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
  State(app_state): State<Arc<AppState>>,
  Path(slug): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let start = std::time::Instant::now();
  info!("Starting request for full slug: {}", slug);

  let cache_key = format!("anime:full:{}", slug);
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
    let full_response: FullResponse = serde_json::from_str(&json_data_string).map_err(|e| {
      error!("Failed to deserialize cached data: {:?}", e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
    })?;
    return Ok(Json(full_response).into_response());
  }

  let data = fetch_anime_full(slug.clone()).await.map_err(|e| {
    error!("Error fetching full for slug: {}, error: {:?}", slug, e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
  })?;

  let full_response = FullResponse {
    status: "Ok".to_string(),
    data: data.clone(),
  };
  let json_data = serde_json::to_string(&full_response).map_err(|e| {
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
  info!("Fetched and parsed full for slug: {}, duration: {:?}", slug, duration);
  Ok(Json(full_response).into_response())
}

async fn fetch_anime_full(
   slug: String
 ) -> Result<AnimeFullData, String> {
  let url = format!("https://otakudesu.cloud/episode/{}", slug);

  let operation = || async {
    let response = fetch_with_proxy(&url).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  let html = retry(backoff, operation).await.map_err(|e|
    format!("Failed to fetch HTML with retry: {}", e)
  )?;

  match
    tokio::task::spawn_blocking(move || {
      let document = Html::parse_document(&html);
      parse_anime_full_document(&document, &slug)
    }).await
  {
    Ok(inner_result) => inner_result.map_err(|e| e.to_string()),
    Err(join_err) => Err(format!("Failed to spawn blocking task: {}", join_err)),
  }
}

fn parse_anime_full_document(
  document: &Html,
  slug: &str
) -> Result<AnimeFullData, Box<dyn std::error::Error + Send + Sync>> {
  let episode = document
    .select(&EPISODE_TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let episode_number = episode
    .split("Episode")
    .nth(1)
    .map(|s| s.trim().to_string())
    .unwrap_or_default();

  let image_url = document
    .select(&IMAGE_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();

  let stream_url = document
    .select(&STREAM_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();

  let mut download_urls = std::collections::HashMap::new();

  for element in document.select(&DOWNLOAD_ITEM_SELECTOR) {
    let resolution = element
      .select(&RESOLUTION_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let mut links = Vec::new();
    for link_element in element.select(&LINK_SELECTOR) {
      let server = link_element.text().collect::<String>().trim().to_string();
      let url = link_element.value().attr("href").unwrap_or("").to_string();
      links.push(DownloadLink { server, url });
    }

    if !resolution.is_empty() && !links.is_empty() {
      download_urls.insert(resolution, links);
    }
  }

  let next_episode_element = document.select(&NEXT_EPISODE_SELECTOR).next();

  let previous_episode_element = document.select(&PREVIOUS_EPISODE_SELECTOR).next();

  let next_episode_slug = next_episode_element
    .and_then(|e| e.value().attr("href"))
    .and_then(|href| href.split('/').nth(href.split('/').count().saturating_sub(2)))
    .map(|s| s.to_string() + "/");

  let previous_episode_slug = previous_episode_element
    .and_then(|e| e.value().attr("href"))
    .and_then(|href| href.split('/').nth(href.split('/').count().saturating_sub(2)))
    .map(|s| s.to_string() + "/");

  Ok(AnimeFullData {
    episode,
    episode_number,
    anime: AnimeInfo { slug: slug.to_string() },
    has_next_episode: next_episode_slug.is_some(),
    next_episode: next_episode_slug.map(|s| EpisodeInfo { slug: s }),
    has_previous_episode: previous_episode_slug.is_some(),
    previous_episode: previous_episode_slug.map(|s| EpisodeInfo { slug: s }),
    stream_url,
    download_urls,
    image_url,
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}