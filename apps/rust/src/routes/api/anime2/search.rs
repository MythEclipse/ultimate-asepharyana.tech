use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use axum::http::StatusCode;
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
use std::time::{ Duration, Instant };
use tokio::sync::Mutex as TokioMutex;
use axum::extract::State;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime2/search";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Searches for anime2 based on query parameters.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime2";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime2_search";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub description: String,
  pub anime_url: String,
  pub genres: Vec<String>,
  pub rating: String,
  pub r#type: String,
  pub season: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Pagination {
  pub current_page: u32,
  pub last_visible_page: u32,
  pub has_next_page: bool,
  pub next_page: Option<String>,
  pub has_previous_page: bool,
  pub previous_page: Option<String>,
}

lazy_static! {
  pub static ref ITEM_SELECTOR: Selector = Selector::parse(".listupd .bs").unwrap();
  pub static ref TITLE_SELECTOR: Selector = Selector::parse(".ntitle").unwrap();
  pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  pub static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  pub static ref DESC_SELECTOR: Selector = Selector::parse(".data .typez").unwrap();
  pub static ref GENRE_SELECTOR: Selector = Selector::parse(".genres a").unwrap();
  pub static ref RATING_SELECTOR: Selector = Selector::parse(".score").unwrap();
  pub static ref TYPE_SELECTOR: Selector = Selector::parse(".typez").unwrap();
  pub static ref SEASON_SELECTOR: Selector = Selector::parse(".season").unwrap();
  pub static ref PAGINATION_SELECTOR: Selector = Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
  pub static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  static ref CACHE: DashMap<String, SearchResponse> = DashMap::new();
  static ref HTML_CACHE: DashMap<String, (String, Instant)> = DashMap::new();
}
const CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

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

#[utoipa::path(
    get,
    params(
        ("q" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value")
    ),
    path = "/api/anime2/search",
    tag = "anime2",
    operation_id = "anime2_search",
    responses(
        (status = 200, description = "Searches for anime2 based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<SearchQuery>
) -> impl IntoResponse {
  let start = Instant::now();
  info!("Starting search for query: {}", params.q.as_deref().unwrap_or(""));

  // Check cache first
  if let Some(cached) = self::CACHE.get(&params.q.clone().unwrap_or_default()) {
    let duration = start.elapsed();
    info!(
      "Cache hit for search query: {}, duration: {:?}",
      params.q.as_deref().unwrap_or(""),
      duration
    );
    return Json(cached.clone()).into_response();
  }

  let query = params.q.unwrap_or_else(|| "one".to_string());
  let url = format!("https://alqanime.net/?s={}", urlencoding::encode(&query));

  let result = fetch_and_parse_search(&Arc::new(TokioMutex::new(())), &url, query.clone()).await;

  match result {
    Ok((data, pagination)) => {
      let response = SearchResponse {
        status: "Ok".to_string(),
        data,
        pagination,
      };
      // Cache the result
      self::CACHE.insert(query.clone(), response.clone());
      let duration = start.elapsed();
      info!("Fetched and parsed search for query: {}, duration: {:?}", query, duration);
      Json(response).into_response()
    }
    Err(e) => {
      let duration = start.elapsed();
      error!("Error searching for query: {}, error: {:?}, duration: {:?}", query, e, duration);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
    }
  }
}

async fn fetch_and_parse_search(
  browser: &Arc<TokioMutex<()>>,
  url: &str,
  query: String
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
  // Check cache first
  if let Some(entry) = HTML_CACHE.get(url) {
    if entry.1.elapsed() < CACHE_TTL {
      info!("Cache hit for URL: {}", url);
      let html_string = entry.0.clone(); // Clone the html string for the blocking task
      let query_clone = query.clone();
      return match tokio::task::spawn_blocking(move || {
        let document = Html::parse_document(&html_string);
        parse_search_document(&document, &query_clone)
      }).await {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
      };
    } else {
      HTML_CACHE.remove(url);
    }
  }

  let operation = || async {
    let response = fetch_with_proxy(url, browser).await?;
    Ok(response.data)
  };

  let backoff = ExponentialBackoff::default();
  let html = retry(backoff, operation).await?;
  HTML_CACHE.insert(url.to_string(), (html.clone(), Instant::now()));
  let query_clone = query.clone();

  match tokio::task::spawn_blocking(move || {
    let document = Html::parse_document(&html);
    parse_search_document(&document, &query_clone)
  }).await {
    Ok(inner_result) => inner_result,
    Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
  }
}

fn parse_search_document(
  document: &Html,
  _query: &str
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
  let mut data = Vec::new();

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
      .and_then(|href| href.split('/').nth(3))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&IMG_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("data-src"))
      .unwrap_or("")
      .to_string();

    let description = element
      .select(&DESC_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let anime_url = element
      .select(&LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let genres: Vec<String> = element
      .select(&GENRE_SELECTOR)
      .map(|e| e.text().collect::<String>().trim().to_string())
      .collect();

    let rating = element
      .select(&RATING_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let r#type = element
      .select(&TYPE_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let season = element
      .select(&SEASON_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    if !title.is_empty() {
      data.push(AnimeItem {
        title,
        slug,
        poster,
        description,
        anime_url,
        genres,
        rating,
        r#type,
        season,
      });
    }
  }

  let pagination = parse_pagination(&document, _query);

  Ok((data, pagination))
}

fn parse_pagination(document: &Html, _query: &str) -> Pagination {
  let page_num = 1; // Simplified, as Next.js uses parseInt(slug, 10) || 1
  let last_visible_page = document
    .select(&PAGINATION_SELECTOR)
    .next_back()
    .map(|e| e.text().collect::<String>().trim().parse::<u32>().unwrap_or(1))
    .unwrap_or(1);

  let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
  let has_previous_page = page_num > 1;

  Pagination {
    current_page: page_num,
    last_visible_page,
    has_next_page,
    next_page: if has_next_page {
      Some((page_num + 1).to_string())
    } else {
      None
    },
    has_previous_page,
    previous_page: if has_previous_page {
      Some((page_num - 1).to_string())
    } else {
      None
    },
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}