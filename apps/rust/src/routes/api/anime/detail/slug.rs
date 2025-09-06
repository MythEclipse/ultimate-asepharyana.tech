use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use reqwest;
use scraper::{Html, Selector};

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime/detail/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/detail/{slug} endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_detail_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EpisodeList {
    pub episode: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Recommendation {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub status: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeDetailData {
    pub title: String,
    pub alternative_title: String,
    pub poster: String,
    pub r#type: String,
    pub status: String,
    pub release_date: String,
    pub studio: String,
    pub genres: Vec<Genre>,
    pub synopsis: String,
    pub episode_lists: Vec<EpisodeList>,
    pub batch: Vec<EpisodeList>,
    pub producers: Vec<String>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
    pub status: String,
    pub data: AnimeDetailData,
}

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "The slug identifier")
    ),
    path = "/api/api/anime/detail/{slug}",
    tag = "anime",
    operation_id = "anime_detail_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/detail/{slug} endpoint.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
    match fetch_anime_detail(&slug).await {
        Ok(data) => Json(DetailResponse {
            status: "Ok".to_string(),
            data,
        }),
        Err(_) => Json(DetailResponse {
            status: "Error".to_string(),
            data: AnimeDetailData {
                title: "".to_string(),
                alternative_title: "".to_string(),
                poster: "".to_string(),
                r#type: "".to_string(),
                status: "".to_string(),
                release_date: "".to_string(),
                studio: "".to_string(),
                genres: vec![],
                synopsis: "".to_string(),
                episode_lists: vec![],
                batch: vec![],
                producers: vec![],
                recommendations: vec![],
            },
        }),
    }
}

async fn fetch_anime_detail(slug: &str) -> Result<AnimeDetailData, Box<dyn std::error::Error>> {
    let url = format!("https://otakudesu.cloud/anime/{}", slug);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let title = document
        .select(&Selector::parse(".infozingle p:contains('Judul')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Judul: ", "").trim().to_string())
        .unwrap_or_default();

    let alternative_title = document
        .select(&Selector::parse(".infozingle p:contains('Japanese')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Japanese: ", "").trim().to_string())
        .unwrap_or_default();

    let poster = document
        .select(&Selector::parse(".fotoanime img").unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .unwrap_or("")
        .to_string();

    let r#type = document
        .select(&Selector::parse(".infozingle p:contains('Tipe')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Tipe: ", "").trim().to_string())
        .unwrap_or_default();

    let release_date = document
        .select(&Selector::parse(".infozingle p:contains('Tanggal Rilis')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Tanggal Rilis: ", "").trim().to_string())
        .unwrap_or_default();

    let status = document
        .select(&Selector::parse(".infozingle p:contains('Status')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Status: ", "").trim().to_string())
        .unwrap_or_default();

    let synopsis = document
        .select(&Selector::parse(".sinopc").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let studio = document
        .select(&Selector::parse(".infozingle p:contains('Studio')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Studio: ", "").trim().to_string())
        .unwrap_or_default();

    let mut genres = Vec::new();
    for element in document.select(&Selector::parse(".infozingle p:contains('Genre') a").unwrap()) {
        let name = element.text().collect::<String>().trim().to_string();
        let genre_slug = element.value().attr("href")
            .and_then(|href| href.split('/').nth(4))
            .unwrap_or("")
            .to_string();
        let anime_url = element.value().attr("href").unwrap_or("").to_string();
        genres.push(Genre { name, slug: genre_slug, anime_url });
    }

    let mut episode_lists = Vec::new();
    let mut batch = Vec::new();
    for element in document.select(&Selector::parse(".episodelist ul li span a").unwrap()) {
        let episode = element.text().collect::<String>().trim().to_string();
        let href = element.value().attr("href").unwrap_or("");
        let episode_slug = href.split('/').last().unwrap_or("").to_string();

        if episode.to_lowercase().contains("batch") {
            batch.push(EpisodeList { episode, slug: episode_slug });
        } else {
            episode_lists.push(EpisodeList { episode, slug: episode_slug });
        }
    }

    let producers_text = document
        .select(&Selector::parse(".infozingle p:contains('Produser')").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().replace("Produser: ", "").trim().to_string())
        .unwrap_or_default();
    let producers = producers_text.split(',').map(|s| s.trim().to_string()).collect();

    let mut recommendations = Vec::new();
    for element in document.select(&Selector::parse("#recommend-anime-series .isi-anime").unwrap()) {
        let title = element
            .select(&Selector::parse(".judul-anime a").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let url = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let poster = element
            .select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|e| e.value().attr("src"))
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

    Ok(AnimeDetailData {
        title,
        alternative_title,
        poster,
        r#type,
        status,
        release_date,
        studio,
        genres,
        synopsis,
        episode_lists,
        batch,
        producers,
        recommendations,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}