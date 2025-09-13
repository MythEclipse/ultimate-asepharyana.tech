use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use tracing::info;
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
  let query = params.q.unwrap_or_else(|| "log".to_string());
  info!("Search functionality disabled - browser not available for query '{}'", query);

  Json(SearchResponse {
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

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}