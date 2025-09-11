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
use tracing::{ info, error };
use std::time::Instant;
use rust_lib::headless_chrome::BrowserPool;
use axum::extract::State;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/detail/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/detail/{slug} endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_detail_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
  pub name: String,
  pub slug: String,
  pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EpisodeList {
  pub episode: String,
  pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Recommendation {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub status: String,
  pub r#type: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeDetailData {
  pub title: String,
  pub alternative_title: String,
  pub poster: String,
  pub r#type: String,
  pub status: String,
  pub release_date: String,
  pub studio: String,
  pub genres: Vec<Genre>,
  pub synopsis: String,
  pub episode_lists: Vec<EpisodeList>,
  pub batch: Vec<EpisodeList>,
  pub producers: Vec<String>,
  pub recommendations: Vec<Recommendation>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
  pub status: String,
  pub data: AnimeDetailData,
}

lazy_static! {
  static ref INFO_SELECTOR: Selector = Selector::parse(".infozingle p").unwrap();
  static ref POSTER_SELECTOR: Selector = Selector::parse(".fotoanime img").unwrap();
  static ref SYNOPSIS_SELECTOR: Selector = Selector::parse(".sinopc").unwrap();
  static ref GENRE_LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref EPISODE_LIST_SELECTOR: Selector = Selector::parse(".episodelist ul li a").unwrap();
  static ref RECOMMENDATION_SELECTOR: Selector = Selector::parse("#recommend-anime-series .isi-anime").unwrap();
  static ref RECOMMENDATION_TITLE_SELECTOR: Selector = Selector::parse(".judul-anime a").unwrap();
  static ref RECOMMENDATION_IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref CACHE: DashMap<String, AnimeDetailData> = DashMap::new();
}

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "naruto-shippuden-episode-1")
    ),
    path = "/api/anime/detail/{slug}",
    tag = "anime",
    operation_id = "anime_detail_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/detail/{slug} endpoint.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
  State(app_state): State<Arc<AppState>>,
  Path(slug): Path<String>
) -> impl IntoResponse {
  let start = Instant::now();
  info!("Starting request for detail slug: {}", slug);

  // Check cache first
  if let Some(cached) = CACHE.get(&slug) {
    let duration = start.elapsed();
    info!("Cache hit for detail slug: {}, duration: {:?}", slug, duration);
    return Json(DetailResponse {
      status: "Ok".to_string(),
      data: cached.clone(),
    });
  }

  match fetch_anime_detail(&app_state.browser_pool, &slug).await {
    Ok(data) => {
      // Cache the result
      CACHE.insert(slug.clone(), data.clone());
      let duration = start.elapsed();
      info!("Fetched and parsed detail for slug: {}, duration: {:?}", slug, duration);
      Json(DetailResponse {
        status: "Ok".to_string(),
        data,
      })
    }
    Err(e) => {
      let duration = start.elapsed();
      error!("Error fetching detail for slug: {}, error: {:?}, duration: {:?}", slug, e, duration);
      Json(DetailResponse {
        status: "Error".to_string(),
        data: AnimeDetailData {
          title: "".to_string(),
          alternative_title: "".to_string(),
          poster: "".to_string(),
          r#type: "".to_string(),
          status: "".to_string(),
          release_date: "".to_string(),
          studio: "".to_string(),
          genres: vec![],
          synopsis: "".to_string(),
          episode_lists: vec![],
          batch: vec![],
          producers: vec![],
          recommendations: vec![],
        },
      })
    }
  }
}

async fn fetch_anime_detail(
  browser_pool: &BrowserPool,
  slug: &str
) -> Result<AnimeDetailData, Box<dyn std::error::Error>> {
  let url = format!("https://otakudesu.cloud/anime/{}", slug);

  let operation = || async {
    let response = fetch_with_proxy(&url, browser_pool).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  let html = retry(backoff, operation).await?;
  let document = Html::parse_document(&html);

  let title = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Judul"))
    .map(|e| e.text().collect::<String>().replace("Judul: ", "").trim().to_string())
    .unwrap_or_default();

  let alternative_title = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Japanese"))
    .map(|e| e.text().collect::<String>().replace("Japanese: ", "").trim().to_string())
    .unwrap_or_default();

  let poster = document
    .select(&*POSTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();

  let r#type = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Tipe"))
    .map(|e| e.text().collect::<String>().replace("Tipe: ", "").trim().to_string())
    .unwrap_or_default();

  let release_date = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Tanggal Rilis"))
    .map(|e| e.text().collect::<String>().replace("Tanggal Rilis: ", "").trim().to_string())
    .unwrap_or_default();

  let status = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Status"))
    .map(|e| e.text().collect::<String>().replace("Status: ", "").trim().to_string())
    .unwrap_or_default();

  let synopsis = document
    .select(&*SYNOPSIS_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let studio = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Studio"))
    .map(|e| e.text().collect::<String>().replace("Studio: ", "").trim().to_string())
    .unwrap_or_default();

  let mut genres = Vec::new();
  if
    let Some(genre_paragraph) = document
      .select(&*INFO_SELECTOR)
      .find(|e| e.text().collect::<String>().contains("Genre"))
  {
    for element in genre_paragraph.select(&*GENRE_LINK_SELECTOR) {
      let name = element.text().collect::<String>().trim().to_string();
      let genre_slug = element
        .value()
        .attr("href")
        .and_then(|href| href.split('/').nth(4))
        .unwrap_or("")
        .to_string();
      let anime_url = element.value().attr("href").unwrap_or("").to_string();
      genres.push(Genre { name, slug: genre_slug, anime_url });
    }
  }

  let mut episode_lists = Vec::new();
  let mut batch = Vec::new();
  for element in document.select(&*EPISODE_LIST_SELECTOR) {
    let episode = element.text().collect::<String>().trim().to_string();
    let href = element.value().attr("href").unwrap_or("");

    // Generate slug from href or episode text if href is empty
    let episode_slug = if !href.is_empty() {
      // Try to extract slug from URL
      href
        .split('/')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap_or("")
        .to_string()
    } else {
      // Generate slug from episode text
      episode
        .to_lowercase()
        .replace("subtitle indonesia", "")
        .replace("episode", "episode-")
        .replace(" ", "-")
        .trim_matches('-')
        .to_string()
    };

    if episode.to_lowercase().contains("batch") {
      batch.push(EpisodeList { episode, slug: episode_slug });
    } else {
      episode_lists.push(EpisodeList { episode, slug: episode_slug });
    }
  }

  let producers_text = document
    .select(&*INFO_SELECTOR)
    .find(|e| e.text().collect::<String>().contains("Produser"))
    .map(|e| e.text().collect::<String>().replace("Produser: ", "").trim().to_string())
    .unwrap_or_default();
  let producers = producers_text
    .split(',')
    .map(|s| s.trim().to_string())
    .collect();

  let mut recommendations = Vec::new();
  for element in document.select(&*RECOMMENDATION_SELECTOR) {
    let title = element
      .select(&*RECOMMENDATION_TITLE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let url = element
      .select(&*GENRE_LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&*RECOMMENDATION_IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();

    let slug = url.split('/').nth(4).unwrap_or("").to_string();

    recommendations.push(Recommendation {
      title,
      slug,
      poster,
      status: "".to_string(),
      r#type: "".to_string(),
    });
  }

  Ok(AnimeDetailData {
    title,
    alternative_title,
    poster,
    r#type,
    status,
    release_date,
    studio,
    genres,
    synopsis,
    episode_lists,
    batch,
    producers,
    recommendations,
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}