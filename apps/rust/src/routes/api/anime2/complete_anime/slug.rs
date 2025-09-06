use axum::{ extract::Path, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use reqwest;
use scraper::{ Html, Selector };

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/anime2/complete-anime/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str =
  "Handles GET requests for the anime2/complete-anime/slug endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime2";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime2_complete_anime_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ListResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
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
pub struct ListResponse {
  pub status: String,
  pub data: Vec<AnimeItem>,
  pub pagination: Pagination,
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let response = client.get(url).send().await?;
  let html = response.text().await?;
  Ok(html)
}

fn parse_anime_page(html: &str, slug: &str) -> (Vec<AnimeItem>, Pagination) {
  let document = Html::parse_document(html);

  let item_selector = Selector::parse(".listupd article.bs").unwrap();
  let title_selector = Selector::parse(".ntitle").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let episode_selector = Selector::parse(".epx").unwrap();
  let pagination_selector = Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
  let next_selector = Selector::parse(".pagination .next.page-numbers").unwrap();

  let mut anime_list = Vec::new();

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

    let episode = element
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

    anime_list.push(AnimeItem {
      title,
      slug,
      poster,
      episode,
      anime_url,
    });
  }

  let current_page = slug.parse::<u32>().unwrap_or(1);
  let last_visible_page = document
    .select(&pagination_selector)
    .last()
    .and_then(|e| e.text().collect::<String>().parse::<u32>().ok())
    .unwrap_or(1);

  let has_next_page = document.select(&next_selector).next().is_some();
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

  (anime_list, pagination)
}

#[utoipa::path(
  get,
  params(("slug" = String, Path, description = "The slug identifier")),
  path = "/api/anime2/complete-anime/{slug}",
  tag = "anime2",
  operation_id = "anime2_complete_anime_slug",
  responses(
    (
      status = 200,
      description = "Handles GET requests for the anime2/complete-anime/slug endpoint.",
      body = ListResponse,
    ),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
  match
    fetch_html(
      &format!("https://alqanime.net/advanced-search/page/{}/?status=completed&order=update", slug)
    ).await
  {
    Ok(html) => {
      let (anime_list, pagination) = parse_anime_page(&html, &slug);
      Json(ListResponse {
        status: "Ok".to_string(),
        data: anime_list,
        pagination,
      })
    }
    Err(_) =>
      Json(ListResponse {
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

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}