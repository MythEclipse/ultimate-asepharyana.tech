// Handler for GET /api/komik/manhua
// Fetches manhua list, parses HTML, and returns JSON response.
// Uses reqwest for HTTP and scraper for HTML parsing.

use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};

#[derive(Serialize, utoipa::ToSchema)]
struct ManhuaData {
    title: String,
    poster: String,
    chapter: String,
    date: String,
    score: String,
    r#type: String,
    slug: String,
}

#[derive(Serialize, utoipa::ToSchema)]
struct Pagination {
    current_page: u32,
    last_visible_page: u32,
    has_next_page: bool,
    next_page: Option<u32>,
    has_previous_page: bool,
    previous_page: Option<u32>,
}

#[derive(Serialize, utoipa::ToSchema)]
struct ManhuaListResponse {
    data: Vec<ManhuaData>,
    pagination: Pagination,
}

#[derive(Deserialize)]
pub struct Params {
    page: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/api/komik/manhua",
    params(
        ("page" = Option<u32>, Query, description = "Page number")
    ),
    responses(
        (status = 200, description = "Manhua list response", body = ManhuaListResponse)
    )
)]
pub async fn handler(Query(params): Query<Params>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let url = format!("https://komikcast.site/manhua/page/{}/", page);

    let html = match reqwest::get(&url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(_) => return axum::http::StatusCode::BAD_GATEWAY.into_response(),
        },
        Err(_) => return axum::http::StatusCode::BAD_GATEWAY.into_response(),
    };

    let document = Html::parse_document(&html);
    let item_selector = Selector::parse(".animposx").unwrap();
    let title_selector = Selector::parse(".tt h4").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let chapter_selector = Selector::parse(".lsch a").unwrap();
    let score_selector = Selector::parse("i").unwrap();
    let date_selector = Selector::parse(".datech").unwrap();
    let typeflag_selector = Selector::parse(".typeflag").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut data = Vec::new();

    for item in document.select(&item_selector) {
        let title = item.select(&title_selector).next().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let poster = item.select(&img_selector).next().and_then(|n| n.value().attr("src")).map(|s| s.split('?').next().unwrap_or("").to_string()).unwrap_or_default();
        let chapter = item.select(&chapter_selector).next().map(|n| {
            let txt = n.text().collect::<String>().replace("Ch.", "");
            txt.trim().split_whitespace().next().unwrap_or("").to_string()
        }).unwrap_or_default();
        let score = item.select(&score_selector).next().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let date = item.select(&date_selector).next().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let r#type = item.select(&typeflag_selector).next().and_then(|n| n.value().attr("class")).map(|c| c.split_whitespace().nth(1).unwrap_or("").to_string()).unwrap_or_default();
        let slug = item.select(&a_selector).next().and_then(|n| n.value().attr("href")).map(|href| href.split('/').nth(4).unwrap_or("").to_string()).unwrap_or_default();

        data.push(ManhuaData {
            title,
            poster,
            chapter,
            date,
            score,
            r#type,
            slug,
        });
    }

    // Pagination parsing
    let pagination_selector = Selector::parse(".pagination").unwrap();
    let current_selector = Selector::parse(".current").unwrap();
    let next_selector = Selector::parse(".next").unwrap();
    let prev_selector = Selector::parse(".prev").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let pagination = if let Some(pagination_node) = document.select(&pagination_selector).next() {
        let current_page = pagination_node.select(&current_selector).next().and_then(|n| n.text().collect::<String>().trim().parse().ok()).unwrap_or(1);
        let last_visible_page = pagination_node.select(&a_selector).filter(|n| !n.value().classes().any(|c| c == "next")).last().and_then(|n| n.text().collect::<String>().trim().parse().ok()).unwrap_or(current_page);
        let has_next_page = pagination_node.select(&next_selector).next().is_some();
        let has_previous_page = pagination_node.select(&prev_selector).next().is_some();
        let next_page = if has_next_page && current_page < last_visible_page { Some(current_page + 1) } else { None };
        let previous_page = if has_previous_page && current_page > 1 { Some(current_page - 1) } else { None };
        Pagination {
            current_page,
            last_visible_page,
            has_next_page,
            next_page,
            has_previous_page,
            previous_page,
        }
    } else {
        Pagination {
            current_page: 1,
            last_visible_page: 1,
            has_next_page: false,
            next_page: None,
            has_previous_page: false,
            previous_page: None,
        }
    };

    Json(ManhuaListResponse { data, pagination }).into_response()
}
