use crate::helpers::{
    internal_err, Cache, fetch_html_with_retry, text_from_or, attr_from_or, extract_slug,
    parse_html, selector
};
use crate::routes::AppState;
use crate::scraping::urls::get_otakudesu_url;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub score: String,
    pub status: String,
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
pub struct GenreAnimeResponse {
    pub status: String,
    pub genre: String,
    pub data: Vec<AnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct GenreQuery {
    pub page: Option<u32>,
}

const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    params(
        ("genre_slug" = String, Path, description = "Parameter for resource identification", example = "sample_value"),
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/anime/genre/{slug}",
    tag = "anime",
    operation_id = "anime_genre_filter",
    responses(
        (status = 200, description = "Filter anime by genre with pagination", body = GenreAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(genre_slug): Path<String>,
    Query(params): Query<GenreQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    info!("Handling request for genre: {}, page: {}", genre_slug, page);

    let cache_key = format!("anime:genre:{}:{}", genre_slug, page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (anime_list, pagination) = fetch_genre_anime(&genre_slug, page)
                .await
                .map_err(|e| e.to_string())?;

            Ok(GenreAnimeResponse {
                status: "Ok".to_string(),
                genre: genre_slug.clone(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_genre_anime(
    genre_slug: &str,
    page: u32,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = if page == 1 {
        format!("{}/genres/{}/", get_otakudesu_url(), genre_slug)
    } else {
        format!(
            "{}/genres/{}/page/{}/",
            get_otakudesu_url(),
            genre_slug,
            page
        )
    };

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<AnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
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
        let slug = extract_slug(&anime_url);

        // Try to determine status from score text
        let status_text = score.to_lowercase();
        let status = if status_text.contains("ongoing") {
            "Ongoing".to_string()
        } else if status_text.contains("completed") || status_text.contains("eps") {
            "Completed".to_string()
        } else {
            "Unknown".to_string()
        };

        if !title.is_empty() {
            anime_list.push(AnimeItem {
                title,
                slug,
                poster,
                score,
                status,
                anime_url,
            });
        }
    }

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

    info!("Parsed {} anime items", anime_list.len());
    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}