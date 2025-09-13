//use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router}; Handler for the komik manhua endpoint.

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::Selector;
use regex::Regex;
use rust_lib::config::CONFIG_MAP;
use lazy_static::lazy_static;
use tracing::info;
use std::time::Instant;
use axum::extract::State;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/komik/manhua";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the komik/manhua endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "komik";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "komik_manhua_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ManhuaResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ManhuaItem {
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
pub struct ManhuaResponse {
  pub data: Vec<ManhuaItem>,
  pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct QueryParams {
  /// Page number for pagination (required, defaults to 1 if not provided)
  pub page: u32,
}

lazy_static! {
  static ref BASE_URL: String = CONFIG_MAP.get("KOMIK_BASE_URL")
    .cloned()
    .unwrap_or_else(|| "https://komikindo.id".to_string());
  static ref ANIMPOST_SELECTOR: Selector = Selector::parse(".animposx").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse(".tt h4").unwrap();
  static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
  static ref CHAPTER_SELECTOR: Selector = Selector::parse(".lsch a").unwrap();
  static ref SCORE_SELECTOR: Selector = Selector::parse("i").unwrap();
  static ref DATE_SELECTOR: Selector = Selector::parse(".datech").unwrap();
  static ref TYPE_SELECTOR: Selector = Selector::parse(".typeflag").unwrap();
  static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref CHAPTER_REGEX: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
  static ref CURRENT_SELECTOR: Selector = Selector::parse(".pagination .current").unwrap();
  static ref PAGE_SELECTORS: Selector = Selector::parse(".pagination a:not(.next)").unwrap();
  static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
  static ref PREV_SELECTOR: Selector = Selector::parse(".pagination .prev").unwrap();
}

#[utoipa::path(
    get,
    params(
        ("page" = u32, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/komik/manhua",
    tag = "komik",
    operation_id = "komik_manhua_slug",
    responses(
        (status = 200, description = "Handles GET requests for the komik/manhua endpoint.", body = ManhuaResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn list(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<QueryParams>
) -> impl IntoResponse {
  let page = params.page;

  let base_url = &*BASE_URL;

  let _url = format!("{}/manhua/page/{}/", base_url, page);

  let start = Instant::now();
  info!("Starting manhua list request for page {}", page);

  info!("Manhua list functionality disabled - browser not available for page {}", page);
  info!("Manhua list request completed in {:?}", start.elapsed());

  Json(ManhuaResponse {
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

/// Handles GET requests for the komik/manhua endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(list))
}