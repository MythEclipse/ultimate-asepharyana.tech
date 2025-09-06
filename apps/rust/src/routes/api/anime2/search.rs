use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use reqwest;
use scraper::{ Html, Selector };

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

#[derive(Deserialize)]
pub struct SearchQuery {
  pub q: Option<String>,
}

#[utoipa::path(
  get,
  path = "/api/api/anime2/search",
  tag = "anime2",
  operation_id = "anime2_search",
  responses(
    (
      status = 200,
      description = "Searches for anime2 based on query parameters.",
      body = SearchResponse,
    ),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
  let query = params.q.unwrap_or_else(|| "log".to_string());
  let url = format!("https://alqanime.net/?s={}", urlencoding::encode(&query));

  match fetch_and_parse_search(&url).await {
    Ok(response) => Json(response),
    Err(_) =>
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
      }),
  }
}

async fn fetch_and_parse_search(url: &str) -> Result<SearchResponse, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let response = client.get(url).send().await?;
  let html = response.text().await?;
  let document = Html::parse_document(&html);

  let item_selector = Selector::parse(".listupd article.bs").unwrap();
  let title_selector = Selector::parse(".ntitle").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let desc_selector = Selector::parse("h2").unwrap();
  let rating_selector = Selector::parse(".numscore").unwrap();
  let type_selector = Selector::parse(".typez").unwrap();

  let mut data = Vec::new();

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

    let description = element
      .select(&desc_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let anime_url = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let genres: Vec<String> = vec![]; // Genres not available

    let rating = element
      .select(&rating_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let r#type = element
      .select(&type_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let season = "".to_string(); // Season not available

    if !title.is_empty() {
      data.push(AnimeItem {
        title,
        slug,
        poster,
        description,
        anime_url,
        genres,
        rating,
        r#type,
        season,
      });
    }
  }

  let pagination = parse_pagination(&document);

  Ok(SearchResponse {
    status: "Ok".to_string(),
    data,
    pagination,
  })
}

fn parse_pagination(document: &Html) -> Pagination {
  let current_page_selector = Selector::parse(".pagination .current").unwrap();
  let page_numbers_selector = Selector::parse(".pagination .page-numbers").unwrap();
  let next_selector = Selector::parse(".pagination .next").unwrap();
  let prev_selector = Selector::parse(".pagination .prev").unwrap();

  let current_page = document
    .select(&current_page_selector)
    .next()
    .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
    .unwrap_or(1);

  let last_visible_page = document
    .select(&page_numbers_selector)
    .last()
    .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
    .unwrap_or(1);

  let has_next_page = document.select(&next_selector).next().is_some();
  let next_page = if has_next_page {
    document
      .select(&next_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .map(|s| s.to_string())
  } else {
    None
  };

  let has_previous_page = current_page > 1;
  let previous_page = if has_previous_page {
    document
      .select(&prev_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .map(|s| s.to_string())
  } else {
    None
  };

  Pagination {
    current_page,
    last_visible_page,
    has_next_page,
    next_page,
    has_previous_page,
    previous_page,
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}