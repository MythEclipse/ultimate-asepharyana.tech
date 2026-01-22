use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/detail/{slug}";
pub const ENDPOINT_DESCRIPTION: &str =
    "Handles GET requests for the anime2/detail/{slug} endpoint.";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_detail_slug";
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

// Pre-compiled CSS selectors for performance
lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse(".entry-title").unwrap();
    static ref ALT_TITLE_SELECTOR: Selector = Selector::parse(".alter").unwrap();
    static ref POSTER_SELECTOR: Selector =
        Selector::parse(".thumb[itemprop=\"image\"] img.lazyload").unwrap();
    static ref POSTER2_SELECTOR: Selector =
        Selector::parse(".bixbox.animefull .bigcover .ime img.lazyload").unwrap();
    static ref SPE_SPAN_SELECTOR: Selector = Selector::parse(".info-content .spe span").unwrap();
    static ref A_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref SYNOPSIS_SELECTOR: Selector = Selector::parse(".entry-content p").unwrap();
    static ref GENRE_SELECTOR: Selector = Selector::parse(".genxed a").unwrap();
    static ref DOWNLOAD_CONTAINER_SELECTOR: Selector =
        Selector::parse(".soraddl .soraurl").unwrap();
    static ref RESOLUTION_SELECTOR: Selector = Selector::parse(".res").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse(".slink a").unwrap();
    static ref H3_SELECTOR: Selector = Selector::parse("h3").unwrap();
    static ref RECOMMENDATION_SELECTOR: Selector = Selector::parse(".listupd .bs").unwrap();
    static ref REC_TITLE_SELECTOR: Selector = Selector::parse(".ntitle").unwrap();
    static ref REC_IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref STATUS_SELECTOR: Selector = Selector::parse(".status").unwrap();
    static ref TYPE_SELECTOR: Selector = Selector::parse(".typez").unwrap();
}

// Pre-compiled regex for slug extraction
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "naruto-shippuden-episode-1")
    ),
    path = "/api/anime2/detail/{slug}",
    tag = "anime2",
    operation_id = "anime2_detail_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/detail/{slug} endpoint.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _start_time = std::time::Instant::now();
    info!("Handling request for anime detail slug: {}", slug);

    let cache_key = format!("anime2:detail:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let data = fetch_anime_detail(slug.clone())
                .await
                .map_err(|e| e.to_string())?;
            Ok(DetailResponse {
                status: "Ok".to_string(),
                data,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_anime_detail(
    slug: String,
) -> Result<AnimeDetailData, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://alqanime.si/{}/", slug);

    // Retry logic with exponential backoff
    let backoff = default_backoff();

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => {
                info!("Successfully fetched URL: {}", url);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch URL: {}, error: {:?}", url, e);
                Err(transient(e))
            }
        }
    };

    match retry(backoff, fetch_operation).await {
        Ok(html) => {
            let html_clone = html.clone(); // Clone the html string
            let slug_clone = slug.clone();

            let parse_result = tokio::task::spawn_blocking(move || {
                parse_anime_detail_document(&Html::parse_document(&html_clone), &slug_clone)
            })
            .await;

            match parse_result {
                Ok(inner_result) => match inner_result {
                    Ok(data) => Ok(data),
                    Err(e) => Err(e),
                },
                Err(join_err) => {
                    Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>)
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch URL after retries: {}, error: {:?}", url, e);
            Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        }
    }
}

fn parse_anime_detail_document(
    document: &Html,
    slug: &str,
) -> Result<AnimeDetailData, Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    info!("Starting to parse anime detail document for slug: {}", slug);

    let title = document
        .select(&TITLE_SELECTOR)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let alternative_title = document
        .select(&ALT_TITLE_SELECTOR)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let poster = document
        .select(&POSTER_SELECTOR)
        .next()
        .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
        .unwrap_or("")
        .to_string();

    let poster2 = document
        .select(&POSTER2_SELECTOR)
        .next()
        .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
        .unwrap_or("")
        .to_string();

    let r#type = document
        .select(&SPE_SPAN_SELECTOR)
        .find(|e| e.text().collect::<String>().contains("Tipe:"))
        .and_then(|span| span.select(&A_SELECTOR).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let release_date = document
        .select(&SPE_SPAN_SELECTOR)
        .find(|e| e.text().collect::<String>().contains("Dirilis:"))
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let status = document
        .select(&SPE_SPAN_SELECTOR)
        .find(|e| e.text().collect::<String>().contains("Status:"))
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let synopsis = document
        .select(&SYNOPSIS_SELECTOR)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let studio = document
        .select(&SPE_SPAN_SELECTOR)
        .find(|e| e.text().collect::<String>().contains("Studio:"))
        .and_then(|span| span.select(&A_SELECTOR).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let mut genres = Vec::new();
    for element in document.select(&GENRE_SELECTOR) {
        let name = element.text().collect::<String>().trim().to_string();
        let anime_url = element.value().attr("href").unwrap_or("").to_string();
        let genre_slug = SLUG_REGEX
            .captures(&anime_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();
        genres.push(Genre {
            name,
            slug: genre_slug,
            anime_url,
        });
    }

    let mut batch = Vec::new();
    let mut ova = Vec::new();
    let mut downloads = Vec::new();

    for element in document.select(&DOWNLOAD_CONTAINER_SELECTOR) {
        let resolution = element
            .select(&RESOLUTION_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let mut links = Vec::new();
        for link_element in element.select(&LINK_SELECTOR) {
            let name = link_element.text().collect::<String>().trim().to_string();
            let url = link_element.value().attr("href").unwrap_or("").to_string();
            links.push(Link { name, url });
        }

        let download_item = DownloadItem { resolution, links };

        // Determine category based on parent h3 text
        if let Some(h3) = element.select(&H3_SELECTOR).next() {
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
    for element in document.select(&RECOMMENDATION_SELECTOR) {
        let title = element
            .select(&REC_TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let anime_url = element
            .select(&A_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let rec_slug = SLUG_REGEX
            .captures(&anime_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        let poster = element
            .select(&REC_IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src").or_else(|| e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let status = element
            .select(&STATUS_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let r#type = element
            .select(&TYPE_SELECTOR)
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

    let duration = start_time.elapsed();
    info!(
        "Parsed anime detail document for slug: {} in {:?}",
        slug, duration
    );

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