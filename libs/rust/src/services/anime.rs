use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use crate::models::anime::{AnimeData, AnimeDetail, AnimeEpisode, AnimeEpisodeImage};
use crate::models::manga::Pagination;

// Placeholder for fetchWithProxy
async fn fetch_with_proxy(url: &str) -> Result<String, Box<dyn Error>> {
    // In a real scenario, this would involve proxy logic.
    // For now, a direct fetch.
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

pub async fn fetch_anime_data(slug: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://otakudesu.cloud/?s={}&post_type=anime", slug);
    let response = fetch_with_proxy(&url).await?;
    Ok(response)
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
        let episode = episode_text.find('(')
            .and_then(|start| episode_text[start..].find(')').map(|end| &episode_text[start + 1..start + end]))
            .unwrap_or("Ongoing")
            .to_string();

        let anime_url = element.select(&Selector::parse("h2 a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();

        let genres_selector = Selector::parse(".set b:contains(\"Genres\") + a").unwrap();
        let genres: Vec<String> = element.select(&genres_selector).map(|e| e.text().collect::<String>()).collect();

        let status_selector = Selector::parse(".set b:contains(\"Status\")").unwrap();
        let status = element.select(&status_selector).next().map(|e| e.text().collect::<String>().strip_prefix("Status :").map(|s| s.trim().to_string())).flatten().unwrap_or_default();

        let rating_selector = Selector::parse(".set b:contains(\"Rating\")").unwrap();
        let rating = element.select(&rating_selector).next().map(|e| e.text().collect::<String>().strip_prefix("Rating :").map(|s| s.trim().to_string())).flatten().unwrap_or_default();

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
    let pagination = Pagination {
        current_page: page_num,
        last_visible_page: 57, // Hardcoded from Next.js example, needs dynamic scraping
        has_next_page: document.select(&Selector::parse(".hpage .r").unwrap()).next().is_some(),
        next_page: if document.select(&Selector::parse(".hpage .r").unwrap()).next().is_some() { Some(page_num + 1) } else { None },
        previous_page: if page_num > 1 { Some(page_num - 1) } else { None },
    };

    (anime_list, pagination)
}

pub async fn get_anime_detail(slug: &str) -> Result<AnimeDetail, Box<dyn Error>> {
    let url = format!("https://otakudesu.cloud/anime/{}", slug);
    let body = fetch_with_proxy(&url).await?;
    let document = Html::parse_document(&body);

    let title = document.select(&Selector::parse(".jdlbar").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let poster = document.select(&Selector::parse(".fotoanime img").unwrap()).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();

    let mut genres: Vec<String> = Vec::new();
    let genres_selector = Selector::parse(".infozingle p:contains(\"Genres\") a").unwrap();
    for element in document.select(&genres_selector) {
        genres.push(element.text().collect::<String>().trim().to_string());
    }

    let status = document.select(&Selector::parse(".infozingle p:contains(\"Status\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Status :", "").trim().to_string()).unwrap_or_default();
    let rating = document.select(&Selector::parse(".infozingle p:contains(\"Rating\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Rating :", "").trim().to_string()).unwrap_or_default();
    let producer = document.select(&Selector::parse(".infozingle p:contains(\"Producer\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Producer :", "").trim().to_string()).unwrap_or_default();
    let type_anime = document.select(&Selector::parse(".infozingle p:contains(\"Type\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Type :", "").trim().to_string()).unwrap_or_default();
    let total_episode = document.select(&Selector::parse(".infozingle p:contains(\"Total Episode\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Total Episode :", "").trim().to_string()).unwrap_or_default();
    let duration = document.select(&Selector::parse(".infozingle p:contains(\"Duration\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Duration :", "").trim().to_string()).unwrap_or_default();
    let release_date = document.select(&Selector::parse(".infozingle p:contains(\"Release Date\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Release Date :", "").trim().to_string()).unwrap_or_default();
    let studio = document.select(&Selector::parse(".infozingle p:contains(\"Studio\")").unwrap()).next().map(|e| e.text().collect::<String>().replace("Studio :", "").trim().to_string()).unwrap_or_default();
    let synopsis = document.select(&Selector::parse(".sinopc").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();

    let mut episodes: Vec<AnimeEpisode> = Vec::new();
    let episode_selector = Selector::parse("#episode_list li").unwrap();
    for element in document.select(&episode_selector) {
        let episode_title = element.select(&Selector::parse("a").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let episode_url = element.select(&Selector::parse("a").unwrap()).next().and_then(|e| e.value().attr("href").map(|s| s.to_string())).unwrap_or_default();
        let uploaded_on = element.select(&Selector::parse(".tgl").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        episodes.push(AnimeEpisode { episode_title, episode_url, uploaded_on });
    }

    Ok(AnimeDetail {
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

pub async fn get_anime_episode_images(episode_url: &str) -> Result<Vec<AnimeEpisodeImage>, Box<dyn Error>> {
    let body = fetch_with_proxy(episode_url).await?;
    let document = Html::parse_document(&body);

    let mut images: Vec<AnimeEpisodeImage> = Vec::new();
    let image_selector = Selector::parse("#lightgallery img").unwrap();
    for element in document.select(&image_selector) {
        images.push(AnimeEpisodeImage { image_url: element.value().attr("src").map(|s| s.to_string()).unwrap_or_default() });
    }

    Ok(images)
}
