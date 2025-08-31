// Annotated with utoipa for OpenAPI generation

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use scraper::{Html, Selector};
use std::sync::Arc;

use crate::routes::ChatState;
use crate::routes::api::komik::manga_dto::Pagination;
use rust_lib::fetch_with_proxy;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnimeCompleteData {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
}

async fn fetch_anime_page_complete(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    tracing::info!("[DEBUG] complete_anime.rs using rust_lib::fetch_with_proxy import");
    let url = format!("https://otakudesu.cloud/complete-anime/page/{}/", slug);
    let response = fetch_with_proxy(&url).await?;
    tracing::info!("[DEBUG] complete_anime.rs fetched body: {} bytes", response.len());
    Ok(response)
}

fn parse_anime_page_complete(html: &str, slug: &str) -> (Vec<AnimeCompleteData>, Pagination) {
    let body = html.to_string();
    let document = Html::parse_document(&body);

    // Declare selectors once for efficiency and readability
    let pagenavix_last_page_selector = Selector::parse(".pagenavix .page-numbers:not(.next):last").unwrap();
    let pagenavix_next_page_selector = Selector::parse(".pagenavix .next.page-numbers").unwrap();
    let venz_ul_li_selector = Selector::parse(".venz ul li").unwrap();
    let thumbz_h2_jdlflm_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epz_selector = Selector::parse(".epz").unwrap();

    let mut anime_list: Vec<AnimeCompleteData> = Vec::new();

    let pagination = {
        let current_page = slug.parse::<u32>().unwrap_or(1);
        let last_visible_page = document.select(&pagenavix_last_page_selector)
            .next()
            .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
            .unwrap_or(1);

        let has_next_page = document.select(&pagenavix_next_page_selector).next().is_some();
        let next_page = if has_next_page { Some(current_page + 1) } else { None };
        let has_previous_page = current_page > 1;
        let previous_page = if current_page > 1 { Some(current_page - 1) } else { None };

        Pagination {
            current_page,
            last_visible_page,
            has_next_page,
            next_page,
            has_previous_page,
            previous_page,
        }
    };

    for element in document.select(&venz_ul_li_selector) {
        let title = element.select(&thumbz_h2_jdlflm_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let anime_url = element.select(&a_selector)
            .next()
            .and_then(|e| e.value().attr("href").map(|s| s.to_string()))
            .unwrap_or_default();

        // Extract slug from anime_url after getting anime_url
        let slug_val = anime_url.split('/').nth(4).unwrap_or_default().to_string();

        let poster = element.select(&img_selector)
            .next()
            .and_then(|e| e.value().attr("src").map(|s| s.to_string()))
            .unwrap_or_default();

        let episode = element.select(&epz_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "N/A".to_string());

        anime_list.push(AnimeCompleteData {
            title,
            slug: slug_val,
            poster,
            episode,
            anime_url,
        });
    }

    (anime_list, pagination)
}

#[utoipa::path(
    get,
    path = "/api/anime/complete-anime/{slug}",
    params(
        ("slug" = String, Path, description = "Anime page slug")
    ),
    responses(
        (status = 200, description = "List of complete anime", body = [AnimeCompleteData]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Anime"
)]
pub async fn complete_anime_handler(
    Path(slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    match fetch_anime_page_complete(&slug).await {
        Ok(html) => {
            let (anime_list, pagination) = parse_anime_page_complete(&html, &slug);
            (
                StatusCode::OK,
                Json(json!({
                    "status": "Ok",
                    "data": anime_list,
                    "pagination": pagination,
                })),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("Complete anime error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to process request: {}", e) })),
            )
                .into_response()
        }
    }
}
