// Standard library imports
use std::sync::Arc;

// External crate imports
use crate::helpers::{
    internal_err, Cache, fetch_html_with_retry, text_from_or, attr_from_or, extract_slug,
    parse_html, selector
};
use crate::routes::AppState;
use crate::scraping::urls::OTAKUDESU_BASE_URL;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub score: String,
    pub anime_url: String,
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
pub struct OngoingAnimeResponse {
    pub status: String,
    pub data: Vec<OngoingAnimeItem>,
    pub pagination: Pagination,
}

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "URL-friendly identifier for the resource (typically lowercase with hyphens)", example = "1")
    ),
    path = "/api/anime/ongoing-anime/{slug}",
    tag = "anime",
    operation_id = "anime_ongoing_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/ongoing-anime/{slug} endpoint.", body = OngoingAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _start = std::time::Instant::now();
    info!("Starting request for ongoing_anime slug: {}", slug);

    let cache_key = format!("anime:ongoing:{}", slug);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (anime_list, pagination) = fetch_ongoing_anime_page(slug.clone())
                .await
                .map_err(|e| e.to_string())?;
            Ok(OngoingAnimeResponse {
                status: "Ok".to_string(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    return Ok(Json(response).into_response());
}

async fn fetch_ongoing_anime_page(
    slug: String,
) -> Result<(Vec<OngoingAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/ongoing-anime/page/{}/", OTAKUDESU_BASE_URL, slug);

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;
    let slug_clone = slug.clone();

    match tokio::task::spawn_blocking(move || {
        parse_ongoing_anime_document(&html, &slug_clone)
    })
    .await
    {
        Ok(inner_result) => inner_result,
        Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
    }
}

fn parse_ongoing_anime_document(
    html: &str,
    slug: &str,
) -> Result<(Vec<OngoingAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    info!(
        "Starting to parse ongoing anime document for slug: {}",
        slug
    );

    let document = parse_html(html);
    let mut anime_list = Vec::new();

    let venz_selector = selector(".venz ul li").unwrap();
    let title_selector = selector(".thumbz h2.jdlflm").unwrap();
    let img_selector = selector("img").unwrap();
    let ep_selector = selector(".epz").unwrap();
    let link_selector = selector("a").unwrap();
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagination .next").unwrap();
    
    for element in document.select(&venz_selector) {
        let title = text_from_or(&element, &title_selector, "");
        let poster = attr_from_or(&element, &img_selector, "src", "");
        let score = text_from_or(&element, &ep_selector, "N/A");
        let anime_url = attr_from_or(&element, &link_selector, "href", "");
        
        // Extract slug from the anime URL, not the current page slug
        let item_slug = extract_slug(&anime_url);

        if !title.is_empty() {
            anime_list.push(OngoingAnimeItem {
                title,
                slug: item_slug,
                poster,
                score,
                anime_url,
            });
        }
    }

    let current_page = slug.parse::<u32>().unwrap_or(1);

    let last_visible_page = document
        .select(&pagination_selector)
        .next_back()
        .map(|e| {
            e.text()
                .collect::<String>()
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        })
        .unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();

    let next_page = if has_next_page {
        Some(current_page + 1)
    } else {
        None
    };

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some(current_page - 1)
    } else {
        None
    };

    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    };

    let duration = start_time.elapsed();
    info!(
        "Parsed {} ongoing anime items in {:?}",
        anime_list.len(),
        duration
    );

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}