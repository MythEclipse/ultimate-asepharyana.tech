use axum::{ extract::Path, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use rust_lib::fetch_with_proxy::fetch_with_proxy;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/ongoing-anime/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str =
  "Handles GET requests for the anime/ongoing-anime/{slug} endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_ongoing_anime_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<OngoingAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub episode: String,
  pub anime_url: String,
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
pub struct OngoingAnimeResponse {
  pub status: String,
  pub data: Vec<OngoingAnimeItem>,
  pub pagination: Pagination,
}

#[utoipa::path(
  get,
  params(("slug" = String, Path, description = "The slug identifier")),
  path = "/api/anime/ongoing-anime/{slug}",
  tag = "anime",
  operation_id = "anime_ongoing_anime_slug",
  responses(
    (
      status = 200,
      description = "Handles GET requests for the anime/ongoing-anime/{slug} endpoint.",
      body = OngoingAnimeResponse,
    ),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
  match fetch_ongoing_anime_page(&slug).await {
    Ok((anime_list, pagination)) =>
      Json(OngoingAnimeResponse {
        status: "Ok".to_string(),
        data: anime_list,
        pagination,
      }),
    Err(_) =>
      Json(OngoingAnimeResponse {
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
      }),
  }
}

async fn fetch_ongoing_anime_page(
  slug: &str
) -> Result<(Vec<OngoingAnimeItem>, Pagination), Box<dyn std::error::Error>> {
  let url = format!("https://otakudesu.cloud/ongoing-anime/page/{}/", slug);
  let response = fetch_with_proxy(&url).await?;
  let html = response.data;
  let document = Html::parse_document(&html);

  let venz_selector = Selector::parse(".venz ul li").unwrap();
  let title_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let ep_selector = Selector::parse(".epz").unwrap();
  let link_selector = Selector::parse("a").unwrap();

  let mut anime_list = Vec::new();

  for element in document.select(&venz_selector) {
    let title = element
      .select(&title_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();

    let episode = element
      .select(&ep_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or("Ongoing".to_string());

    let anime_url = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let slug = anime_url.split('/').nth(4).unwrap_or("").to_string();

    if !title.is_empty() {
      anime_list.push(OngoingAnimeItem {
        title,
        slug,
        poster,
        episode,
        anime_url,
      });
    }
  }

  let current_page = slug.parse::<u32>().unwrap_or(1);

  let last_visible_page = document
    .select(&Selector::parse(".pagination .page-numbers:not(.next)").unwrap())
    .last()
    .map(|e| e.text().collect::<String>().trim().parse::<u32>().unwrap_or(1))
    .unwrap_or(1);

  let has_next_page = document
    .select(&Selector::parse(".pagination .next").unwrap())
    .next()
    .is_some();

  let next_page = if has_next_page { Some(current_page + 1) } else { None };

  let has_previous_page = current_page > 1;
  let previous_page = if has_previous_page { Some(current_page - 1) } else { None };

  let pagination = Pagination {
    current_page,
    last_visible_page,
    has_next_page,
    next_page,
    has_previous_page,
    previous_page,
  };

  Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}