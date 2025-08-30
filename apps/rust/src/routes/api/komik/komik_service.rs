use reqwest::Client;
use scraper::{Html, Selector};
use crate::routes::api::komik::manga_dto::{MangaData, MangaDetail, MangaChapter, ChapterData, Pagination};
use std::error::Error;

// Utility function to parse manga data
pub fn parse_manga_data(body: &str) -> Vec<MangaData> {
    let document = Html::parse_document(body);
    let selector = Selector::parse(".animposx").unwrap();
    let title_selector = Selector::parse(".tt h4").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let chapter_selector = Selector::parse(".lsch a").unwrap();
    let score_selector = Selector::parse("i").unwrap();
    let date_selector = Selector::parse(".datech").unwrap();
    let type_selector = Selector::parse(".typeflag").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    let mut data: Vec<MangaData> = Vec::new();

    for element in document.select(&selector) {
        let title = element.select(&title_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let mut poster = element.select(&img_selector).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();
        poster = poster.split('?').next().unwrap_or(&poster).to_string(); // Remove query parameters
        let chapter = element.select(&chapter_selector).next().map(|e| {
            e.text().collect::<String>().trim().replace("Ch.", "").chars().filter(|c| c.is_ascii_digit() || *c == '.').collect::<String>()
        }).unwrap_or_default();
        let score = element.select(&score_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let date = element.select(&date_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let manga_type = element.select(&type_selector).next().and_then(|e| e.value().attr("class").and_then(|s| s.split(' ').nth(1).map(|s| s.to_string()))).unwrap_or_default();
        let slug = element.select(&link_selector).next().and_then(|e| e.value().attr("href").and_then(|s| s.split('/').nth(4).map(|s| s.to_string()))).unwrap_or_default();

        data.push(MangaData {
            title,
            poster,
            chapter,
            score,
            date,
            manga_type,
            slug,
            pagination: None, // Pagination is handled separately
        });
    }
    data
}

// Placeholder for fetchWithProxyOnlyWrapper
async fn fetch_with_proxy_only_wrapper(url: &str) -> Result<String, Box<dyn Error>> {
    // In a real scenario, this would involve proxy logic.
    // For now, a direct fetch.
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

// Placeholder for getCachedKomikBaseUrl
async fn get_cached_komik_base_url(refresh: bool) -> Result<String, Box<dyn Error>> {
    // In a real scenario, this would fetch from a cache or external service.
    // For now, return a dummy URL.
    if refresh {
        Ok("http://dummy-komik-base-url-refreshed.com".to_string())
    } else {
        Ok("http://dummy-komik-base-url.com".to_string())
    }
}
#[utoipa::path(
    get,
    path = "/api/komik/detail/{komik_id}",
    params(
        ("komik_id" = String, Path, description = "ID of the komik")
    ),
    responses(
        (status = 200, description = "Manga detail", body = MangaDetail)
    ),
    tag = "Komik"
)]

pub async fn get_detail(komik_id: &str) -> Result<MangaDetail, Box<dyn Error>> {
    let base_url = get_cached_komik_base_url(false).await?;
    let body = fetch_with_proxy_only_wrapper(&format!("{}/komik/{}", base_url, komik_id)).await?;
    let document = Html::parse_document(&body);

    let title = document.select(&Selector::parse("h1.entry-title").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let alternative_title = document.select(&Selector::parse(".spe span:contains('Judul Alternatif:')").unwrap()).next().map(|e| e.text().collect::<String>().replace("Judul Alternatif:", "").trim().to_string()).unwrap_or_default();
    let score = document.select(&Selector::parse(".rtg > div > i").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let mut poster = document.select(&Selector::parse(".thumb img").unwrap()).next().and_then(|e| e.value().attr("src").map(|s| s.to_string())).unwrap_or_default();
    poster = poster.split('?').next().unwrap_or(&poster).to_string();
    let description = document.select(&Selector::parse("#sinopsis > section > div > div.entry-content.entry-content-single > p").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let status = document.select(&Selector::parse(".spe span:contains('Status:')").unwrap()).next().map(|e| e.text().collect::<String>().replace("Status:", "").trim().to_string()).unwrap_or_default();
    let manga_type = document.select(&Selector::parse(".spe span:contains('Jenis Komik:') a").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let release_date = document.select(&Selector::parse("#chapter_list > ul > li:last-child > span.dt").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let author = document.select(&Selector::parse(".spe span:contains('Pengarang:')").unwrap()).next().map(|e| e.text().collect::<String>().replace("Pengarang:", "").trim().to_string()).unwrap_or_default();
    let total_chapter = document.select(&Selector::parse("#chapter_list > ul > li:nth-child(1) > span.lchx").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
    let updated_on = document.select(&Selector::parse("#chapter_list > ul > li:nth-child(1) > span.dt").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();

    let mut genres: Vec<String> = Vec::new();
    let genre_selector = Selector::parse(".genre-info a").unwrap();
    for element in document.select(&genre_selector) {
        genres.push(element.text().collect::<String>().trim().to_string());
    }

    let mut chapters: Vec<ChapterData> = Vec::new();
    let chapter_list_selector = Selector::parse("#chapter_list ul li").unwrap();
    let chapter_link_selector = Selector::parse(".lchx a").unwrap();
    let chapter_date_selector = Selector::parse(".dt a").unwrap();

    for element in document.select(&chapter_list_selector) {
        let chapter = element.select(&chapter_link_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let date = element.select(&chapter_date_selector).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let chapter_id = element.select(&chapter_link_selector).next().and_then(|e| e.value().attr("href").and_then(|s| s.split('/').nth(3).map(|s| s.to_string()))).unwrap_or_default();
        chapters.push(ChapterData { chapter, date, chapter_id });
    }

    Ok(MangaDetail {
        title,
        alternative_title,
        score,
        poster,
        description,
        status,
        manga_type,
        release_date,
        author,
        total_chapter,
        updated_on,
        genres,
        chapters,
    })
}

#[utoipa::path(
    get,
    path = "/api/komik/chapter/{chapter_url}",
    params(
        ("chapter_url" = String, Path, description = "URL of the chapter")
    ),
    responses(
        (status = 200, description = "Manga chapter", body = MangaChapter)
    ),
    tag = "Komik"
)]
pub async fn get_chapter(chapter_url: &str) -> Result<MangaChapter, Box<dyn Error>> {
    let base_url = get_cached_komik_base_url(false).await?;
    let body = fetch_with_proxy_only_wrapper(&format!("{}/chapter/{}", base_url, chapter_url)).await?;
    let document = Html::parse_document(&body);

    let title = document.select(&Selector::parse(".entry-title").unwrap()).next().map(|e| e.text().collect::<String>().trim().to_string()).unwrap_or_default();

    let prev_chapter_id = document.select(&Selector::parse(".nextprev a[rel='prev']").unwrap())
        .next()
        .and_then(|e| e.value().attr("href").and_then(|s| s.split('/').nth(3).map(|s| s.to_string())))
        .unwrap_or_default();

    let list_chapter = document.select(&Selector::parse(".nextprev a:has(.icol.daftarch)").unwrap())
        .next()
        .and_then(|e| e.value().attr("href").and_then(|s| s.split('/').nth(4).map(|s| s.to_string())))
        .unwrap_or_default();

    let next_chapter_id = document.select(&Selector::parse(".nextprev a[rel='next']").unwrap())
        .next()
        .and_then(|e| e.value().attr("href").and_then(|s| s.split('/').nth(3).map(|s| s.to_string())))
        .unwrap_or_default();

    let mut images: Vec<String> = Vec::new();
    let image_selector = Selector::parse("#chimg-auh img").unwrap();
    for element in document.select(&image_selector) {
        images.push(element.value().attr("src").map(|s| s.to_string()).unwrap_or_default());
    }

    Ok(MangaChapter {
        title,
        next_chapter_id,
        prev_chapter_id,
        images,
        list_chapter,
    })
}

#[utoipa::path(
    get,
    path = "/api/komik/list/{manga_type}/{page}",
    params(
        ("manga_type" = String, Path, description = "Type of manga"),
        ("page" = u32, Path, description = "Page number"),
        ("query" = Option<String>, Query, description = "Search query")
    ),
    responses(
        (status = 200, description = "Manga list and pagination", body = [MangaData])
    ),
    tag = "Komik"
)]
pub async fn handle_list_or_search(
    manga_type: &str,
    page: u32,
    query: Option<&str>,
) -> Result<(Vec<MangaData>, Pagination), Box<dyn Error>> {
    let base_url = get_cached_komik_base_url(false).await?;
    let mut api_url = format!("{}/{}/page/{}/", base_url, manga_type, page);
    if let Some(q) = query {
        api_url = format!("{}/page/{}/?s={}", base_url, page, q);
    }

    let body = fetch_with_proxy_only_wrapper(&api_url).await?;
    let document = Html::parse_document(&body);

    let parsed_data = parse_manga_data(&body);

    let current_page = document.select(&Selector::parse(".pagination .current").unwrap())
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(1);

    let total_pages = document.select(&Selector::parse(".pagination a:not(.next):last").unwrap())
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse::<u32>().ok())
        .unwrap_or(current_page);

    let pagination = Pagination {
        current_page,
        last_visible_page: total_pages,
        has_next_page: document.select(&Selector::parse(".pagination .next").unwrap()).next().is_some(),
        next_page: if current_page < total_pages { Some(current_page + 1) } else { None },
        previous_page: if current_page > 1 { Some(current_page - 1) } else { None },
    };

    Ok((parsed_data, pagination))
}

#[utoipa::path(
    get,
    path = "/api/komik/external-link",
    responses(
        (status = 200, description = "External Komik base URL", body = String)
    ),
    tag = "Komik"
)]
pub async fn handle_external_link() -> Result<String, Box<dyn Error>> {
    let base_url = get_cached_komik_base_url(false).await?;
    Ok(base_url)
}
