use scraper::{Html, Selector};
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::routes::api::komik::manga_dto::Pagination;
use rust_lib::fetch_with_proxy::fetch_with_proxy;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
}

pub async fn fetch_anime_page_ongoing(slug: &str) -> Result<String, Box<dyn Error>> {
    tracing::info!("[DEBUG] otakudesu_service.rs using rust_lib::fetch_with_proxy import");
    let url = format!("https://otakudesu.cloud/ongoing-anime/page/{}/", slug);
    let response = fetch_with_proxy(&url).await?;
    tracing::info!("[DEBUG] otakudesu_service.rs fetched body: {} bytes", response.data.len());
    tracing::debug!("FetchResult (otakudesu_service.rs): {:?}", &response);
    Ok(response.data)
}

pub fn parse_anime_page_ongoing(html: &str, slug: &str) -> (Vec<AnimeItem>, Pagination) {
    let body = html.to_string();
    let document = Html::parse_document(&body);

    let mut anime_list: Vec<AnimeItem> = Vec::new();
    let anime_selector = Selector::parse(".venz ul li").unwrap();
    let title_selector = Selector::parse(".thumbz h2.jdlflm").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let epz_selector = Selector::parse(".epz").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    for element in document.select(&anime_selector) {
        let title = element.select(&title_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let anime_url = element.select(&a_selector).next().and_then(|e| e.value().attr("href")).unwrap_or_default().to_string();
        let item_slug = anime_url.split('/').nth(4).unwrap_or_default().to_string();
        let poster = element.select(&img_selector).next().and_then(|e| e.value().attr("src")).unwrap_or_default().to_string();
        let episode = element.select(&epz_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or("Ongoing".to_string());

        anime_list.push(AnimeItem {
            title,
            slug: item_slug,
            poster,
            episode,
            anime_url,
        });
    }

    let pagination_selector = Selector::parse(".pagination .page-numbers:not(.next):last").unwrap();
    let next_selector = Selector::parse(".pagination .next").unwrap();

    let current_page_int = slug.parse::<u32>().unwrap_or(1);
    let last_visible_page_text = document.select(&pagination_selector).next().map(|e| e.text().collect::<String>()).unwrap_or("1".to_string());
    let last_visible_page_int = last_visible_page_text.parse::<u32>().unwrap_or(1);

    let has_next_page = document.select(&next_selector).next().is_some();
    let next_page = if has_next_page { Some(current_page_int + 1) } else { None };
    let previous_page = if current_page_int > 1 { Some(current_page_int - 1) } else { None };

    let pagination = Pagination {
        current_page: current_page_int,
        last_visible_page: last_visible_page_int,
        has_next_page,
        next_page,
        previous_page,
        has_previous_page: current_page_int > 1,
    };

    (anime_list, pagination)
}
