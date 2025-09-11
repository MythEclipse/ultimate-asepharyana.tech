use axum::{ response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::chromiumoxide::BrowserPool;
use axum::extract::State;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime2";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime2 endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime2";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime2_index";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<Anime2Response>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub current_episode: String,
  pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompleteAnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub episode_count: String,
  pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Anime2Data {
  pub ongoing_anime: Vec<OngoingAnimeItem>,
  pub complete_anime: Vec<CompleteAnimeItem>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Anime2Response {
  pub status: String,
  pub data: Anime2Data,
}

#[utoipa::path(
    get,
    path = "/api/anime2",
    tag = "anime2",
    operation_id = "anime2_index",
    responses(
        (status = 200, description = "Handles GET requests for the anime2 endpoint.", body = Anime2Response),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn anime2(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
  match fetch_anime_data(&app_state.browser_pool).await {
    Ok(data) =>
      Json(Anime2Response {
        status: "Ok".to_string(),
        data,
      }),
    Err(_) =>
      Json(Anime2Response {
        status: "Error".to_string(),
        data: Anime2Data {
          ongoing_anime: vec![],
          complete_anime: vec![],
        },
      }),
  }
}

async fn fetch_anime_data(browser_pool: &BrowserPool) -> Result<Anime2Data, Box<dyn std::error::Error>> {
  let ongoing_url = "https://alqanime.net/advanced-search/?status=ongoing&order=update";
  let complete_url = "https://alqanime.net/advanced-search/?status=completed&order=update";

  let ongoing_html = fetch_html(browser_pool, ongoing_url).await?;
  let complete_html = fetch_html(browser_pool, complete_url).await?;

  let ongoing_anime = parse_ongoing_anime(&ongoing_html);
  let complete_anime = parse_complete_anime(&complete_html);

  Ok(Anime2Data {
    ongoing_anime,
    complete_anime,
  })
}

async fn fetch_html(browser_pool: &BrowserPool, url: &str) -> Result<String, Box<dyn std::error::Error>> {
  let response = fetch_with_proxy(url, browser_pool).await?;
  Ok(response.data)
}

fn parse_ongoing_anime(html: &str) -> Vec<OngoingAnimeItem> {
  let document = Html::parse_document(html);
  let item_selector = Selector::parse(".listupd .bs").unwrap();
  let title_selector = Selector::parse(".ntitle").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let episode_selector = Selector::parse(".epx").unwrap();

  let mut ongoing_anime = Vec::new();

  for element in document.select(&item_selector) {
    let title = element
      .select(&title_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let slug = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(3))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("data-src"))
      .unwrap_or("")
      .to_string();

    let current_episode = element
      .select(&episode_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_else(|| "N/A".to_string());

    let anime_url = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    if !title.is_empty() {
      ongoing_anime.push(OngoingAnimeItem {
        title,
        slug,
        poster,
        current_episode,
        anime_url,
      });
    }
  }

  ongoing_anime
}

fn parse_complete_anime(html: &str) -> Vec<CompleteAnimeItem> {
  let document = Html::parse_document(html);
  let item_selector = Selector::parse(".listupd .bs").unwrap();
  let title_selector = Selector::parse(".ntitle").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let episode_selector = Selector::parse(".epx").unwrap();

  let mut complete_anime = Vec::new();

  for element in document.select(&item_selector) {
    let title = element
      .select(&title_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let slug = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(3))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("data-src"))
      .unwrap_or("")
      .to_string();

    let episode_count = element
      .select(&episode_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_else(|| "N/A".to_string());

    let anime_url = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    if !title.is_empty() {
      complete_anime.push(CompleteAnimeItem {
        title,
        slug,
        poster,
        episode_count,
        anime_url,
      });
    }
  }

  complete_anime
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(anime2))
}