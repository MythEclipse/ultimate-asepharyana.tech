use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use reqwest;
use scraper::{ Html, Selector };
use regex::Regex;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/anime/search";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Searches for anime based on query parameters.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime_search";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub episode: String,
  pub anime_url: String,
  pub genres: Vec<String>,
  pub status: String,
  pub rating: String,
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
  path = "/api/anime/search",
  tag = "anime",
  operation_id = "anime_search",
  responses(
    (
      status = 200,
      description = "Searches for anime based on query parameters.",
      body = SearchResponse,
    ),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
  let query = params.q.unwrap_or_else(|| "one".to_string());
  let url = format!("https://otakudesu.cloud/?s={}&post_type=anime", urlencoding::encode(&query));

  match fetch_and_parse_search(&url, &query).await {
    Ok(response) => Json(response),
    Err(_) =>
      Json(SearchResponse {
        status: "Error".to_string(),
        data: vec![],
        pagination: Pagination {
          current_page: 1,
          last_visible_page: 57,
          has_next_page: false,
          next_page: None,
          has_previous_page: false,
          previous_page: None,
        },
      }),
  }
}

async fn fetch_and_parse_search(
  url: &str,
  query: &str
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let response = client.get(url).send().await?;
  let html = response.text().await?;
  let document = Html::parse_document(&html);

  let item_selector = Selector::parse("#venkonten .chivsrc li").unwrap();
  let title_selector = Selector::parse("h2 a").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let link_selector = Selector::parse("a").unwrap();
  let genre_selector = Selector::parse(".set a").unwrap();
  let status_selector = Selector::parse(".set").unwrap();

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
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();

    let episode_text = element
      .select(&title_selector)
      .next()
      .map(|e| e.text().collect::<String>())
      .unwrap_or_default();

    let episode_regex = Regex::new(r"\(([^)]+)\)").unwrap();
    let episode = episode_regex
      .captures(&episode_text)
      .and_then(|cap| cap.get(1))
      .map(|m| m.as_str().to_string())
      .unwrap_or_else(|| "Ongoing".to_string());

    let anime_url = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let genres: Vec<String> = element
      .select(&Selector::parse(".set").unwrap())
      .find(|e| e.text().collect::<String>().contains("Genres"))
      .map(|set|
        set
          .select(&genre_selector)
          .map(|e| e.text().collect::<String>().trim().to_string())
          .collect()
      )
      .unwrap_or_default();

    let status = element
      .select(&status_selector)
      .find(|e| e.text().collect::<String>().contains("Status"))
      .map(|e| e.text().collect::<String>().replace("Status :", "").trim().to_string())
      .unwrap_or_default();

    let rating = element
      .select(&status_selector)
      .find(|e| e.text().collect::<String>().contains("Rating"))
      .map(|e| e.text().collect::<String>().replace("Rating :", "").trim().to_string())
      .unwrap_or_default();

    if !title.is_empty() {
      data.push(AnimeItem {
        title,
        slug,
        poster,
        episode,
        anime_url,
        genres,
        status,
        rating,
      });
    }
  }

  let pagination = parse_pagination(&document, query);

  Ok(SearchResponse {
    status: "Ok".to_string(),
    data,
    pagination,
  })
}

fn parse_pagination(document: &Html, _query: &str) -> Pagination {
  let page_num = 1; // Simplified, as Next.js uses parseInt(slug, 10) || 1
  let last_visible_page = 57;
  let next_selector = Selector::parse(".hpage .r").unwrap();

  let has_next_page = document.select(&next_selector).next().is_some();
  let has_previous_page = page_num > 1;

  Pagination {
    current_page: page_num,
    last_visible_page,
    has_next_page,
    next_page: if has_next_page {
      Some(page_num + 1)
    } else {
      None
    },
    has_previous_page,
    previous_page: if has_previous_page {
      Some(page_num - 1)
    } else {
      None
    },
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
  router.route(ENDPOINT_PATH, get(search))
}
