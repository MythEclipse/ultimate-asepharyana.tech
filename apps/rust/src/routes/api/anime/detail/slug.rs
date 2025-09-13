use axum::{ extract::Path, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::Selector;
use lazy_static::lazy_static;
use dashmap::DashMap;
use tracing::{ info, error };
use std::time::Instant;
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
  State(_app_state): State<Arc<AppState>>,
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

  match fetch_anime_detail(&slug).await {
    Ok(data) => {
      let detail_response = DetailResponse {
        status: "Ok".to_string(),
        data: data.clone(),
      };
      // Cache the result
      CACHE.insert(slug.clone(), data);
      let duration = start.elapsed();
      info!("Fetched and parsed detail for slug: {}, duration: {:?}", slug, duration);
      Json(detail_response)
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
  _slug: &str
) -> Result<AnimeDetailData, Box<dyn std::error::Error>> {
  Err("Browser functionality has been removed".into())
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}