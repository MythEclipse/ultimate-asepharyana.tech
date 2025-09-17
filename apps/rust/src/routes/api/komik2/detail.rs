//! Handler for the detail endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use scraper::{ Html, Selector };
use tracing::{ info, error, warn };
use axum::extract::State;
use axum::http::StatusCode;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::urls::get_komik2_url;
use backoff::{ future::retry, ExponentialBackoff };
use std::time::Duration;
use deadpool_redis::redis::AsyncCommands;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik2/detail";
pub const ENDPOINT_DESCRIPTION: &str = "Retrieves details for a specific komik2 by ID.";
pub const ENDPOINT_TAG: &str = "komik2";
pub const OPERATION_ID: &str = "komik2_detail";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailData>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Chapter {
  pub chapter: String,
  pub date: String,
  pub chapter_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailData {
  pub title: String,
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

// Define selectors without using lazy_static to avoid initialization issues
fn get_title_selector() -> Selector {
  Selector::parse("h1#Judul, h1.entry-title, .entry-title, .title-series, .post-title").unwrap()
}

fn get_info_row_selector() -> Selector {
  Selector::parse(
    ".spe span, .inftable tr, .infos .infox .spe span, .info dd, .detail-info dd"
  ).unwrap()
}


fn get_poster_selector() -> Selector {
  Selector::parse("#Imgnovel, div.ims img, .thumb img, .poster img").unwrap()
}

fn get_description_selector() -> Selector {
  Selector::parse("article section p, .entry-content p, .desc p, .sinopsis, .desc-text").unwrap()
}

fn get_genre_selector() -> Selector {
  Selector::parse(".genre a, ul.genre li a, .tag a").unwrap()
}

fn get_chapter_list_selector() -> Selector {
  Selector::parse(
    "table#Daftar_Chapter tbody#daftarChapter tr, #chapter_list li, .eplister ul li, .chapter-list li"
  ).unwrap()
}

fn get_chapter_link_selector() -> Selector {
  Selector::parse("td.judulseries a, a.chapter, a, .chapter-item a").unwrap()
}

fn get_date_link_selector() -> Selector {
  Selector::parse(
    "td.tanggalseries, .rightarea .date, .epcontent .date, .udate, .chapter-date"
  ).unwrap()
}

fn get_release_date_selector() -> Selector {
  Selector::parse(".spe span").unwrap()
}

fn get_updated_on_selector() -> Selector {
  Selector::parse(".spe span").unwrap()
}

fn get_total_chapter_selector() -> Selector {
  Selector::parse(".spe span").unwrap()
}

// Helper function to find table rows containing specific text
fn find_table_row_with_text(
  selector: &Selector,
  document: &Html,
  text_fragments: &[&str]
) -> Option<String> {
  let lower_text_fragments: Vec<String> = text_fragments
    .iter()
    .map(|&s| s.to_lowercase())
    .collect();

  document
    .select(selector)
    .find(|row| {
      let row_text = row.text().collect::<String>();
      lower_text_fragments
        .iter()
        .any(|fragment| row_text.to_lowercase().contains(fragment))
    })
    .and_then(|row| {
      row
        .select(&Selector::parse("td:last-child").unwrap())
        .next()
        .map(|cell| cell.text().collect::<String>().trim().to_string())
    })
}

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("komik_id" = Option<String>, Query, description = "Comic/manga identifier", example = "sample_value")
    ),
    path = "/api/komik2/detail",
    tag = "komik2",
    operation_id = "komik2_detail",
    responses(
        (status = 200, description = "Retrieves details for a specific komik2 by ID.", body = DetailData),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
#[axum::debug_handler]
pub async fn detail(
  State(app_state): State<Arc<AppState>>,
  Query(params): Query<DetailQuery>
) -> Result<Json<DetailResponse>, (StatusCode, String)> {
  let start_time = std::time::Instant::now();
  let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
  info!("Handling request for komik2 detail: {}", komik_id);

  let cache_key = format!("komik2:detail:{}", komik_id);
  let mut conn = app_state.redis_pool.get().await.map_err(|e| {
    error!("Failed to get Redis connection: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
  })?;

  // Try to get cached data
  let cached_response: Option<String> = conn.get(&cache_key).await.map_err(|e| {
    error!("Failed to get data from Redis: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
  })?;

  if let Some(json_data_string) = cached_response {
    info!("Cache hit for key: {}", cache_key);
    let detail_response: DetailResponse = serde_json::from_str(&json_data_string).map_err(|e| {
      error!("Failed to deserialize cached data: {:?}", e);
      (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
    })?;
    return Ok(Json(detail_response));
  }

  match fetch_komik_detail(komik_id.clone()).await {
    Ok(data) => {
      // Validate that we got meaningful data
      let is_valid_response =
        !data.title.is_empty() || !data.chapters.is_empty() || !data.description.is_empty();

      if !is_valid_response {
        error!("Received empty response for komik_id: {}. All fields are empty.", komik_id);
        return Err((StatusCode::NOT_FOUND, "No data found for this komik".to_string()));
      }

      let detail_response = DetailResponse { status: true, data };
      let json_data = serde_json::to_string(&detail_response).map_err(|e| {
        error!("Failed to serialize response for caching: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e))
      })?;

      // Store in Redis with TTL
      conn.set_ex::<_, _, ()>(&cache_key, json_data, CACHE_TTL).await.map_err(|e| {
        error!("Failed to set data in Redis: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {}", e))
      })?;
      info!("Cache set for key: {}", cache_key);

      let total_duration = start_time.elapsed();
      info!("Successfully processed request for komik_id: {} in {:?}", komik_id, total_duration);
      Ok(Json(detail_response))
    }
    Err(e) => {
      let total_duration = start_time.elapsed();
      error!(
        "Failed to process request for komik_id: {} after {:?}, error: {:?}",
        komik_id,
        total_duration,
        e
      );

      // Provide more specific error messages
      let error_msg = match e.to_string().as_str() {
        "Empty response" => "No data received from the source website".to_string(),
        "Failed to fetch" => "Failed to connect to the source website".to_string(),
        "Timeout" => "Request timed out while connecting to source website".to_string(),
        _ => format!("Failed to fetch komik data: {}", e),
      };

      Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
    }
  }
}

async fn fetch_komik_detail(
  komik_id: String
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  let base_url = get_komik2_url();
  let url = format!("{}/manga/{}/", base_url, komik_id); // Correct URL format for komiku.org

  // Retry logic with exponential backoff
  let backoff = ExponentialBackoff {
    initial_interval: Duration::from_millis(500),
    max_interval: Duration::from_secs(10),
    multiplier: 2.0,
    max_elapsed_time: Some(Duration::from_secs(30)),
    ..Default::default()
  };

  let fetch_operation = || async {
    info!("Fetching URL: {}", url);
    match fetch_with_proxy(&url).await {
      Ok(response) => {
        let duration = start_time.elapsed();
        info!("Successfully fetched URL: {} in {:?}", url, duration);
        Ok(response.data)
      }
      Err(e) => {
        warn!("Failed to fetch URL: {}, error: {:?}", url, e);
        Err(backoff::Error::transient(e))
      }
    }
  };

  let html = retry(backoff, fetch_operation).await?;

  tokio::task::spawn_blocking(move ||
    parse_komik_detail_document(&Html::parse_document(&html))
  ).await?
}

fn parse_komik_detail_document(
  document: &Html,
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  info!("Starting to parse komik2 detail document");

  // Improved title extraction with fallback options
  let title = document
    .select(&get_title_selector())
    .next()
    .map(|e| {
      let text = e.text().collect::<String>().trim().to_string();
      // Remove common prefixes/suffixes that might be included
      text
        .replace("Komik ", "")
        .replace("Manga ", "")
        .replace("Manhua ", "")
        .replace("Manhwa ", "")
        .trim()
        .to_string()
    })
    .or_else(|| {
      // Fallback to try to extract title from h1 elements
      document
        .select(&Selector::parse("h1").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
    })
    .or_else(|| {
      // Final fallback to document title
      document
        .select(&Selector::parse("title").unwrap())
        .next()
        .map(|e| {
          let text = e.text().collect::<String>();
          if text.contains("Komik ") {
            text.replace("Komik ", "").trim().to_string()
          } else {
            text.trim().to_string()
          }
        })
    })
    .unwrap_or_default();

  let status = document
    .select(&get_info_row_selector())
    .find(|row| {
      let text = row.text().collect::<String>();
      text.to_lowercase().contains("status")
    })
    .map(|row| {
      let full_text = row.text().collect::<String>();
      let lower_text = full_text.to_lowercase();

      let status_value = if let Some(status_pos) = lower_text.find("status:") {
        &full_text[status_pos + 7..]
      } else if let Some(colon_pos) = full_text.find(':') {
        &full_text[colon_pos + 1..]
      } else {
        full_text.as_str()
      };

      status_value
        .replace('\n', " ")
        .replace('\t', " ")
        .replace("Status", "")
        .replace("Jenis Komik", "")
        .replace("Type", "")
        .trim()
        .to_string()
    })
    .unwrap_or_default();

  let r#type = document
    .select(&get_info_row_selector())
    .find(|row| {
      let text = row.text().collect::<String>();
      text.to_lowercase().contains("jenis komik") || text.to_lowercase().contains("type")
    })
    .map(|row| {
      let full_text = row.text().collect::<String>();
      let lower_text = full_text.to_lowercase();

      let type_value = if let Some(jk_pos) = lower_text.find("jenis komik:") {
        &full_text[jk_pos + 11..]
      } else if let Some(type_pos) = lower_text.find("type:") {
        &full_text[type_pos + 5..]
      } else if let Some(colon_pos) = full_text.find(':') {
        &full_text[colon_pos + 1..]
      } else {
        full_text.as_str()
      };

      type_value
        .replace('\n', " ")
        .replace('\t', " ")
        .replace("Jenis Komik", "")
        .replace("Type", "")
        .trim()
        .to_string()
    })
    .unwrap_or_default();

  let author = document
    .select(&get_info_row_selector())
    .find(|row| {
      let text = row.text().collect::<String>();
      text.to_lowercase().contains("pengarang") || text.to_lowercase().contains("author")
    })
    .map(|row| {
      let full_text = row.text().collect::<String>();
      let lower_text = full_text.to_lowercase();

      let author_name = if let Some(pos) = lower_text.find("pengarang:") {
        &full_text[pos + 10..]
      } else if let Some(pos) = lower_text.find("pengarang ") {
        &full_text[pos + 10..]
      } else if let Some(pos) = lower_text.find("author:") {
        &full_text[pos + 7..]
      } else if let Some(pos) = lower_text.find("author ") {
        &full_text[pos + 7..]
      } else if let Some(colon_pos) = full_text.find(':') {
        &full_text[colon_pos + 1..]
      } else {
        full_text.as_str()
      };

      let final_author = author_name
        .replace("Pengarang", "")
        .replace("Author", "")
        .replace("pengarang", "")
        .replace("author", "")
        .trim()
        .to_string();

      if final_author.starts_with("Pengarang") || final_author.starts_with("pengarang") {
        final_author.split_whitespace().skip(1).collect::<Vec<_>>().join(" ")
      } else {
        final_author
      }
    })
    .unwrap_or_default();

  let poster = document
    .select(&get_poster_selector())
    .next()
    .and_then(|e| e.value().attr("src"))
    .map(|s| s.split('?').next().unwrap_or(s).to_string())
    .unwrap_or_default();

  let description = document
    .select(&get_description_selector())
    .map(|e| e.text().collect::<String>())
    .filter(|t| t.len() > 50) // avoid tiny fragments
    .collect::<Vec<String>>()
    .join("\n")
    .trim()
    .to_string();

  // Extract release date, total chapter, and updated_on using specific selectors first
  let release_date = find_table_row_with_text(
    &get_release_date_selector(),
    document,
    &["tanggal rilis", "release date"]
  ).map(|date| {
    // Clean up whitespace including newlines and extra spaces
    date.replace('\n', " ").replace('\t', " ").trim().to_string()
  })
  .unwrap_or_else(|| {
    // Fallback to last chapter date if no specific release date found
    document
      .select(&get_chapter_list_selector())
      .last()
      .and_then(|last| last.select(&get_date_link_selector()).next())
      .map(|e| e.text().collect::<String>().replace('\n', " ").replace('\t', " ").trim().to_string())
      .unwrap_or_default()
  });

  let total_chapter = find_table_row_with_text(
    &get_total_chapter_selector(),
    document,
    &["total chapter", "total chapters"]
  ).unwrap_or_else(|| {
    // Fallback to chapter count if no specific total found
    let count = document.select(&get_chapter_list_selector()).count();
    if count > 0 {
      count.to_string()
    } else {
      String::new()
    }
  });

  let updated_on = find_table_row_with_text(
    &get_updated_on_selector(),
    document,
    &["diperbarui", "updated"]
  )
    .or_else(|| {
      // Look for updated date in the "judul2" class which contains "pembaca • X waktu lalu"
      document
        .select(&Selector::parse(".bge .kan .judul2").unwrap())
        .next()
        .map(|e| {
          let text = e.text().collect::<String>().trim().to_string();
          // Extract the part after "• " which contains the time information
          text.split("• ").nth(1).unwrap_or("").trim().to_string()
        })
    })
    .unwrap_or_else(|| {
      // Fallback to first chapter date if no specific updated date found
      document
        .select(&get_chapter_list_selector())
        .next()
        .and_then(|first| first.select(&get_date_link_selector()).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
    });

  let mut genres = Vec::new();
  for element in document.select(&get_genre_selector()) {
    let genre = element.text().collect::<String>().trim().to_string();
    if !genre.is_empty() {
      genres.push(genre);
    }
  }

  // Improved chapter parsing with better data extraction
  let mut chapters = Vec::new();
  for el in document.select(&get_chapter_list_selector()) {
    let chapter_link_element = el.select(&get_chapter_link_selector()).next();
    let date_element = el.select(&get_date_link_selector()).next();

    let chapter = chapter_link_element
      .as_ref()
      .map(|e| {
        let text = e.text().collect::<String>();
        text
          .split_whitespace()
          .find(|&s| s.chars().any(|c| c.is_digit(10)))
          .map(|num_part| num_part.to_string())
          .unwrap_or(text.trim().to_string())
      })
      .unwrap_or_default();

    let date = date_element
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let chapter_id = chapter_link_element
      .and_then(|e| e.value().attr("href"))
      .map(|href| {
        href
          .split('/')
          .filter(|s| !s.is_empty())
          .last() // Get the last segment, which is usually the ID
          .unwrap_or("")
          .to_string()
      })
      .unwrap_or_default();

    if !chapter_id.is_empty() {
      chapters.push(Chapter { chapter, date, chapter_id });
    }
  }

  let duration = start_time.elapsed();
  info!("Parsed komik2 detail document in {:?}", duration);

  Ok(DetailData {
    title,
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