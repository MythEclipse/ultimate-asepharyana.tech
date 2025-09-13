use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::Selector;
use regex::Regex;
use tracing::info;
use lazy_static::lazy_static;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/komik/search";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Searches for komik based on query parameters.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "komik";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "komik_search";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaItem {
  pub title: String,
  pub poster: String,
  pub chapter: String,
  pub score: String,
  pub date: String,
  pub r#type: String,
  pub slug: String,
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
  pub data: Vec<MangaItem>,
  pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct SearchQuery {
  /// Search query string to filter komik results
  pub query: Option<String>,
  /// Page number for pagination (defaults to 1)
  pub page: Option<u32>,
}

lazy_static! {
  static ref ANIMPOST_SELECTOR: Selector = Selector::parse(".animposx").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse(".tt h4").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref CHAPTER_SELECTOR: Selector = Selector::parse(".lsch a").unwrap();
  static ref SCORE_SELECTOR: Selector = Selector::parse("i").unwrap();
  static ref DATE_SELECTOR: Selector = Selector::parse(".datech").unwrap();
  static ref TYPE_SELECTOR: Selector = Selector::parse(".typeflag").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
  static ref PAGE_SELECTORS: Selector = Selector::parse(".pagination a:not(.next)").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  static ref PREV_SELECTOR: Selector = Selector::parse(".pagination .prev").unwrap();
}

use axum::extract::State;

#[utoipa::path(
    get,
    params(
        ("query" = Option<String>, Query, description = "Search parameter for filtering results", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/search",
    tag = "komik",
    operation_id = "komik_search",
    responses(
        (status = 200, description = "Searches for komik based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<SearchQuery>
) -> impl IntoResponse {
  let query = params.query.unwrap_or_default();
  let page = params.page.unwrap_or(1);
  info!("Search functionality disabled - browser not available for query '{}' page {}", query, page);

  Json(SearchResponse {
    data: vec![],
    pagination: Pagination {
      current_page: page,
      last_visible_page: page,
      has_next_page: false,
      next_page: None,
      has_previous_page: false,
      previous_page: None,
    },
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}