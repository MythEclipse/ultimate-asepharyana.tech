//! Handler for the detail endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::komik_base_url::get_cached_komik_base_url;
use scraper::{ Html, Selector };
use tracing::{ info, error };
use lazy_static::lazy_static;
use std::time::Instant;
use tokio::time::{ sleep, Duration };

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

async fn fetch_with_retry(
  url: &str,
  max_retries: u32
) -> Result<String, Box<dyn std::error::Error>> {
  let mut attempt = 0;
  loop {
    match fetch_with_proxy(url).await {
      Ok(response) => {
        return Ok(response.data);
      }
      Err(e) => {
        attempt += 1;
        if attempt > max_retries {
          error!("Failed to fetch {} after {} attempts: {:?}", url, max_retries, e);
          return Err(Box::new(e));
        }
        let delay = Duration::from_millis((2u64).pow(attempt) * 100);
        info!("Retrying fetch for {} in {:?}", url, delay);
        sleep(delay).await;
      }
    }
  }
}

#[utoipa::path(
    get,
    path = "/api/komik/detail",
    tag = "komik",
    operation_id = "komik_detail",
    params(
        ("komik_id" = Option<String>, Query, description = "The unique identifier for the komik (typically the slug or URL path)")
    ),
    responses(
        (status = 200, description = "Retrieves details for a specific komik by ID.", body = DetailData),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn detail(Query(params): Query<DetailQuery>) -> impl IntoResponse {
  let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
  let start = Instant::now();
  info!("Starting detail request for komik_id {}", komik_id);

  match get_cached_komik_base_url(false).await {
    Ok(base_url) => {
      match fetch_and_parse_detail(&komik_id, &base_url).await {
        Ok(data) => {
          info!("[komik][detail] Success for komik_id: {}", komik_id);
          info!("Detail request completed in {:?}", start.elapsed());
          Json(data)
        }
        Err(e) => {
          error!("[komik][detail] Error parsing detail for {}: {:?}", komik_id, e);
          info!("Detail request completed in {:?}", start.elapsed());
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
      }
    }
    Err(e) => {
      error!("[komik][detail] Error getting base URL: {:?}", e);
      info!("Detail request completed in {:?}", start.elapsed());
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
  }
}

async fn fetch_and_parse_detail(
  komik_id: &str,
  base_url: &str
) -> Result<DetailData, Box<dyn std::error::Error>> {
  let start = Instant::now();
  let url = format!("{}/komik/{}", base_url, komik_id);
  info!("[fetch_and_parse_detail] Fetching URL: {}", url);

  let html = fetch_with_retry(&url, 3).await?;
  let document = Html::parse_document(&html);

  // Title
  let title = document
    .select(&*TITLE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let spe_selector = &*SPE_SELECTOR;

  // Alternative Title
  let alternative_title = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Judul Alternatif:"))
    .map(|e| e.text().collect::<String>().replace("Judul Alternatif:", "").trim().to_string())
    .unwrap_or_default();

  // Score
  let score = document
    .select(&*SCORE_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Poster
  let mut poster = document
    .select(&*POSTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();
  if let Some(pos) = poster.find('?') {
    poster = poster[..pos].to_string();
  }

  // Description
  let description = document
    .select(&*DESC_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Status
  let status = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Status:"))
    .map(|e| e.text().collect::<String>().replace("Status:", "").trim().to_string())
    .unwrap_or_default();

  // Type
  let r#type = document
    .select(spe_selector)
    .find(|e| e.text().collect::<String>().contains("Jenis Komik:"))
    .and_then(|span| span.select(&*A_SELECTOR).next())
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Release Date
  let release_date = document
    .select(&*RELEASE_DATE_SELECTOR)
    .last()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Author
  let author = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Pengarang:"))
    .map(|e| e.text().collect::<String>().replace("Pengarang:", "").trim().to_string())
    .unwrap_or_default();

  // Total Chapter
  let total_chapter = document
    .select(&*TOTAL_CHAPTER_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Updated On
  let updated_on = document
    .select(&*UPDATED_ON_SELECTOR)
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Genres
  let mut genres = Vec::new();
  for element in document.select(&*GENRE_SELECTOR) {
    genres.push(element.text().collect::<String>().trim().to_string());
  }

  // Chapters
  let mut chapters = Vec::new();
  for element in document.select(&*CHAPTER_LIST_SELECTOR) {
    let chapter = element
      .select(&*CHAPTER_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let date = element
      .select(&*DATE_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let chapter_id = element
      .select(&*CHAPTER_LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .and_then(|href| href.split('/').nth(3))
      .unwrap_or("")
      .to_string();

    chapters.push(Chapter {
      chapter,
      date,
      chapter_id,
    });
  }

  info!("[fetch_and_parse_detail] Successfully parsed detail for {}", komik_id);
  info!("Fetched and parsed detail in {:?}", start.elapsed());
  Ok(DetailData {
    title,
    alternative_title,
    score,
    poster,
    description,
    status,
    r#type,
    release_date,
    author,
    total_chapter,
    updated_on,
    genres,
    chapters,
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
  router.route(ENDPOINT_PATH, get(detail))
}
