// Handler for GET /api/anime/complete-anime/{slug}.
// Fetches and parses the paginated complete anime list from otakudesu.cloud using reqwest and scraper.

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AnimeItem {
    title: String,
    slug: String,
    poster: String,
    episode: String,
    anime_url: String,
}

#[derive(Serialize)]
pub struct Pagination {
    current_page: usize,
    last_visible_page: usize,
    has_next_page: bool,
    next_page: Option<usize>,
    has_previous_page: bool,
    previous_page: Option<usize>,
}

#[derive(Serialize)]
pub struct AnimeListResponse {
    status: &'static str,
    data: Vec<AnimeItem>,
    pagination: Pagination,
}

pub async fn complete_anime_handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!("https://otakudesu.cloud/complete-anime/page/{}/", slug);

    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(HashMap::from([
                        ("message", "Failed to fetch data"),
                        ("error", &e.to_string()),
                    ])),
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(HashMap::from([
                    ("message", "Failed to fetch data"),
                    ("error", &e.to_string()),
                ])),
            )
                .into_response();
        }
    };

    let (anime_list, pagination) = parse_anime_page(&html, &slug);

    let response = AnimeListResponse {
        status: "Ok",
        data: anime_list,
        pagination,
    };

    Json(response).into_response()
}

fn parse_anime_page(html: &str, slug: &str) -> (Vec<AnimeItem>, Pagination) {
    let document = Html::parse_document(html);
    let li_selector = Selector::parse(".venz ul li").unwrap();
    let title_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epz_selector = Selector::parse(".epz").unwrap();

    let mut anime_list = Vec::new();

    for li in document.select(&li_selector) {
        let title = li
            .select(&title_selector)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        let anime_url = li
            .select(&a_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("")
            .to_string();
        let slug_val = anime_url.split('/').nth(4).unwrap_or("").to_string();
        let poster = li
            .select(&img_selector)
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("")
            .to_string();
        let episode = li
            .select(&epz_selector)
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
    let page_num = slug.parse::<usize>().unwrap_or(1);
    let pagenav_selector = Selector::parse(".pagenavix .page-numbers:not(.next)").unwrap();
    let next_selector = Selector::parse(".pagenavix .next.page-numbers").unwrap();

    let last_visible_page = document
        .select(&pagenav_selector)
        .last()
        .and_then(|n| {
            let text = n.text().collect::<String>();
            text.trim().parse::<usize>().ok()
        })
        .unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();
    let next_page = if has_next_page { Some(page_num + 1) } else { None };
    let has_previous_page = page_num > 1;
    let previous_page = if has_previous_page { Some(page_num - 1) } else { None };

    let pagination = Pagination {
        current_page: page_num,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    };

    (anime_list, pagination)
}
