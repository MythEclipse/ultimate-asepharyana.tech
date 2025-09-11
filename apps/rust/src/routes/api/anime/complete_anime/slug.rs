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
use rust_lib::chromiumoxide::BrowserPool;
use axum::extract::State;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/complete-anime/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str =
  "Handles GET requests for the anime/complete-anime/slug endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_complete_anime_slug";
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

lazy_static! {
  static ref ITEM_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref EPISODE_SELECTOR: Selector = Selector::parse(".epz").unwrap();
  static ref PAGINATION_SELECTOR: Selector = Selector::parse(".pagenavix .page-numbers:not(.next)").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".pagenavix .next.page-numbers").unwrap();
  static ref CACHE: DashMap<String, (Vec<CompleteAnimeItem>, Pagination)> = DashMap::new();
}

async fn fetch_html(browser_pool: &BrowserPool, url: &str) -> Result<String, Box<dyn std::error::Error>> {
  let operation = || async {
    let response = fetch_with_proxy(url, browser_pool).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  retry(backoff, operation).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn parse_anime_page(html: &str, slug: &str) -> (Vec<CompleteAnimeItem>, Pagination) {
  let document = Html::parse_document(html);

  let mut anime_list = Vec::new();

  for element in document.select(&*ITEM_SELECTOR) {
    let title = element
      .select(&*TITLE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let slug = element
      .select(&*LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&*IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();

    let episode_count = element
      .select(&*EPISODE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_else(|| "N/A".to_string());

    let anime_url = element
      .select(&*LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    if !title.is_empty() {
      anime_list.push(CompleteAnimeItem {
        title,
        slug,
        poster,
        episode_count,
        anime_url,
      });
    }
  }

  let current_page = slug.parse::<u32>().unwrap_or(1);
  let last_visible_page = document
    .select(&*PAGINATION_SELECTOR)
    .last()
    .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
    .unwrap_or(1);

  let has_next_page = document.select(&*NEXT_SELECTOR).next().is_some();
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

  (anime_list, pagination)
}

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime/complete-anime/{slug}",
    tag = "anime",
    operation_id = "anime_complete_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/complete-anime/slug endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
  State(app_state): State<Arc<AppState>>,
  Path(slug): Path<String>
) -> impl IntoResponse {
  let start = Instant::now();
  info!("Starting request for complete_anime slug: {}", slug);

  // Check cache first
  if let Some(cached) = CACHE.get(&slug) {
    let duration = start.elapsed();
    info!("Cache hit for slug: {}, duration: {:?}", slug, duration);
    return Json(ListResponse {
      message: "Success".to_string(),
      data: cached.0.clone(),
      total: Some(cached.0.len() as i64),
    });
  }

  match fetch_html(&app_state.browser_pool, &format!("https://otakudesu.cloud/complete-anime/page/{}/", slug)).await {
    Ok(html) => {
      let (anime_list, pagination) = parse_anime_page(&html, &slug);
      // Cache the result
      CACHE.insert(slug.clone(), (anime_list.clone(), pagination.clone()));
      let duration = start.elapsed();
      info!("Fetched and parsed for slug: {}, duration: {:?}", slug, duration);
      Json(ListResponse {
        message: "Success".to_string(),
        data: anime_list.clone(),
        total: Some(anime_list.len() as i64),
      })
    }
    Err(e) => {
      let duration = start.elapsed();
      error!("Error fetching for slug: {}, error: {:?}, duration: {:?}", slug, e, duration);
      Json(ListResponse {
        message: "Error".to_string(),
        data: vec![],
        total: Some(0),
      })
    }
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}