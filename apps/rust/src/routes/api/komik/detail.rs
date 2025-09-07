//! Handler for the detail endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use rust_lib::fetch_with_proxy::fetch_with_proxy_only;
use rust_lib::komik_base_url::get_cached_komik_base_url;
use scraper::{ Html, Selector };
use tracing::{ info, error };

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

#[derive(Deserialize)]
pub struct DetailQuery {
  pub komik_id: Option<String>,
}

#[utoipa::path(
  get,
  path = "/api/komik/detail",
  tag = "komik",
  operation_id = "komik_detail",
  responses(
    (
      status = 200,
      description = "Retrieves details for a specific komik by ID.",
      body = DetailData,
    ),
    (status = 500, description = "Internal Server Error", body = DetailData)
  )
)]
pub async fn detail(Query(params): Query<DetailQuery>) -> impl IntoResponse {
  let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());

  match get_cached_komik_base_url(false).await {
    Ok(base_url) => {
      match fetch_and_parse_detail(&komik_id, &base_url).await {
       Ok(data) => {
         info!("[komik][detail] Success for komik_id: {}", komik_id);
         Json(data)
       }
       Err(e) => {
         error!("[komik][detail] Error parsing detail for {}: {:?}", komik_id, e);
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
  let url = format!("{}/komik/{}", base_url, komik_id);
  info!("[fetch_and_parse_detail] Fetching URL: {}", url);

  let response = fetch_with_proxy_only(&url).await?;
  let html = response.data;
  let document = Html::parse_document(&html);

  // Title
  let title = document
    .select(&Selector::parse("h1.entry-title").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let spe_selector = Selector::parse(".spe span").unwrap();

  // Alternative Title
  let alternative_title = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Judul Alternatif:"))
    .map(|e| e.text().collect::<String>().replace("Judul Alternatif:", "").trim().to_string())
    .unwrap_or_default();

  // Score
  let score = document
    .select(&Selector::parse(".rtg > div > i").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Poster
  let mut poster = document
    .select(&Selector::parse(".thumb img").unwrap())
    .next()
    .and_then(|e| e.value().attr("src"))
    .unwrap_or("")
    .to_string();
  if let Some(pos) = poster.find('?') {
    poster = poster[..pos].to_string();
  }

  // Description
  let description = document
    .select(
      &Selector::parse(
        "#sinopsis > section > div > div.entry-content.entry-content-single > p"
      ).unwrap()
    )
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
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Jenis Komik:"))
    .and_then(|span| span.select(&Selector::parse("a").unwrap()).next())
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Release Date
  let release_date = document
    .select(&Selector::parse("#chapter_list > ul > li > span.dt").unwrap())
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
    .select(&Selector::parse("#chapter_list > ul > li > span.lchx").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Updated On
  let updated_on = document
    .select(&Selector::parse("#chapter_list > ul > li > span.dt").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  // Genres
  let mut genres = Vec::new();
  for element in document.select(&Selector::parse(".genre-info a").unwrap()) {
    genres.push(element.text().collect::<String>().trim().to_string());
  }

  // Chapters
  let mut chapters = Vec::new();
  for element in document.select(&Selector::parse("#chapter_list ul li").unwrap()) {
    let chapter = element
      .select(&Selector::parse(".lchx a").unwrap())
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let date = element
      .select(&Selector::parse(".dt a").unwrap())
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let chapter_id = element
      .select(&Selector::parse(".lchx a").unwrap())
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