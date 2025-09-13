//! Handler for the detail endpoint.
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
pub const ENDPOINT_PATH: &str = "/api/komik/detail";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves details for a specific komik by ID.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_detail";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailData>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EpisodeList {
  pub quality: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Recommendation {
  pub title: String,
  pub poster: String,
  pub komik_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Chapter {
  pub chapter: String,
  pub date: String,
  pub chapter_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailData {
  pub title: String,
  pub alternative_title: String,
  pub score: String,
  pub poster: String,
  pub description: String,
  pub status: String,
  pub r#type: String,
  pub release_date: String,
  pub author: String,
  pub total_chapter: String,
  pub updated_on: String,
  pub genres: Vec<String>,
  pub chapters: Vec<Chapter>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
  pub status: bool,
  pub data: DetailData,
}

#[derive(Deserialize, ToSchema)]
pub struct DetailQuery {
  /// The unique identifier for the komik (typically the slug or URL path)
  pub komik_id: Option<String>,
}

lazy_static! {
  static ref TITLE_SELECTOR: Selector = Selector::parse("h1.entry-title").unwrap();
  static ref SPE_SELECTOR: Selector = Selector::parse(".spe span").unwrap();
  static ref SCORE_SELECTOR: Selector = Selector::parse(".rtg > div > i").unwrap();
  static ref POSTER_SELECTOR: Selector = Selector::parse(".thumb img").unwrap();
  static ref DESC_SELECTOR: Selector = Selector::parse(
    "#sinopsis > section > div > div.entry-content.entry-content-single > p"
  ).unwrap();
  static ref A_SELECTOR: Selector = Selector::parse("a").unwrap();
  static ref RELEASE_DATE_SELECTOR: Selector = Selector::parse(
    "#chapter_list > ul > li > span.dt"
  ).unwrap();
  static ref TOTAL_CHAPTER_SELECTOR: Selector = Selector::parse(
    "#chapter_list > ul > li > span.lchx"
  ).unwrap();
  static ref UPDATED_ON_SELECTOR: Selector = Selector::parse(
    "#chapter_list > ul > li > span.dt"
  ).unwrap();
  static ref GENRE_SELECTOR: Selector = Selector::parse(".genre-info a").unwrap();
  static ref CHAPTER_LIST_SELECTOR: Selector = Selector::parse("#chapter_list ul li").unwrap();
  static ref CHAPTER_LINK_SELECTOR: Selector = Selector::parse(".lchx a").unwrap();
  static ref DATE_LINK_SELECTOR: Selector = Selector::parse(".dt a").unwrap();
}


#[utoipa::path(
    get,
    params(
        ("komik_id" = Option<String>, Query, description = "Comic/manga identifier", example = "sample_value")
    ),
    path = "/api/komik/detail",
    tag = "komik",
    operation_id = "komik_detail",
    responses(
        (status = 200, description = "Retrieves details for a specific komik by ID.", body = DetailData),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn detail(
  State(_app_state): State<Arc<AppState>>,
  Query(params): Query<DetailQuery>
) -> impl IntoResponse {
  let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
  info!("Detail functionality disabled - browser not available for komik_id {}", komik_id);

  Json(DetailData {
    title: "".to_string(),
    alternative_title: "".to_string(),
    score: "".to_string(),
    poster: "".to_string(),
    description: "".to_string(),
    status: "".to_string(),
    r#type: "".to_string(),
    release_date: "".to_string(),
    author: "".to_string(),
    total_chapter: "".to_string(),
    updated_on: "".to_string(),
    genres: vec![],
    chapters: vec![],
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(detail))
}