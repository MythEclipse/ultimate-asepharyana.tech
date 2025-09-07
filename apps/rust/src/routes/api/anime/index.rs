use axum::{ response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_index";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<AnimeResponse>";

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
pub struct AnimeData {
  pub ongoing_anime: Vec<OngoingAnimeItem>,
  pub complete_anime: Vec<CompleteAnimeItem>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeResponse {
  pub status: String,
  pub data: AnimeData,
}

#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "anime",
    operation_id = "anime_index",
    responses(
        (status = 200, description = "Handles GET requests for the anime endpoint.", body = AnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn anime() -> impl IntoResponse {
  match fetch_anime_data().await {
    Ok(data) =>
      Json(AnimeResponse {
        status: "Ok".to_string(),
        data,
      }),
    Err(_) =>
      Json(AnimeResponse {
        status: "Error".to_string(),
        data: AnimeData {
          ongoing_anime: vec![],
          complete_anime: vec![],
        },
      }),
  }
}

async fn fetch_anime_data() -> Result<AnimeData, Box<dyn std::error::Error>> {
  let ongoing_url = "https://otakudesu.cloud/ongoing-anime/";
  let complete_url = "https://otakudesu.cloud/complete-anime/";

  let ongoing_html = fetch_html(ongoing_url).await?;
  let complete_html = fetch_html(complete_url).await?;

  let ongoing_anime = parse_ongoing_anime(&ongoing_html);
  let complete_anime = parse_complete_anime(&complete_html);

  Ok(AnimeData {
    ongoing_anime,
    complete_anime,
  })
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
  let response = fetch_with_proxy(url).await?;
  Ok(response.data)
}

fn parse_ongoing_anime(html: &str) -> Vec<OngoingAnimeItem> {
  let document = Html::parse_document(html);
  let item_selector = Selector::parse(".venz ul li").unwrap();
  let title_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let episode_selector = Selector::parse(".epz").unwrap();

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
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("src"))
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
  let item_selector = Selector::parse(".venz ul li").unwrap();
  let title_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let episode_selector = Selector::parse(".epz").unwrap();

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
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("src"))
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
    router.route(ENDPOINT_PATH, get(anime))
}