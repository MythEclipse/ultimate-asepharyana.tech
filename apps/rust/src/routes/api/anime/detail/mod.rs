// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime/detail/{slug}";
const ENDPOINT_DESCRIPTION: &str = "Fetches and parses anime detail from otakudesu.cloud";
const ENDPOINT_TAG: &str = "anime";
const SUCCESS_RESPONSE_BODY: &str = "DetailResponse";
const SLUG_DESCRIPTION: &str = "Slug for the anime detail (e.g., 'isekai-ojisan-sub-indo').";
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
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub anime_url: String,
}

#[derive(Serialize, ToSchema)]
pub struct Episode {
    pub episode: String,
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
pub struct Recommendation {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub status: String,
    pub r#type: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnimeDetailData {
    pub title: String,
    pub alternative_title: String,
    pub poster: String,
    pub r#type: String,
    pub release_date: String,
    pub status: String,
    pub synopsis: String,
    pub studio: String,
    pub genres: Vec<Genre>,
    pub producers: Vec<String>,
    pub recommendations: Vec<Recommendation>,
    pub batch: Vec<Episode>,
    pub episode_lists: Vec<Episode>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DetailResponse {
    pub status: &'static str,
    pub data: AnimeDetailData,
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

/// Fetches and parses anime detail from otakudesu.cloud
#[utoipa::path(get, path = "/api/anime/detail/{slug}", tag = "anime", responses((status = 200, description = "Success", body = DetailResponse), (status = 500, description = "Internal Server Error")), params(("slug" = String, Path, description = "Slug for the anime detail (e.g., 'isekai-ojisan-sub-indo').")))]
pub async fn detail_handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!("https://otakudesu.cloud/anime/{}", slug);

    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return ErrorResponse {
                    message: "Failed to read anime detail response body".to_string(),
                    error: e.to_string(),
                }.into_response();
            }
        },
        Err(e) => {
            return ErrorResponse {
                message: "Failed to fetch anime detail data".to_string(),
                error: e.to_string(),
            }.into_response();
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

    let data = AnimeDetailData {
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

    let response = DetailResponse {
        status: "Ok",
        data,
    };

    Json(response).into_response()
}
