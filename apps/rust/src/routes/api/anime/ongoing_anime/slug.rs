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
use tokio::sync::Mutex as TokioMutex;
use axum::extract::State;

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
  static ref VENZ_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref EP_SELECTOR: Selector = Selector::parse(".epz").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref PAGINATION_SELECTOR: Selector = Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  static ref CACHE: DashMap<String, (Vec<OngoingAnimeItem>, Pagination)> = DashMap::new();
}

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
  State(_app_state): State<Arc<AppState>>,
  Path(slug): Path<String>
) -> impl IntoResponse {
  let start = Instant::now();
  info!("Starting request for ongoing_anime slug: {}", slug);

  // Check cache first
  if let Some(cached) = CACHE.get(&slug) {
    let duration = start.elapsed();
    info!("Cache hit for ongoing_anime slug: {}, duration: {:?}", slug, duration);
    return Json(OngoingAnimeResponse {
      status: "Ok".to_string(),
      data: cached.0.clone(),
      pagination: cached.1.clone(),
    });
  }

  match fetch_ongoing_anime_page(&Arc::new(TokioMutex::new(())), &slug).await {
    Ok((anime_list, pagination)) => {
      // Cache the result
      CACHE.insert(slug.clone(), (anime_list.clone(), pagination.clone()));
      let duration = start.elapsed();
      info!("Fetched and parsed ongoing_anime for slug: {}, duration: {:?}", slug, duration);
      Json(OngoingAnimeResponse {
        status: "Ok".to_string(),
        data: anime_list,
        pagination,
      })
    }
    Err(e) => {
      let duration = start.elapsed();
      error!(
        "Error fetching ongoing_anime for slug: {}, error: {:?}, duration: {:?}",
        slug,
        e,
        duration
      );
      Json(OngoingAnimeResponse {
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

async fn fetch_ongoing_anime_page(
  client: &Arc<TokioMutex<()>>,
  slug: &str
) -> Result<(Vec<OngoingAnimeItem>, Pagination), Box<dyn std::error::Error>> {
  let url = format!("https://otakudesu.cloud/ongoing-anime/page/{}/", slug);

  let operation = || async {
    let response = fetch_with_proxy(&url, client).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  let html = retry(backoff, operation).await?;
  let document = Html::parse_document(&html);

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

    let slug = anime_url.split('/').nth(4).unwrap_or("").to_string();

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

  Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}