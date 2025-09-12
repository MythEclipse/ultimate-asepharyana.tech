use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use regex::Regex;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use lazy_static::lazy_static;
use backoff::{ future::retry, ExponentialBackoff };
use dashmap::DashMap;
use tracing::{ info, error };
use std::time::Instant;
use fantoccini::Client as FantocciniClient;
use axum::extract::State;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/search";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Searches for anime based on query parameters.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_search";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub episode: String,
  pub anime_url: String,
  pub genres: Vec<String>,
  pub status: String,
  pub rating: String,
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
  pub status: String,
  pub data: Vec<AnimeItem>,
  pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct SearchQuery {
  pub q: Option<String>,
}

lazy_static! {
  static ref ITEM_SELECTOR: Selector = Selector::parse("#venkonten .chivsrc li").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse("h2 a").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref GENRE_SELECTOR: Selector = Selector::parse(".set a").unwrap();
  static ref STATUS_SELECTOR: Selector = Selector::parse(".set").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".hpage .r").unwrap();
  static ref EPISODE_REGEX: Regex = Regex::new(r"\(([^)]+)\)").unwrap();
  static ref CACHE: DashMap<String, SearchResponse> = DashMap::new();
}

#[utoipa::path(
    get,
    params(
        ("q" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value")
    ),
    path = "/api/anime/search",
    tag = "anime",
    operation_id = "anime_search",
    responses(
        (status = 200, description = "Searches for anime based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<SearchQuery>
) -> impl IntoResponse {
  let start = Instant::now();
  let query = params.q.unwrap_or_else(|| "one".to_string());
  info!("Starting search for query: {}", query);

  // Check cache first
  if let Some(cached) = CACHE.get(&query) {
    let duration = start.elapsed();
    info!("Cache hit for search query: {}, duration: {:?}", query, duration);
    return Json(cached.clone());
  }

  let url = format!("https://otakudesu.cloud/?s={}&post_type=anime", urlencoding::encode(&query));

  match fetch_and_parse_search(&app_state.browser_client, &url, &query).await {
    Ok(response) => {
      // Cache the result
      CACHE.insert(query.clone(), response.clone());
      let duration = start.elapsed();
      info!("Fetched and parsed search for query: {}, duration: {:?}", query, duration);
      Json(response)
    }
    Err(e) => {
      let duration = start.elapsed();
      error!("Error searching for query: {}, error: {:?}, duration: {:?}", query, e, duration);
      Json(SearchResponse {
        status: "Error".to_string(),
        data: vec![],
        pagination: Pagination {
          current_page: 1,
          last_visible_page: 57,
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
  client: &FantocciniClient,
  url: &str,
  query: &str
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
  let operation = || async {
    let response = fetch_with_proxy(url, client).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  let html = retry(backoff, operation).await?;
  let document = Html::parse_document(&html);

  let mut data = Vec::new();

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

    let episode_text = element
      .select(&*TITLE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>())
      .unwrap_or_default();

    let episode = EPISODE_REGEX.captures(&episode_text)
      .and_then(|cap| cap.get(1))
      .map(|m| m.as_str().to_string())
      .unwrap_or_else(|| "Ongoing".to_string());

    let anime_url = element
      .select(&*LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let genres: Vec<String> = element
      .select(&*STATUS_SELECTOR)
      .find(|e| e.text().collect::<String>().contains("Genres"))
      .map(|set|
        set
          .select(&*GENRE_SELECTOR)
          .map(|e| e.text().collect::<String>().trim().to_string())
          .collect()
      )
      .unwrap_or_default();

    let status = element
      .select(&*STATUS_SELECTOR)
      .find(|e| e.text().collect::<String>().contains("Status"))
      .map(|e| e.text().collect::<String>().replace("Status :", "").trim().to_string())
      .unwrap_or_default();

    let rating = element
      .select(&*STATUS_SELECTOR)
      .find(|e| e.text().collect::<String>().contains("Rating"))
      .map(|e| e.text().collect::<String>().replace("Rating :", "").trim().to_string())
      .unwrap_or_default();

    if !title.is_empty() {
      data.push(AnimeItem {
        title,
        slug,
        poster,
        episode,
        anime_url,
        genres,
        status,
        rating,
      });
    }
  }

  let pagination = parse_pagination(&document, query);

  Ok(SearchResponse {
    status: "Ok".to_string(),
    data,
    pagination,
  })
}

fn parse_pagination(document: &Html, _query: &str) -> Pagination {
  let page_num = 1; // Simplified, as Next.js uses parseInt(slug, 10) || 1
  let last_visible_page = 57;

  let has_next_page = document.select(&*NEXT_SELECTOR).next().is_some();
  let has_previous_page = page_num > 1;

  Pagination {
    current_page: page_num,
    last_visible_page,
    has_next_page,
    next_page: if has_next_page {
      Some(page_num + 1)
    } else {
      None
    },
    has_previous_page,
    previous_page: if has_previous_page {
      Some(page_num - 1)
    } else {
      None
    },
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}