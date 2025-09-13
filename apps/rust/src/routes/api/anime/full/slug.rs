use axum::{ extract::{ Path, State }, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use std::time::Instant;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use lazy_static::lazy_static;
use backoff::{ future::retry, ExponentialBackoff };
use dashmap::DashMap;
use tracing::{ info, error };
use headless_chrome::browser::Browser;
use tokio::sync::Mutex as TokioMutex;

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
  static ref EPISODE_TITLE_SELECTOR: Selector = Selector::parse("h1.posttl").unwrap();
  static ref IMAGE_SELECTOR: Selector = Selector::parse(".cukder img").unwrap();
  static ref STREAM_SELECTOR: Selector = Selector::parse("#embed_holder iframe").unwrap();
  static ref DOWNLOAD_ITEM_SELECTOR: Selector = Selector::parse(".download ul li").unwrap();
  static ref RESOLUTION_SELECTOR: Selector = Selector::parse("strong").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref NEXT_EPISODE_SELECTOR: Selector = Selector::parse(".flir a[title*='Episode Selanjutnya']").unwrap();
  static ref PREVIOUS_EPISODE_SELECTOR: Selector = Selector::parse(".flir a[title*='Episode Sebelumnya']").unwrap();
  static ref CACHE: DashMap<String, AnimeFullData> = DashMap::new();
}

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
) -> impl IntoResponse {
  let start = Instant::now();
  info!("Starting request for full slug: {}", slug);

  // Check cache first
  if let Some(cached) = CACHE.get(&slug) {
    let duration = start.elapsed();
    info!("Cache hit for full slug: {}, duration: {:?}", slug, duration);
    return Json(FullResponse {
      status: "Ok".to_string(),
      data: cached.clone(),
    });
  }

  match fetch_anime_full(&app_state.browser, &slug).await {
    Ok(data) => {
      let full_response = FullResponse {
        status: "Ok".to_string(),
        data: data.clone(),
      };
      // Cache the result
      CACHE.insert(slug.clone(), data);
      let duration = start.elapsed();
      info!("Fetched and parsed full for slug: {}, duration: {:?}", slug, duration);
      Json(full_response)
    }
    Err(e) => {
      let duration = start.elapsed();
      error!("Error fetching full for slug: {}, error: {:?}, duration: {:?}", slug, e, duration);
      Json(FullResponse {
        status: "Error".to_string(),
        data: AnimeFullData {
          episode: "".to_string(),
          episode_number: "".to_string(),
          anime: AnimeInfo { slug: "".to_string() },
          has_next_episode: false,
          next_episode: None,
          has_previous_episode: false,
          previous_episode: None,
          stream_url: "".to_string(),
          download_urls: std::collections::HashMap::new(),
          image_url: "".to_string(),
        },
      })
    }
  }
}

async fn fetch_anime_full(
  client: &Arc<TokioMutex<Browser>>,
  slug: &str
) -> Result<AnimeFullData, Box<dyn std::error::Error>> {
  let url = format!("https://otakudesu.cloud/episode/{}", slug);

  let operation = || async {
    let response = fetch_with_proxy(&url, client).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  let html = retry(backoff, operation).await?;
  let document = Html::parse_document(&html);

  let episode = document
    .select(&*EPISODE_TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let episode_number = episode
    .split("Episode")
    .nth(1)
    .map(|s| s.trim().to_string())
    .unwrap_or_default();

  let image_url = document
    .select(&*IMAGE_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();

  let stream_url = document
    .select(&*STREAM_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();

  let mut download_urls = std::collections::HashMap::new();

  for element in document.select(&*DOWNLOAD_ITEM_SELECTOR) {
    let resolution = element
      .select(&*RESOLUTION_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let mut links = Vec::new();
    for link_element in element.select(&*LINK_SELECTOR) {
      let server = link_element.text().collect::<String>().trim().to_string();
      let url = link_element.value().attr("href").unwrap_or("").to_string();
      links.push(DownloadLink { server, url });
    }

    if !resolution.is_empty() && !links.is_empty() {
      download_urls.insert(resolution, links);
    }
  }

  let next_episode_element = document.select(&*NEXT_EPISODE_SELECTOR).next();

  let previous_episode_element = document.select(&*PREVIOUS_EPISODE_SELECTOR).next();

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