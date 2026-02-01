use crate::helpers::{internal_err, Cache, fetch_html_with_retry, parse_html};
use crate::services::images::cache::{get_cached_or_original, cache_image_urls_batch_lazy};
use crate::helpers::scraping::{selector, text_from_or, extract_slug, text, attr};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
use utoipa::ToSchema;


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
            let mut data = fetch_anime_detail(slug.clone())
                .await
                .map_err(|e| e.to_string())?;

            // 1. Cache posters
            data.poster = get_cached_or_original(
                app_state.db.clone(),
                &app_state.redis_pool,
                &data.poster,
                Some(app_state.image_processing_semaphore.clone()),
            ).await;
            
            data.poster2 = get_cached_or_original(
                app_state.db.clone(),
                &app_state.redis_pool,
                &data.poster2,
                Some(app_state.image_processing_semaphore.clone()),
            ).await;

            // 2. Batch cache for recommendations
            let rec_posters: Vec<String> = data.recommendations.iter().map(|r| r.poster.clone()).collect();
            let cached_rec_posters = cache_image_urls_batch_lazy(
                app_state.db.clone(),
                &app_state.redis_pool,
                rec_posters,
                Some(app_state.image_processing_semaphore.clone()),
            ).await;

            for (i, rec) in data.recommendations.iter_mut().enumerate() {
                if let Some(url) = cached_rec_posters.get(i) {
                    rec.poster = url.clone();
                }
            }

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
    let url = format!("https://alqanime.si/anime/{}/", slug);

    let html = fetch_html_with_retry(&url)
        .await
        .map_err(|e| format!("Failed to fetch HTML with retry: {}", e))?;
    let slug_clone = slug.clone();

    match tokio::task::spawn_blocking(move || {
        parse_anime_detail_document(&html, &slug_clone)
    })
    .await {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(format!("Failed to spawn blocking task: {}", join_err).into()),
    }
}

fn parse_anime_detail_document(
    html: &str,
    slug: &str,
) -> Result<AnimeDetailData, Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    info!("Starting to parse anime detail document for slug: {}", slug);

    let document = parse_html(html);

    let title_selector = selector(".entry-title").unwrap();
    let alt_title_selector = selector(".alter").unwrap();
    let poster_selector = selector(".thumb img, .thumbook img, .wp-post-image, .ts-post-image")
        .unwrap();
    let poster2_selector =
        selector(".bigcover img, .bixbox.animefull .bigcover .ime img").unwrap();
    let spe_span_selector = selector(".info-content .spe span").unwrap();
    let a_selector = selector("a").unwrap();
    let synopsis_selector = selector(".entry-content p").unwrap();
    let genre_selector = selector(".genxed a").unwrap();
    let download_container_selector = selector(".soraddl.dlone").unwrap();
    let resolution_selector = selector(".res").unwrap();
    let link_selector = selector(".slink a").unwrap();
    let h3_selector = selector("h3").unwrap();
    let recommendation_selector = selector(".listupd .bs").unwrap();
    let rec_title_selector = selector(".ntitle").unwrap();
    let rec_img_selector = selector("img").unwrap();
    let status_selector = selector(".status").unwrap();
    let type_selector = selector(".typez").unwrap();

    let title = text_from_or(&document.root_element(), &title_selector, "");

    let alternative_title = text_from_or(&document.root_element(), &alt_title_selector, "");

    let poster = document
        .select(&poster_selector)
        .next()
        .and_then(|e| {
            attr(&e, "src")
                .or_else(|| attr(&e, "data-src"))
                .or_else(|| attr(&e, "data-lazy-src"))
        })
        .unwrap_or_default();

    let poster2 = document
        .select(&poster2_selector)
        .next()
        .and_then(|e| {
            attr(&e, "src")
                .or_else(|| attr(&e, "data-src"))
                .or_else(|| attr(&e, "data-lazy-src"))
        })
        .unwrap_or_default();

    let r#type = document
        .select(&spe_span_selector)
        .find(|e| text(&e).contains("Tipe:"))
        .and_then(|span| span.select(&a_selector).next())
        .map(|e| text(&e))
        .unwrap_or_default();

    let release_date = document
        .select(&spe_span_selector)
        .find(|e| text(&e).contains("Dirilis:"))
        .map(|e| text(&e))
        .unwrap_or_default();

    let status = document
        .select(&spe_span_selector)
        .find(|e| text(&e).contains("Status:"))
        .map(|e| text(&e))
        .unwrap_or_default();

    let synopsis = text_from_or(&document.root_element(), &synopsis_selector, "");

    let studio = document
        .select(&spe_span_selector)
        .find(|e| text(&e).contains("Studio:"))
        .and_then(|span| span.select(&a_selector).next())
        .map(|e| text(&e))
        .unwrap_or_default();

    let mut genres = Vec::new();
    for element in document.select(&genre_selector) {
        let name = text(&element);
        let anime_url = attr(&element, "href").unwrap_or_default();
        let genre_slug = extract_slug(&anime_url);
        genres.push(Genre {
            name,
            slug: genre_slug,
            anime_url,
        });
    }

    let mut batch = Vec::new();
    let mut ova = Vec::new();
    let mut downloads = Vec::new();

    for element in document.select(&download_container_selector) {
        let resolution = text_from_or(&element, &resolution_selector, "");

        let mut links = Vec::new();
        for link_element in element.select(&link_selector) {
            let name = text(&link_element);
            let url = attr(&link_element, "href").unwrap_or_default();
            links.push(Link { name, url });
        }

        let download_item = DownloadItem { resolution, links };

        if let Some(h3) = element.select(&h3_selector).next() {
            let category = text(&h3).to_lowercase();
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
    for element in document.select(&recommendation_selector) {
        let title = text_from_or(&element, &rec_title_selector, "");

        let anime_url = element
            .select(&a_selector)
            .next()
            .and_then(|e| attr(&e, "href"))
            .unwrap_or_default();

        let rec_slug = extract_slug(&anime_url);

        let poster = element
            .select(&rec_img_selector)
            .next()
            .and_then(|e| attr(&e, "data-src").or_else(|| attr(&e, "src")))
            .unwrap_or_default();

        let status = text_from_or(&element, &status_selector, "");

        let r#type = text_from_or(&element, &type_selector, "");

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
        producers: vec![],
        recommendations,
        batch,
        ova,
        downloads,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}