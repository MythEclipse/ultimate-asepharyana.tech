// Handler for GET /api/anime/detail/{slug}.
// Fetches and parses anime detail from otakudesu.cloud using reqwest and scraper.

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Serialize)]
struct Genre {
    name: String,
    slug: String,
    anime_url: String,
}

#[derive(Serialize)]
struct Episode {
    episode: String,
    slug: String,
}

#[derive(Serialize)]
struct Recommendation {
    title: String,
    slug: String,
    poster: String,
    status: String,
    r#type: String,
}

#[derive(Serialize)]
struct AnimeDetail {
    title: String,
    alternative_title: String,
    poster: String,
    r#type: String,
    release_date: String,
    status: String,
    synopsis: String,
    studio: String,
    genres: Vec<Genre>,
    producers: Vec<String>,
    recommendations: Vec<Recommendation>,
    batch: Vec<Episode>,
    episode_lists: Vec<Episode>,
}

#[derive(Serialize)]
struct AnimeDetailResponse {
    status: &'static str,
    data: AnimeDetail,
}

pub async fn detail_handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!("https://otakudesu.cloud/anime/{}", slug);

    let html = match client.get(&url).send().await.and_then(|r| r.text().await) {
        Ok(html) => html,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "message": "Failed to fetch anime detail data",
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let document = Html::parse_document(&html);

    // Helper to extract text
    fn extract_text(document: &Html, selector: &str, prefix: &str) -> String {
        let sel = Selector::parse(selector).unwrap();
        document
            .select(&sel)
            .next()
            .map(|n| n.text().collect::<String>().replace(prefix, "").trim().to_string())
            .unwrap_or_default()
    }

    let title = extract_text(&document, ".infozingle p:contains(\"Judul\")", "Judul: ");
    let alternative_title = extract_text(&document, ".infozingle p:contains(\"Japanese\")", "Japanese: ");
    let poster = {
        let sel = Selector::parse(".fotoanime img").unwrap();
        document
            .select(&sel)
            .next()
            .and_then(|n| n.value().attr("src"))
            .unwrap_or("")
            .to_string()
    };
    let r#type = extract_text(&document, ".infozingle p:contains(\"Tipe\")", "Tipe: ");
    let release_date = extract_text(&document, ".infozingle p:contains(\"Tanggal Rilis\")", "Tanggal Rilis: ");
    let status = extract_text(&document, ".infozingle p:contains(\"Status\")", "Status: ");
    let synopsis = {
        let sel = Selector::parse(".sinopc").unwrap();
        document
            .select(&sel)
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    };
    let studio = extract_text(&document, ".infozingle p:contains(\"Studio\")", "Studio: ");

    // Genres
    let mut genres = Vec::new();
    if let Ok(sel) = Selector::parse(".infozingle p:contains(\"Genre\") a") {
        for element in document.select(&sel) {
            let name = element.text().collect::<String>().trim().to_string();
            let anime_url = element.value().attr("href").unwrap_or("").to_string();
            let slug = anime_url.split('/').nth(4).unwrap_or("").to_string();
            genres.push(Genre { name, slug, anime_url });
        }
    }

    // Episodes and batch
    let mut episode_lists = Vec::new();
    let mut batch = Vec::new();
    if let Ok(sel) = Selector::parse(".episodelist ul li span a") {
        for element in document.select(&sel) {
            let episode = element.text().collect::<String>().trim().to_string();
            let href = element.value().attr("href").unwrap_or("");
            let segments: Vec<&str> = href.split('/').collect();
            let episode_slug = segments.last().unwrap_or(&"").to_string();
            let ep = Episode { episode: episode.clone(), slug: episode_slug };
            if episode.to_lowercase().contains("batch") {
                batch.push(ep);
            } else {
                episode_lists.push(ep);
            }
        }
    }

    // Producers
    let producers = extract_text(&document, ".infozingle p:contains(\"Produser\")", "Produser: ")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    // Recommendations
    let mut recommendations = Vec::new();
    if let Ok(sel) = Selector::parse("#recommend-anime-series .isi-anime") {
        for element in document.select(&sel) {
            let title = element
                .select(&Selector::parse(".judul-anime a").unwrap())
                .next()
                .map(|n| n.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            let url = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|n| n.value().attr("href"))
                .unwrap_or("")
                .to_string();
            let poster = element
                .select(&Selector::parse("img").unwrap())
                .next()
                .and_then(|n| n.value().attr("src"))
                .unwrap_or("")
                .to_string();
            let slug = url.split('/').nth(4).unwrap_or("").to_string();
            recommendations.push(Recommendation {
                title,
                slug,
                poster,
                status: "".to_string(),
                r#type: "".to_string(),
            });
        }
    }

    let data = AnimeDetail {
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
    };

    let response = AnimeDetailResponse {
        status: "Ok",
        data,
    };

    Json(response).into_response()
}
