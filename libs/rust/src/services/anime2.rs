use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use crate::models::anime2::{Anime2Data, Anime2Detail, Anime2Episode};
use crate::models::manga::Pagination;

// Placeholder for fetchWithProxy
async fn fetch_with_proxy(url: &str) -> Result<String, Box<dyn Error>> {
    // In a real scenario, this would involve proxy logic.
    // For now, a direct fetch.
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

pub async fn fetch_anime2_data(slug: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://alqanime.net/?s={}", slug);
    let response = fetch_with_proxy(&url).await?;
    Ok(response)
}

pub fn parse_anime2_data(html: &str) -> (Vec<Anime2Data>, Pagination) {
    let document = Html::parse_document(html);
    let anime_list_selector = Selector::parse(".listupd article.bs").unwrap();

    let mut anime_list: Vec<Anime2Data> = Vec::new();

    for element in document.select(&anime_list_selector) {
        let title = element.select(&Selector::parse(".ntitle").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let slug = element.select(&Selector::parse("a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();
        let poster = element.select(&Selector::parse("img").unwrap()).next().and_then(|e| e.value().attr("data-src").map(|s| s.to_string())).unwrap_or_default();
        let description = element.select(&Selector::parse("h2").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let anime_url = element.select(&Selector::parse("a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();
        let genres: Vec<String> = Vec::new(); // Not available in the provided HTML
        let rating = element.select(&Selector::parse(".numscore").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let anime_type = element.select(&Selector::parse(".typez").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let season = "".to_string(); // Not available in the provided HTML

        anime_list.push(Anime2Data {
            title,
            slug,
            poster,
            description,
            anime_url,
            genres,
            rating,
            anime_type,
            season,
        });
    }

    let current_page = document.select(&Selector::parse(".pagination .current").unwrap())
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(1);

    let last_visible_page = document.select(&Selector::parse(".pagination .page-numbers").unwrap())
        .last()
        .and_then(|e| e.prev_sibling().and_then(|s| scraper::ElementRef::wrap(s).and_then(|el| el.text().collect::<String>().trim().parse::<u32>().ok())))
        .unwrap_or(1);

    let has_next_page = document.select(&Selector::parse(".pagination .next").unwrap()).next().is_some();
    let next_page = if has_next_page {
        document.select(&Selector::parse(".pagination .next").unwrap())
            .next()
            .and_then(|e| e.value().attr("href").and_then(|s| s.split('/').last().and_then(|p| p.parse::<u32>().ok())))
    } else {
        None
    };

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        document.select(&Selector::parse(".pagination .prev").unwrap())
            .next()
            .and_then(|e| e.value().attr("href").and_then(|s| s.split('/').last().and_then(|p| p.parse::<u32>().ok())))
    } else {
        None
    };

    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        previous_page,
    };

    (anime_list, pagination)
}

pub async fn get_anime2_detail(slug: &str) -> Result<Anime2Detail, Box<dyn Error>> {
    let url = format!("https://alqanime.net/anime/{}", slug);
    let body = fetch_with_proxy(&url).await?;
    let document = Html::parse_document(&body);

    let title = document.select(&Selector::parse(".entry-title").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let poster = document.select(&Selector::parse(".thumb img").unwrap()).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();

    let mut genres: Vec<String> = Vec::new();
    let genres_selector = Selector::parse(".info-content .genxed a").unwrap();
    for element in document.select(&genres_selector) {
        genres.push(element.text().collect::<String>().trim().to_string());
    }

    let status = document.select(&Selector::parse(".info-content .spe span:contains(\"Status\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Status:", "").trim().to_string()).unwrap_or_default();
    let rating = document.select(&Selector::parse(".info-content .spe span:contains(\"Rating\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Rating:", "").trim().to_string()).unwrap_or_default();
    let producer = document.select(&Selector::parse(".info-content .spe span:contains(\"Producer\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Producer:", "").trim().to_string()).unwrap_or_default();
    let type_anime = document.select(&Selector::parse(".info-content .spe span:contains(\"Type\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Type:", "").trim().to_string()).unwrap_or_default();
    let total_episode = document.select(&Selector::parse(".info-content .spe span:contains(\"Total Episode\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Total Episode:", "").trim().to_string()).unwrap_or_default();
    let duration = document.select(&Selector::parse(".info-content .spe span:contains(\"Duration\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Duration:", "").trim().to_string()).unwrap_or_default();
    let release_date = document.select(&Selector::parse(".info-content .spe span:contains(\"Released\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Released:", "").trim().to_string()).unwrap_or_default();
    let studio = document.select(&Selector::parse(".info-content .spe span:contains(\"Studio\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Studio:", "").trim().to_string()).unwrap_or_default();
    let synopsis = document.select(&Selector::parse(".entry-content.entry-content-single p").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();

    let mut episodes: Vec<Anime2Episode> = Vec::new();
    let episode_selector = Selector::parse("#chapter_list li").unwrap();
    for element in document.select(&episode_selector) {
        let episode_title = element.select(&Selector::parse("a").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let episode_url = element.select(&Selector::parse("a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();
        let uploaded_on = element.select(&Selector::parse(".date").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        episodes.push(Anime2Episode { episode_title, episode_url, uploaded_on });
    }

    Ok(Anime2Detail {
        title,
        poster,
        genres,
        status,
        rating,
        producer,
        type_anime,
        total_episode,
        duration,
        release_date,
        studio,
        synopsis,
        episodes,
    })
}

pub async fn get_anime2_episode_images(episode_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let body = fetch_with_proxy(episode_url).await?;
    let document = Html::parse_document(&body);

    let mut images: Vec<String> = Vec::new();
    let image_selector = Selector::parse("#readerarea img").unwrap();
    for element in document.select(&image_selector) {
        images.push(element.value().attr("src").map(|s| s.to_string()).unwrap_or_default());
    }

    Ok(images)
}
