// Standard library imports
use std::sync::Arc;

// External crate imports
use crate::helpers::{default_backoff, internal_err, parse_html, transient, Cache};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use backoff::future::retry;

use lazy_static::lazy_static;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use utoipa::ToSchema;

// Internal imports
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::OTAKUDESU_BASE_URL;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/detail/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/detail/{slug} endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_detail_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EpisodeList {
    pub episode: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Recommendation {
    pub title: String,
    pub slug: String,
    pub poster: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeDetailData {
    pub title: String,
    pub alternative_title: String,
    pub poster: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub release_date: String,
    pub studio: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub genres: Vec<Genre>,
    pub synopsis: String,
    pub episode_lists: Vec<EpisodeList>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub batch: Vec<EpisodeList>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub producers: Vec<String>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub data: AnimeDetailData,
}

lazy_static! {
    pub static ref INFO_SELECTOR: Selector = Selector::parse(".infozingle p").unwrap();
    pub static ref POSTER_SELECTOR: Selector = Selector::parse(".fotoanime img").unwrap();
    pub static ref SYNOPSIS_SELECTOR: Selector = Selector::parse(".sinopc").unwrap();
    pub static ref GENRE_LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    pub static ref EPISODE_LIST_SELECTOR: Selector =
        Selector::parse(".episodelist ul li a").unwrap();
    pub static ref RECOMMENDATION_SELECTOR: Selector =
        Selector::parse("#recommend-anime-series .isi-anime").unwrap();
    pub static ref RECOMMENDATION_TITLE_SELECTOR: Selector =
        Selector::parse(".judul-anime a").unwrap();
    pub static ref RECOMMENDATION_IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
}
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "naruto-shippuden-episode-1")
    ),
    path = "/api/anime/detail/{slug}",
    tag = "anime",
    operation_id = "anime_detail_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/detail/{slug} endpoint.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _start = std::time::Instant::now();
    info!("Starting request for detail slug: {}", slug);

    let cache_key = format!("anime:detail:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let data = fetch_anime_detail(slug.clone())
                .await
                .map_err(|e| e.to_string())?;
            Ok(DetailResponse {
                status: Some("Ok".to_string()),
                data,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    return Ok(Json(response).into_response());
}

async fn fetch_anime_detail(
    slug: String,
) -> Result<AnimeDetailData, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/anime/{}", OTAKUDESU_BASE_URL, slug);

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

    let html = retry(backoff, fetch_operation)
        .await
        .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;

    match tokio::task::spawn_blocking(move || parse_anime_detail_document(&parse_html(&html))).await
    {
        Ok(inner_result) => inner_result.map_err(|e| e.into()),
        Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
    }
}

fn parse_anime_detail_document(document: &Html) -> Result<AnimeDetailData, AppError> {
    let info_selector = Selector::parse(".infozingle p").unwrap();
    let poster_selector = Selector::parse(".fotoanime img").unwrap();
    let synopsis_selector = Selector::parse(".sinopc").unwrap();
    let genre_link_selector = Selector::parse("a").unwrap();
    let episode_list_selector = Selector::parse(".episodelist ul li a").unwrap();
    let recommendation_selector = Selector::parse("#recommend-anime-series .isi-anime").unwrap();
    let recommendation_title_selector = Selector::parse(".judul-anime a").unwrap();
    let recommendation_img_selector = Selector::parse("img").unwrap();

    let mut title = String::new();
    let mut alternative_title = String::new();
    let mut r#type: Option<String> = None;
    let mut status: Option<String> = None;
    let mut release_date = String::new();
    let mut studio = String::new();
    let producers = Vec::new(); // Not present in the original HTML, keeping empty

    for element in document.select(&info_selector) {
        let text = element.text().collect::<String>();
        if text.contains("Judul:") {
            title = text.replace("Judul:", "").trim().to_string();
        } else if text.contains("Japanese:") {
            alternative_title = text.replace("Japanese:", "").trim().to_string();
        } else if text.contains("Type:") {
            let type_str = text.replace("Type:", "").trim().to_string();
            if !type_str.is_empty() {
                r#type = Some(type_str);
            }
        } else if text.contains("Status:") {
            let status_str = text.replace("Status:", "").trim().to_string();
            if !status_str.is_empty() {
                status = Some(status_str);
            }
        } else if text.contains("Tanggal Rilis:") {
            release_date = text.replace("Tanggal Rilis:", "").trim().to_string();
        } else if text.contains("Studio:") {
            studio = text.replace("Studio:", "").trim().to_string();
        }
    }

    let poster = document
        .select(&poster_selector)
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or("")
        .to_string();

    let synopsis = document
        .select(&synopsis_selector)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let mut genres = Vec::new();
    if let Some(genres_element) = document
        .select(&info_selector)
        .find(|e| e.text().collect::<String>().contains("Genres:"))
    {
        for genre_link in genres_element.select(&genre_link_selector) {
            let name = genre_link.text().collect::<String>().trim().to_string();
            let anime_url = genre_link.value().attr("href").unwrap_or("").to_string();
            let genre_slug = anime_url.split('/').nth(4).unwrap_or("").to_string(); // Adjust as needed
            genres.push(Genre {
                name,
                slug: genre_slug,
                anime_url,
            });
        }
    }

    let mut episode_lists = Vec::new();
    for element in document.select(&episode_list_selector) {
        let episode = element.text().collect::<String>().trim().to_string();
        let slug = element
            .value()
            .attr("href")
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();
        episode_lists.push(EpisodeList { episode, slug });
    }

    // Batch and producers are not directly parsable from the provided HTML structure
    // Keeping them empty as per previous implementation for anime/full/slug.rs

    let mut recommendations = Vec::new();
    for element in document.select(&recommendation_selector) {
        let title = element
            .select(&recommendation_title_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let poster = element
            .select(&recommendation_img_selector)
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();
        let slug = element
            .select(&genre_link_selector) // Reusing genre_link_selector for general links
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();
        recommendations.push(Recommendation {
            title,
            slug,
            poster,
            status: None,
            r#type: None,
        }); // Status and type not available from this selector
    }

    Ok(AnimeDetailData {
        title,
        alternative_title,
        poster,
        r#type,
        status,
        release_date,
        studio,
        genres,
        synopsis,
        episode_lists,
        batch: vec![],
        producers,
        recommendations,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}