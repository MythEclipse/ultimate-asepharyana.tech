use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use regex::Regex;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::komik_base_url::get_cached_komik_base_url;
use tracing::{ error, info, warn };
use urlencoding;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/komik/search";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Searches for komik based on query parameters.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "komik";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "komik_search";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaItem {
  pub title: String,
  pub poster: String,
  pub chapter: String,
  pub score: String,
  pub date: String,
  pub r#type: String,
  pub slug: String,
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
  pub data: Vec<MangaItem>,
  pub pagination: Pagination,
}

#[derive(Deserialize)]
pub struct SearchQuery {
  pub query: Option<String>,
  pub page: Option<u32>,
}

#[utoipa::path(
  get,
  path = "/api/komik/search",
  tag = "komik",
  operation_id = "komik_search",
  params(
    ("query" = Option<String>, Query, description = "Search query for komik"),
    ("page" = Option<u32>, Query, description = "Page number for pagination")
  ),
  responses(
    (
      status = 200,
      description = "Searches for komik based on query parameters.",
      body = SearchResponse,
    ),
    (status = 400, description = "Bad Request", body = String),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
  let query = params.query.unwrap_or_default();
  let page = params.page.unwrap_or(1);

  let base_url = match get_cached_komik_base_url(false).await {
    Ok(url) => url,
    Err(_) => {
      warn!("[search] Failed to get cached base URL, trying refresh");
      match get_cached_komik_base_url(true).await {
        Ok(url) => url,
        Err(e) => {
          error!("[search] Failed to get base URL: {:?}", e);
          return Json(SearchResponse {
            data: vec![],
            pagination: Pagination {
              current_page: page,
              last_visible_page: page,
              has_next_page: false,
              next_page: None,
              has_previous_page: false,
              previous_page: None,
            },
          });
        }
      }
    }
  };

  let url = if query.is_empty() {
    format!("{}/page/{}/", base_url, page)
  } else {
    format!("{}/page/{}/?s={}", base_url, page, urlencoding::encode(&query))
  };

  match fetch_and_parse_search(&url, &query, page).await {
    Ok(response) => Json(response),
    Err(e) => {
      error!("[search] Failed to fetch and parse search: {:?}", e);
      Json(SearchResponse {
        data: vec![],
        pagination: Pagination {
          current_page: page,
          last_visible_page: page,
          has_next_page: false,
          next_page: None,
          has_previous_page: false,
          previous_page: None,
        },
      })
    }
  }
}

async fn fetch_and_parse_search(
  url: &str,
  _query: &str,
  page: u32
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
  info!("[fetch_and_parse_search] Starting fetch for URL: {}", url);

  // Use fetch_with_proxy which includes comprehensive retry logic
  let html = match fetch_with_proxy(url).await {
    Ok(result) => {
      info!("[fetch_and_parse_search] fetch_with_proxy successful");
      result.data
    }
    Err(e) => {
      error!("[fetch_and_parse_search] All proxy attempts failed: {:?}", e);
      return Err(Box::new(e));
    }
  };

  let document = Html::parse_document(&html);

  let animposx_selector = Selector::parse(".animposx").unwrap();
  let title_selector = Selector::parse(".tt h4").unwrap();
  let img_selector = Selector::parse("img").unwrap();
  let chapter_selector = Selector::parse(".lsch a").unwrap();
  let score_selector = Selector::parse("i").unwrap();
  let date_selector = Selector::parse(".datech").unwrap();
  let type_selector = Selector::parse(".typeflag").unwrap();
  let link_selector = Selector::parse("a").unwrap();

  let mut data = Vec::new();

  for element in document.select(&animposx_selector) {
    let title = element
      .select(&title_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let mut poster = element
      .select(&img_selector)
      .next()
      .and_then(|e| e.value().attr("src"))
      .unwrap_or("")
      .to_string();
    if let Some(pos) = poster.find('?') {
      poster = poster[..pos].to_string();
    }

    let chapter_text = element
      .select(&chapter_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();
    let chapter = Regex::new(r"\d+(\.\d+)?")
      .unwrap()
      .find(&chapter_text)
      .map(|m| m.as_str().to_string())
      .unwrap_or_default();

    let score = element
      .select(&score_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let date = element
      .select(&date_selector)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let r#type = element
      .select(&type_selector)
      .next()
      .and_then(|e| e.value().attr("class"))
      .and_then(|class| class.split_whitespace().nth(1))
      .unwrap_or("")
      .to_string();

    let slug = element
      .select(&link_selector)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(4))
      .unwrap_or("")
      .to_string();

    if !title.is_empty() {
      data.push(MangaItem {
        title,
        poster,
        chapter,
        score,
        date,
        r#type,
        slug,
      });
    }
  }

  let pagination = parse_pagination(&document, page);

  Ok(SearchResponse {
    data,
    pagination,
  })
}

fn parse_pagination(document: &Html, current_page: u32) -> Pagination {
  let page_selectors = Selector::parse(".pagination a:not(.next)").unwrap();
  let next_selector = Selector::parse(".pagination .next").unwrap();
  let prev_selector = Selector::parse(".pagination .prev").unwrap();

  let mut last_visible_page = current_page;
  for element in document.select(&page_selectors) {
    if let Ok(page) = element.text().collect::<String>().trim().parse::<u32>() {
      if page > last_visible_page {
        last_visible_page = page;
      }
    }
  }

  let has_next_page = document.select(&next_selector).next().is_some();
  let next_page = if has_next_page && current_page < last_visible_page {
    Some(current_page + 1)
  } else {
    None
  };

  let has_previous_page = document.select(&prev_selector).next().is_some();
  let previous_page = if has_previous_page && current_page > 1 {
    Some(current_page - 1)
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