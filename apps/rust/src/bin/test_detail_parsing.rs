use axum::{extract::Query, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use scraper::{Html, Selector};
use tracing::{info, error, warn};
use lazy_static::lazy_static;
use axum::extract::State;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::urls::get_komik2_url;
use backoff::{future::retry, ExponentialBackoff};
use std::time::Duration;
use tokio;

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
  // More specific title selectors including common patterns from the site
  static ref TITLE_SELECTOR: Selector = Selector::parse(
    "h1#Judul, h1.entry-title, .entry-title, .title-series, .post-title"
  ).unwrap();

  // More specific info row selectors for better data extraction
  static ref INFO_ROW_SELECTOR: Selector = Selector::parse(
    ".spe span, .inftable tr, .infos .infox .spe span, .info dd, .detail-info dd"
  ).unwrap();

  // More specific score selectors - target elements with rating/score data
  static ref SCORE_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Rating:'), .score, .rating, .numscore, .srating"
  ).unwrap();

  // More specific alternative title selectors
  static ref ALTERNATIVE_TITLE_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Judul Alternatif:'), .spe span:contains('Alternative:')"
  ).unwrap();

  static ref POSTER_SELECTOR: Selector = Selector::parse("#Imgnovel, div.ims img, .thumb img, .poster img").unwrap();
  static ref DESCRIPTION_SELECTOR: Selector = Selector::parse(
    "article section p, .entry-content p, .desc p, .sinopsis, .desc-text"
  ).unwrap();

  static ref GENRE_SELECTOR: Selector = Selector::parse(".genre a, ul.genre li a, .tag a").unwrap();

  // More specific chapter selectors
  static ref CHAPTER_LIST_SELECTOR: Selector = Selector::parse(
    "table#Daftar_Chapter tbody#daftarChapter tr, #chapter_list li, .eplister ul li, .chapter-list li"
  ).unwrap();

  static ref CHAPTER_LINK_SELECTOR: Selector = Selector::parse(
    "td.judulseries a, a.chapter, a, .chapter-item a"
  ).unwrap();

  static ref DATE_LINK_SELECTOR: Selector = Selector::parse(
    "td.tanggalseries, .rightarea .date, .epcontent .date, .udate, .chapter-date"
  ).unwrap();

  // Selector for release date
  static ref RELEASE_DATE_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Tanggal Rilis:'), .spe span:contains('Release Date:')"
  ).unwrap();

  // Selector for updated on date
  static ref UPDATED_ON_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Diperbarui:'), .spe span:contains('Updated:')"
  ).unwrap();

  // Selector for total chapters
  static ref TOTAL_CHAPTER_SELECTOR: Selector = Selector::parse(
    ".spe span:contains('Total Chapter:'), .spe span:contains('Total Chapters:')"
  ).unwrap();
}

async fn fetch_komik_detail(
  komik_id: String
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  let base_url = get_komik2_url();
  let url = format!("{}/manga/{}", base_url, komik_id);

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
    parse_komik_detail_document(&Html::parse_document(&html), &komik_id)
  ).await?
}

