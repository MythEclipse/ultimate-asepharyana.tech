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
pub const ENDPOINT_PATH: &str = "/api/api/anime2/detail/{slug}";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str =
  "Handles GET requests for the anime2/detail/{slug} endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "anime2";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "anime2_detail_slug";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
  pub name: String,
  pub slug: String,
  pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Link {
  pub name: String,
  pub url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DownloadItem {
  pub resolution: String,
  pub links: Vec<Link>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Recommendation {
  pub title: String,
  pub slug: String,
  pub poster: String,
  pub status: String,
  pub r#type: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeDetailData {
  pub title: String,
  pub alternative_title: String,
  pub poster: String,
  pub poster2: String,
  pub r#type: String,
  pub release_date: String,
  pub status: String,
  pub synopsis: String,
  pub studio: String,
  pub genres: Vec<Genre>,
  pub producers: Vec<String>,
  pub recommendations: Vec<Recommendation>,
  pub batch: Vec<DownloadItem>,
  pub ova: Vec<DownloadItem>,
  pub downloads: Vec<DownloadItem>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
  pub status: String,
  pub data: AnimeDetailData,
}

#[utoipa::path(
  get,
  params(("slug" = String, Path, description = "The slug identifier")),
  path = "/api/anime2/detail/{slug}",
  tag = "anime2",
  operation_id = "anime2_detail_slug",
  responses(
    (
      status = 200,
      description = "Handles GET requests for the anime2/detail/{slug} endpoint.",
      body = DetailResponse,
    ),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
  match fetch_anime_detail(&slug).await {
    Ok(data) =>
      Json(DetailResponse {
        status: "Ok".to_string(),
        data,
      }),
    Err(_) =>
      Json(DetailResponse {
        status: "Error".to_string(),
        data: AnimeDetailData {
          title: "".to_string(),
          alternative_title: "".to_string(),
          poster: "".to_string(),
          poster2: "".to_string(),
          r#type: "".to_string(),
          status: "".to_string(),
          release_date: "".to_string(),
          synopsis: "".to_string(),
          studio: "".to_string(),
          genres: vec![],
          producers: vec![],
          recommendations: vec![],
          batch: vec![],
          ova: vec![],
          downloads: vec![],
        },
      }),
  }
}

async fn fetch_anime_detail(slug: &str) -> Result<AnimeDetailData, Box<dyn std::error::Error>> {
  let url = format!("https://alqanime.net/{}/", slug);
  let client = reqwest::Client::new();
  let response = client.get(&url).send().await?;
  let html = response.text().await?;
  let document = Html::parse_document(&html);

  let title = document
    .select(&Selector::parse(".entry-title").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let alternative_title = document
    .select(&Selector::parse(".alter").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let poster = document
    .select(&Selector::parse(".thumb[itemprop=\"image\"] img.lazyload").unwrap())
    .next()
    .and_then(|e| e.value().attr("data-src"))
    .unwrap_or("")
    .to_string();

  let poster2 = document
    .select(&Selector::parse(".bixbox.animefull .bigcover .ime img.lazyload").unwrap())
    .next()
    .and_then(|e| e.value().attr("data-src"))
    .unwrap_or("")
    .to_string();

  let spe_selector = Selector::parse(".info-content .spe span").unwrap();

  let r#type = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Tipe:"))
    .and_then(|span| span.select(&Selector::parse("a").unwrap()).next())
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let release_date = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Dirilis:"))
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let status = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Status:"))
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let synopsis = document
    .select(&Selector::parse(".entry-content p").unwrap())
    .next()
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let studio = document
    .select(&spe_selector)
    .find(|e| e.text().collect::<String>().contains("Studio:"))
    .and_then(|span| span.select(&Selector::parse("a").unwrap()).next())
    .map(|e| e.text().collect::<String>().trim().to_string())
    .unwrap_or_default();

  let mut genres = Vec::new();
  for element in document.select(&Selector::parse(".genxed a").unwrap()) {
    let name = element.text().collect::<String>().trim().to_string();
    let anime_url = element.value().attr("href").unwrap_or("").to_string();
    let genre_slug = anime_url
      .split('/')
      .filter(|s| !s.is_empty())
      .last()
      .unwrap_or("")
      .to_string();
    genres.push(Genre { name, slug: genre_slug, anime_url });
  }

  let mut batch = Vec::new();
  let mut ova = Vec::new();
  let mut downloads = Vec::new();

  for element in document.select(&Selector::parse(".soraddl.dlone .soraurl").unwrap()) {
    let resolution = element
      .select(&Selector::parse(".res").unwrap())
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let mut links = Vec::new();
    for link_element in element.select(&Selector::parse(".slink a").unwrap()) {
      let name = link_element.text().collect::<String>().trim().to_string();
      let url = link_element.value().attr("href").unwrap_or("").to_string();
      links.push(Link { name, url });
    }

    let download_item = DownloadItem { resolution, links };

    // Determine category based on parent h3 text
    if let Some(h3) = element.select(&Selector::parse("h3").unwrap()).next() {
      let category = h3.text().collect::<String>().to_lowercase();
      if category.contains("batch") {
        batch.push(download_item);
      } else if category.contains("ova") {
        ova.push(download_item);
      } else {
        downloads.push(download_item);
      }
    } else {
      downloads.push(download_item);
    }
  }

  let mut recommendations = Vec::new();
  for element in document.select(&Selector::parse(".listupd .bs").unwrap()) {
    let title = element
      .select(&Selector::parse(".ntitle").unwrap())
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let anime_url = element
      .select(&Selector::parse("a").unwrap())
      .next()
      .and_then(|e| e.value().attr("href"))
      .unwrap_or("")
      .to_string();

    let rec_slug = anime_url
      .split('/')
      .filter(|s| !s.is_empty())
      .last()
      .unwrap_or("")
      .to_string();

    let poster = element
      .select(&Selector::parse("img").unwrap())
      .next()
      .and_then(|e|
        e
          .value()
          .attr("data-src")
          .or_else(|| e.value().attr("src"))
      )
      .unwrap_or("")
      .to_string();

    let status = element
      .select(&Selector::parse(".status").unwrap())
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    let r#type = element
      .select(&Selector::parse(".typez").unwrap())
      .next()
      .map(|e| e.text().collect::<String>().trim().to_string())
      .unwrap_or_default();

    recommendations.push(Recommendation {
      title,
      slug: rec_slug,
      poster,
      status,
      r#type,
    });
  }

  Ok(AnimeDetailData {
    title,
    alternative_title,
    poster,
    poster2,
    r#type,
    release_date,
    status,
    synopsis,
    studio,
    genres,
    producers: vec![], // Empty as per Next.js implementation
    recommendations,
    batch,
    ova,
    downloads,
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}