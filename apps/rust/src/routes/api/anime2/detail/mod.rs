// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime2/detail/{slug}";
const ENDPOINT_DESCRIPTION: &str = "Fetches anime detail from alqanime.net and parses the HTML into structured JSON.";
const ENDPOINT_TAG: &str = "anime2";
const SUCCESS_RESPONSE_BODY: &str = "DetailResponse";
const SLUG_DESCRIPTION: &str = "Slug for the anime (e.g., 'isekai-ojisan').";
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
pub struct Recommendation {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub status: String,
    pub r#type: String,
}

#[derive(Serialize, ToSchema)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, ToSchema)]
pub struct DownloadGroup {
    pub resolution: String,
    pub links: Vec<Link>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnimeDetailData {
    pub title: String,
    pub alternative_title: String,
    pub poster: String,
    pub poster2: String,
    pub r#type: String,
    pub release_date: String,
    pub status: String,
    pub synopsis: String,
    pub studio: String,
    pub genres: Vec<Genre>,
    pub producers: Vec<String>,
    pub recommendations: Vec<Recommendation>,
    pub batch: Vec<DownloadGroup>,
    pub ova: Vec<DownloadGroup>,
    pub downloads: Vec<DownloadGroup>,
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

/// Fetches anime detail from alqanime.net and parses the HTML into structured JSON.
#[utoipa::path(get, path = "/api/anime2/detail/{slug}", tag = "anime2", responses((status = 200, description = "Success", body = DetailResponse), (status = 500, description = "Internal Server Error")), params(("slug" = String, Path, description = "Slug for the anime (e.g., 'isekai-ojisan').")))]
pub async fn detail_handler(Path(slug): Path<String>) -> Response {
    let client = Client::new();
    let url = format!("https://alqanime.net/{}/", slug);

    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => html,
            Err(e) => {
                return ErrorResponse {
                    message: "Failed to fetch anime detail data".to_string(),
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

    let data = parse_anime_detail(&html);

    let response = DetailResponse {
        status: "Ok",
        data,
    };

    Json(response).into_response()
}

fn parse_anime_detail(html: &str) -> AnimeDetailData {
    let document = Html::parse_document(html);

    let extract_text = |selector: &str| {
        Selector::parse(selector)
            .ok()
            .and_then(|sel| document.select(&sel).next())
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    };

    let title = extract_text(".entry-title");
    let alternative_title = extract_text(".alter");

    let poster = Selector::parse(".thumb[itemprop=\"image\"] img.lazyload")
        .ok()
        .and_then(|sel| document.select(&sel).next())
        .and_then(|img| img.value().attr("data-src"))
        .unwrap_or("")
        .to_string();

    let poster2 = Selector::parse(".bixbox.animefull .bigcover .ime img.lazyload")
        .ok()
        .and_then(|sel| document.select(&sel).next())
        .and_then(|img| img.value().attr("data-src"))
        .unwrap_or("")
        .to_string();

    let r#type = Selector::parse(".info-content .spe span")
        .ok()
        .and_then(|sel| {
            document
                .select(&sel)
                .find(|span| span.text().any(|t| t.contains("Tipe:")))
        })
        .and_then(|span| {
            span.select(&Selector::parse("a").unwrap())
                .next()
                .map(|a| a.text().collect::<String>().trim().to_string())
        })
        .unwrap_or_default();

    let release_date = Selector::parse(".info-content .spe span")
        .ok()
        .and_then(|sel| {
            document
                .select(&sel)
                .find(|span| span.text().any(|t| t.contains("Dirilis:")))
        })
        .map(|span| span.text().collect::<String>().replace("Dirilis:", "").trim().to_string())
        .unwrap_or_default();

    let status = Selector::parse(".info-content .spe span")
        .ok()
        .and_then(|sel| {
            document
                .select(&sel)
                .find(|span| span.text().any(|t| t.contains("Status:")))
        })
        .map(|span| span.text().collect::<String>().replace("Status:", "").trim().to_string())
        .unwrap_or_default();

    let synopsis = Selector::parse(".entry-content p")
        .ok()
        .and_then(|sel| document.select(&sel).next())
        .map(|n| n.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let studio = Selector::parse(".info-content .spe span")
        .ok()
        .and_then(|sel| {
            document
                .select(&sel)
                .find(|span| span.text().any(|t| t.contains("Studio:")))
        })
        .and_then(|span| {
            span.select(&Selector::parse("a").unwrap())
                .next()
                .map(|a| a.text().collect::<String>().trim().to_string())
        })
        .unwrap_or_default();

    let mut genres = Vec::new();
    if let Ok(sel) = Selector::parse(".genxed a") {
        for a in document.select(&sel) {
            let name = a.text().collect::<String>().trim().to_string();
            let anime_url = a.value().attr("href").unwrap_or("").to_string();
            let slug = anime_url.split('/').filter(|s| !s.is_empty()).last().unwrap_or("").to_string();
            genres.push(Genre { name, slug, anime_url });
        }
    }

    let batch = Vec::new();
    let ova = Vec::new();
    let mut downloads = Vec::new();

    if let Ok(soraurl_sel) = Selector::parse(".soraddl.dlone .soraurl") {
        for soraurl in document.select(&soraurl_sel) {
            let resolution = Selector::parse(".res")
                .ok()
                .and_then(|sel| soraurl.select(&sel).next())
                .map(|n| n.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let mut links = Vec::new();
            if let Ok(slink_sel) = Selector::parse(".slink a") {
                for link in soraurl.select(&slink_sel) {
                    let name = link.text().collect::<String>().trim().to_string();
                    let url = link.value().attr("href").unwrap_or("").to_string();
                    links.push(Link { name, url });
                }
            }

            // Grouping logic placeholder (not fully implemented)
            downloads.push(DownloadGroup { resolution, links });
        }
    }

    let producers = Vec::new(); // Not parsed, placeholder for compatibility

    let mut recommendations = Vec::new();
    if let Ok(bs_sel) = Selector::parse(".listupd .bs") {
        for bs in document.select(&bs_sel) {
            let title = Selector::parse(".ntitle")
                .ok()
                .and_then(|sel| bs.select(&sel).next())
                .map(|n| n.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let anime_url = Selector::parse("a")
                .ok()
                .and_then(|sel| bs.select(&sel).next())
                .and_then(|a| a.value().attr("href"))
                .unwrap_or("")
                .to_string();

            let slug = anime_url.split('/').filter(|s| !s.is_empty()).last().unwrap_or("").to_string();

            let poster = Selector::parse("img")
                .ok()
                .and_then(|sel| bs.select(&sel).next())
                .and_then(|img| img.value().attr("data-src").or_else(|| img.value().attr("src")))
                .unwrap_or("")
                .to_string();

            let status = Selector::parse(".status")
                .ok()
                .and_then(|sel| bs.select(&sel).next())
                .map(|n| n.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let r#type = Selector::parse(".typez")
                .ok()
                .and_then(|sel| bs.select(&sel).next())
                .map(|n| n.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            recommendations.push(Recommendation {
                title,
                slug,
                poster,
                status,
                r#type,
            });
        }
    }

    AnimeDetailData {
        title,
        alternative_title,
        poster,
        poster2,
        r#type,
        release_date,
        status,
        synopsis,
        studio,
        genres,
        producers,
        recommendations,
        batch,
        ova,
        downloads,
    }
}