fn parse_komik_detail_document(
  document: &Html,
  _komik_id: &str
) -> Result<DetailData, Box<dyn std::error::Error + Send + Sync>> {
  let start_time = std::time::Instant::now();
  info!("Starting to parse komik2 detail document");

  // Improved title extraction with fallback options
  let title = document
    .select(&TITLE_SELECTOR)
    .next()
    .map(|e| {
      let text = e.text().collect::<String>().trim().to_string();
      // Remove common prefixes/suffixes that might be included
      text.replace("Komik ", "")
          .replace("Manga ", "")
          .replace("Manhua ", "")
          .replace("Manhwa ", "")
          .trim()
          .to_string()
    })
    .unwrap_or_default();

  // Extract labeled fields from info rows with more specific selectors
  let alternative_title = document
    .select(&ALTERNATIVE_TITLE_SELECTOR)
    .next()
    .map(|e| {
      let text = e.text().collect::<String>().trim().to_string();
      text.replace("Judul Alternatif:", "")
          .replace("Alternative:", "")
          .trim()
          .to_string()
    })
    .unwrap_or_default();

  let status = document
    .select(&INFO_ROW_SELECTOR)
    .find(|row| row.text().collect::<String>().to_lowercase().contains("status"))
    .map(|row| {
      let txt = row.text().collect::<String>().trim().to_string();
      txt.replace("Status:", "")
          .replace("Status :", "")
          .trim()
          .to_string()
    })
    .unwrap_or_default();

  let r#type = document
    .select(&INFO_ROW_SELECTOR)
    .find(|row| row.text().collect::<String>().to_lowercase().contains("jenis komik") ||
                row.text().collect::<String>().to_lowercase().contains("type"))
    .map(|row| {
      let txt = row.text().collect::<String>().trim().to_string();
      txt.replace("Jenis Komik:", "")
          .replace("Type:", "")
          .replace("Jenis :", "")
          .trim()
          .to_string()
    })
    .unwrap_or_default();

  let author = document
    .select(&INFO_ROW_SELECTOR)
    .find(|row| row.text().collect::<String>().to_lowercase().contains("pengarang") ||
                row.text().collect::<String>().to_lowercase().contains("author"))
    .map(|row| {
      let txt = row.text().collect::<String>().trim().to_string();
      txt.replace("Pengarang:", "")
          .replace("Author:", "")
          .replace("Pengarang :", "")
          .trim()
          .to_string()
    })
    .unwrap_or_default();

  // Improved score extraction - look for numeric values or specific rating formats
  let score = document
    .select(&SCORE_SELECTOR)
    .find(|e| {
      let text = e.text().collect::<String>().to_lowercase();
      text.contains("rating") || text.contains("score") ||
      text.chars().any(|c| c.is_digit(10) || c == '.' || c == ',')
    })
    .map(|e| {
      let text = e.text().collect::<String>();
      // Extract numeric score if available (e.g., "8.5/10" or "4.2")
      let numeric_score = text.split_whitespace()
          .find(|&s| s.chars().any(|c| c.is_digit(10)))
          .unwrap_or(&text);

      // Clean up the score - keep only numbers, dots, and slashes
      let cleaned = numeric_score.chars()
          .filter(|&c| c.is_digit(10) || c == '.' || c == ',' || c == '/')
          .collect::<String>();

      // If we have something like "Up 1", keep it but prefer numeric scores
      if cleaned.len() < 2 {
        text.replace("Rating:", "")
            .replace("Score:", "")
            .trim()
            .to_string()
      } else {
        cleaned
      }
    })
    .unwrap_or_default();

  let poster = document
    .select(&POSTER_SELECTOR)
    .next()
    .and_then(|e| e.value().attr("src"))
    .map(|s| s.split('?').next().unwrap_or(s).to_string())
    .unwrap_or_default();

  let description = document
    .select(&DESCRIPTION_SELECTOR)
    .map(|e| e.text().collect::<String>())
    .filter(|t| t.len() > 50) // avoid tiny fragments
    .collect::<Vec<String>>()
    .join("\n")
    .trim()
    .to_string();

  // Extract release date, total chapter, and updated_on using specific selectors first
  let release_date = document
    .select(&RELEASE_DATE_SELECTOR)
    .next()
    .map(|e| {
      let text = e.text().collect::<String>().trim().to_string();
      text.replace("Tanggal Rilis:", "")
          .replace("Release Date:", "")
          .trim()
          .to_string()
    })
    .unwrap_or_else(|| {
      // Fallback to last chapter date if no specific release date found
      document.select(&CHAPTER_LIST_SELECTOR)
          .last()
          .and_then(|last| last.select(&DATE_LINK_SELECTOR).next())
          .map(|e| e.text().collect::<String>().trim().to_string())
          .unwrap_or_default()
    });

  let total_chapter = document
    .select(&TOTAL_CHAPTER_SELECTOR)
    .next()
    .map(|e| {
      let text = e.text().collect::<String>().trim().to_string();
      text.replace("Total Chapter:", "")
          .replace("Total Chapters:", "")
          .trim()
          .to_string()
    })
    .unwrap_or_else(|| {
      // Fallback to chapter count if no specific total found
      let count = document.select(&CHAPTER_LIST_SELECTOR).count();
      if count > 0 {
        count.to_string()
      } else {
        String::new()
      }
    });

  let updated_on = document
    .select(&UPDATED_ON_SELECTOR)
    .next()
    .map(|e| {
      let text = e.text().collect::<String>().trim().to_string();
      text.replace("Diperbarui:", "")
          .replace("Updated:", "")
          .trim()
          .to_string()
    })
    .unwrap_or_else(|| {
      // Fallback to first chapter date if no specific updated date found
      document.select(&CHAPTER_LIST_SELECTOR)
          .next()
          .and_then(|first| first.select(&DATE_LINK_SELECTOR).next())
          .map(|e| e.text().collect::<String>().trim().to_string())
          .unwrap_or_default()
    });

  let mut genres = Vec::new();
  for element in document.select(&GENRE_SELECTOR) {
    let genre = element.text().collect::<String>().trim().to_string();
    if !genre.is_empty() {
      genres.push(genre);
    }
  }

  // Improved chapter parsing with better data extraction
  let mut chapters = Vec::new();
  for el in document.select(&CHAPTER_LIST_SELECTOR) {
    // Extract chapter title/number
    let chapter = el
      .select(&CHAPTER_LINK_SELECTOR)
      .next()
      .map(|e| {
        let text = e.text().collect::<String>().trim().to_string();
        // Extract numeric chapter if available (e.g., "Chapter 123" -> "123")
        text.split_whitespace()
            .find(|&s| s.chars().any(|c| c.is_digit(10)))
            .map(|num_part| num_part.to_string())
            .unwrap_or(text)
      })
      .unwrap_or_default();

    // Extract date
    let date = el
      .select(&DATE_LINK_SELECTOR)
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    // Extract chapter ID from URL
    let chapter_id = el
      .select(&CHAPTER_LINK_SELECTOR)
      .next()
      .and_then(|e| e.value().attr("href"))
      .map(|href| {
        let parts: Vec<&str> = href
          .split('/')
          .filter(|s| !s.is_empty())
          .collect();
        // Try to find segment after known category (manga|manhua|manhwa)
        if let Some(pos) = parts.iter().position(|s|
          *s == "manga" || *s == "manhua" || *s == "manhwa" ||
          *s == "chapter" || *s == "chapters"
        ) {
          parts.get(pos + 1).cloned().unwrap_or("").to_string()
        } else {
          // Fallback to last segment or full path if nothing else works
          parts.last().cloned().unwrap_or("").to_string()
        }
      })
      .unwrap_or_default();

    // Only add chapter if it has at least a chapter ID
    if !chapter_id.is_empty() {
      chapters.push(Chapter { chapter, date, chapter_id });
    }
  }

  let duration = start_time.elapsed();
  info!("Parsed komik2 detail document in {:?}", duration);

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Test with a popular manga
  let komik_id = "naruto";
  println!("Testing detail parsing for: {}", komik_id);

  match fetch_komik_detail(komik_id.to_string()).await {
    Ok(data) => {
      println!("✅ Successfully parsed detail data:");
      println!("==============================");
      println!("Title: {}", data.title);
      println!("Alternative Title: {}", data.alternative_title);
      println!("Score: {}", data.score);
      println!("Updated On: {}", data.updated_on);
      println!("Total Chapters: {}", data.total_chapter);
      println!("Status: {}", data.status);
      println!("Type: {}", data.r#type);
      println!("Author: {}", data.author);
      println!("Genres: {:?}", data.genres);
      println!("Chapters Found: {}", data.chapters.len());

      // Print first few chapters as sample
      if !data.chapters.is_empty() {
        println!("First Chapter: {}", data.chapters[0].chapter);
        println!("First Chapter ID: {}", data.chapters[0].chapter_id);
        println!("First Chapter Date: {}", data.chapters[0].date);
      }

      // Verify specific fields mentioned in the task
      println!("\n🔍 Task Verification:");
      println!("- 'chapter' field: {}", if data.chapters.len() > 0 { "✅ Populated" } else { "❌ Empty" });
      println!("- 'score' field: {}", if !data.score.is_empty() { "✅ Populated" } else { "❌ Empty" });
      println!("- 'title' field: {}", if !data.title.is_empty() { "✅ Populated" } else { "❌ Empty" });
      println!("- 'alternative_title' field: {}", if !data.alternative_title.is_empty() { "✅ Populated" } else { "❌ Empty" });
      println!("- 'updated_on' field: {}", if !data.updated_on.is_empty() { "✅ Populated" } else { "❌ Empty" });
    }
    Err(e) => {
      println!("❌ Failed to parse detail data: {:?}", e);
    }
  }

  Ok(())
}
