// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime2/complete-anime/{slug}";
const ENDPOINT_DESCRIPTION: &str = "Fetches a paginated list of completed anime from alqanime.net";
const ENDPOINT_TAG: &str = "anime2";
const SUCCESS_RESPONSE_BODY: &str = "CompleteAnimeResponse";
const SLUG_DESCRIPTION: &str = "Slug for pagination (page number).";
// --- AKHIR METADATA ---

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use reqwest::Client;
use scraper::{Html, Selector};
use axum::http::StatusCode;

#[derive(Serialize, ToSchema)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
}

#[derive(Serialize, ToSchema)]
pub struct Pagination {
    pub current_page: usize,
    pub last_visible_page: usize,
    pub has_next_page: bool,
    pub next_page: Option<usize>,
    pub has_previous_page: bool,
    pub previous_page: Option<usize>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompleteAnimeResponse {
    pub status: &'static str,
    pub data: Vec<AnimeItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub error: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

pub async fn complete_anime_handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!(
        "https://alqanime.net/advanced-search/page/{}/?status=completed&order=update",
        slug
    );

    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return ErrorResponse {
                    message: "Failed to fetch data".to_string(),
                    error: e.to_string(),
                }.into_response();
            }
        },
        Err(e) => {
            return ErrorResponse {
                message: "Failed to fetch data".to_string(),
                error: e.to_string(),
            }.into_response();
        }
    };

    let (anime_list, pagination) = parse_anime_page(&html, &slug);

    let response = CompleteAnimeResponse {
        status: "Ok",
        data: anime_list,
        pagination,
    };

    Json(response).into_response()
}

fn parse_anime_page(html: &str, slug: &str) -> (Vec<AnimeItem>, Pagination) {
    let document = Html::parse_document(html);
    let bs_selector = Selector::parse(".listupd .bs").unwrap();
    let ntitle_selector = Selector::parse(".ntitle").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epx_selector = Selector::parse(".epx").unwrap();

    let mut anime_list = Vec::new();

    for bs in document.select(&bs_selector) {
        let title = bs
            .select(&ntitle_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let anime_url = bs
            .select(&a_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("")
            .to_string();
        let slug_val = anime_url
            .split('/')
            .nth(3)
            .unwrap_or("")
            .to_string();
        let poster = bs
            .select(&img_selector)
            .next()
            .and_then(|img| img.value().attr("data-src"))
            .unwrap_or("")
            .to_string();
        let episode = bs
            .select(&epx_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        anime_list.push(AnimeItem {
            title,
            slug: slug_val,
            poster,
            episode,
            anime_url,
        });
    }

    // Pagination parsing
    let pagination_selector = Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = Selector::parse(".pagination .next.page-numbers").unwrap();

    let current_page = slug.parse::<usize>().unwrap_or(1);
    let last_visible_page = document
        .select(&pagination_selector)
        .last()
        .and_then(|n| {
            let text = n.text().collect::<String>();
            text.parse::<usize>().ok()
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

    (anime_list, pagination)
}
