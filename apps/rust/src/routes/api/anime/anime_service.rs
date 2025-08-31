use scraper::{Html, Selector, Element};
use std::error::Error;
use crate::routes::api::anime::anime_dto::AnimeData;
use crate::routes::api::anime::anime_detail_dto::{AnimeDetailResponseData, Genre, EpisodeListItem, Recommendation};
use crate::routes::api::komik::manga_dto::Pagination;
use rust_lib::fetch_with_proxy::fetch_with_proxy;

pub async fn fetch_anime_data(slug: &str) -> Result<String, Box<dyn Error>> {
    tracing::info!("[DEBUG] anime_service.rs using rust_lib::fetch_with_proxy import");
    let url = format!("https://otakudesu.cloud/?s={}&post_type=anime", slug);
    let response = fetch_with_proxy(&url).await?;
    tracing::debug!("FetchResult (anime_service.rs): {:?}", &response);
    Ok(response.data)
}

pub fn parse_anime_data(html: &str, slug: &str) -> (Vec<AnimeData>, Pagination) {
    let document = Html::parse_document(html);
    let anime_list_selector = Selector::parse("#venkonten .chivsrc li").unwrap();

    let mut anime_list: Vec<AnimeData> = Vec::new();

    for element in document.select(&anime_list_selector) {
        let title = element.select(&Selector::parse("h2 a").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let slug_val = element.select(&Selector::parse("h2 a").unwrap()).next().and_then(|e| e.value().attr("href").and_then(|s| s.split('/').nth(4).map(|s| s.to_string()))).unwrap_or_default();
        let poster = element.select(&Selector::parse("img").unwrap()).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();

        let episode_text = element.select(&Selector::parse("h2 a").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
        let episode_regex = regex::Regex::new(r"\(([^)]+)\)").unwrap();
        let episode = episode_regex.captures(&episode_text)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_else(|| "Ongoing".to_string());

        let anime_url = element.select(&Selector::parse("a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();

        let genres_selector = Selector::parse(".set b:contains(\"Genres\") + a").unwrap();
        let genres: Vec<String> = element.select(&genres_selector).map(|e| e.text().collect::<String>()).collect();

        let status_selector = Selector::parse(".set b:contains(\"Status\")").unwrap();
        let status = element.select(&status_selector)
            .next()
            .map(|e| e.parent_element().map(|p| p.text().collect::<String>().replace("Status :", "").trim().to_string()))
            .flatten()
            .unwrap_or_default();

        let rating_selector = Selector::parse(".set b:contains(\"Rating\")").unwrap();
        let rating = element.select(&rating_selector)
            .next()
            .map(|e| e.parent_element().map(|p| p.text().collect::<String>().replace("Rating :", "").trim().to_string()))
            .flatten()
            .unwrap_or_default();

        anime_list.push(AnimeData {
            title,
            slug: slug_val,
            poster,
            episode,
            anime_url,
            genres,
            status,
            rating,
        });
    }

    let page_num = slug.parse::<u32>().unwrap_or(1);
    tracing::debug!("current_page: {}, has_previous_page: {}", page_num, page_num > 1);
    let pagination = Pagination {
        current_page: page_num,
        last_visible_page: 57, // Hardcoded from Next.js example, needs dynamic scraping
        has_next_page: document.select(&Selector::parse(".hpage .r").unwrap()).next().is_some(),
        next_page: if document.select(&Selector::parse(".hpage .r").unwrap()).next().is_some() { Some(page_num + 1) } else { None },
        previous_page: if page_num > 1 { Some(page_num - 1) } else { None },
        has_previous_page: page_num > 1,
    };

    (anime_list, pagination)
}

pub async fn get_anime_episode_images(episode_url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let body = fetch_with_proxy(episode_url).await?;
    let body = body.to_string();
    let document = Html::parse_document(&body);

    let mut images: Vec<String> = Vec::new();
    let image_selector = Selector::parse(".vmirror img").unwrap(); // Assuming images are within .vmirror and are <img> tags
    for element in document.select(&image_selector) {
        if let Some(src) = element.value().attr("src") {
            images.push(src.to_string());
        }
    }

    Ok(serde_json::json!({ "images": images }))
}

pub async fn get_anime_detail(slug: &str) -> Result<AnimeDetailResponseData, Box<dyn Error>> {
    let url = format!("https://otakudesu.cloud/anime/{}", slug);
    let body = fetch_with_proxy(&url).await?;
    let body = body.to_string();
    let document = Html::parse_document(&body);

    let extract_text = |selector_str: &str, prefix: &str| -> String {
        let selector = Selector::parse(selector_str).unwrap();
        document.select(&selector).next().map(|e| e.text().collect::<String>().replace(prefix, "").trim().to_string()).unwrap_or_default()
    };

    let title = extract_text(".infozingle p:contains(\"Judul\")", "Judul: ");
    let alternative_title = extract_text(".infozingle p:contains(\"Japanese\")", "Japanese: ");
    let poster = document.select(&Selector::parse(".fotoanime img").unwrap()).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();
    let r#type = extract_text(".infozingle p:contains(\"Tipe\")", "Tipe: ");
    let release_date = extract_text(".infozingle p:contains(\"Tanggal Rilis\")", "Tanggal Rilis: ");
    let status = extract_text(".infozingle p:contains(\"Status\")", "Status: ");
    let synopsis = document.select(&Selector::parse(".sinopc").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let studio = extract_text(".infozingle p:contains(\"Studio\")", "Studio: ");

    let mut genres: Vec<Genre> = Vec::new();
    let genres_selector = Selector::parse(".infozingle p:contains(\"Genre\") a").unwrap();
    for element in document.select(&genres_selector) {
        let name = element.text().collect::<String>().trim().to_string();
        let href = element.value().attr("href").unwrap_or_default().to_string();
        let genre_slug = href.split('/').nth(4).unwrap_or_default().to_string();
        genres.push(Genre { name, slug: genre_slug, anime_url: href });
    }

    let producers_str = extract_text(".infozingle p:contains(\"Produser\")", "Produser: ");
    let producers: Vec<String> = producers_str.split(',').map(|s| s.trim().to_string()).collect();

    let mut episode_lists: Vec<EpisodeListItem> = Vec::new();
    let mut batch: Vec<EpisodeListItem> = Vec::new();
    let episode_selector = Selector::parse(".episodelist ul li span a").unwrap();
    for element in document.select(&episode_selector) {
        let episode = element.text().collect::<String>().trim().to_string();
        let href = element.value().attr("href").unwrap_or_default().to_string();
        let segments: Vec<&str> = href.split('/').collect();
        let episode_slug = segments.last().unwrap_or(&"").to_string();

        if episode.to_lowercase().contains("batch") {
            batch.push(EpisodeListItem { episode, slug: episode_slug });
        } else {
            episode_lists.push(EpisodeListItem { episode, slug: episode_slug });
        }
    }

    let mut recommendations: Vec<Recommendation> = Vec::new();
    let recommendation_selector = Selector::parse("#recommend-anime-series .isi-anime").unwrap();
    for element in document.select(&recommendation_selector) {
        let title = element.select(&Selector::parse(".judul-anime a").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let url = element.select(&Selector::parse("a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();
        let poster = element.select(&Selector::parse("img").unwrap()).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();
        let slug = url.split('/').nth(4).unwrap_or_default().to_string();
        recommendations.push(Recommendation { title, slug, poster, r#type: String::new() });
    }

    Ok(AnimeDetailResponseData {
        title,
        alternative_title,
        poster,
        r#type,
        release_date,
        status,
        synopsis,
        studio,
        genres,
        producers,
        recommendations,
        batch,
        episode_lists,
    })
}
