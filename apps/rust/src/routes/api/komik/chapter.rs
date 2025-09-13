//! Handler for the chapter endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::Selector;
use tracing::info;
use lazy_static::lazy_static;
use axum::extract::State;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik/chapter";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves chapter data for a specific komik chapter.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_chapter";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ChapterResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChapterData {
  pub title: String,
  pub next_chapter_id: String,
  pub prev_chapter_id: String,
  pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChapterResponse {
  pub message: String,
  pub data: ChapterData,
}

#[derive(Deserialize, ToSchema)]
pub struct ChapterQuery {
  /// URL-friendly identifier for the chapter (typically the chapter slug or URL path)
  pub chapter_url: Option<String>,
}

lazy_static! {
  static ref TITLE_SELECTOR: Selector = Selector::parse(".entry-title").unwrap();
  static ref PREV_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".nextprev a[rel=\"prev\"]"
  ).unwrap();
  static ref NEXT_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".nextprev a[rel=\"next\"]"
  ).unwrap();
  static ref IMAGE_SELECTOR: Selector = Selector::parse("#chimg-auh img").unwrap();
}


#[utoipa::path(
    get,
    params(
        ("chapter_url" = Option<String>, Query, description = "Chapter-specific identifier", example = "sample_value")
    ),
    path = "/api/komik/chapter",
    tag = "komik",
    operation_id = "komik_chapter",
    responses(
        (status = 200, description = "Retrieves chapter data for a specific komik chapter.", body = ChapterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn chapter(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<ChapterQuery>
) -> impl IntoResponse {
  let chapter_url = params.chapter_url.unwrap_or_default();
  info!("Chapter functionality disabled - browser not available for chapter_url {}", chapter_url);

  Json(ChapterResponse {
    message: "Error parsing chapter".to_string(),
    data: ChapterData {
      title: "".to_string(),
      next_chapter_id: "".to_string(),
      prev_chapter_id: "".to_string(),
      images: vec![],
    },
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(chapter))
}