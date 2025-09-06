//! Handler for the detail endpoint.
    #![allow(dead_code)]

    use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use reqwest;
    use scraper::{Html, Selector};
    use rust_lib::config::CONFIG_MAP;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/api/komik/detail";
    pub const ENDPOINT_DESCRIPTION: &str = "Retrieves details for a specific komik by ID.";
    pub const ENDPOINT_TAG: &str = "komik";
    pub const OPERATION_ID: &str = "komik_detail";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct EpisodeList {
        pub quality: String,
    }

    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct Recommendation {
        pub title: String,
        pub poster: String,
        pub komik_id: String,
    }

    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct DetailData {
        pub title: String,
        pub japanese_title: String,
        pub poster: String,
        pub rating: String,
        pub credit: String,
        pub r#type: String,
        pub status: String,
        pub episode_count: String,
        pub duration: String,
        pub release_date: String,
        pub studio: String,
        pub genres: Vec<String>,
        pub synopsis: String,
        pub episode_lists: Vec<EpisodeList>,
        pub recommendations: Vec<Recommendation>,
    }

    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct DetailResponse {
        pub status: bool,
        pub data: DetailData,
    }

    #[derive(Deserialize)]
    pub struct DetailQuery {
        pub komik_id: Option<String>,
    }

    #[utoipa::path(
    get,
    path = "/api/api/komik/detail",
    tag = "komik",
    operation_id = "komik_detail",
    responses(
        (status = 200, description = "Retrieves details for a specific komik by ID.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn detail(Query(params): Query<DetailQuery>) -> impl IntoResponse {
        let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
        let base_url = CONFIG_MAP
            .get("KOMIK_BASE_URL")
            .cloned()
            .unwrap_or_else(|| "https://komikindo.id".to_string());

        match fetch_and_parse_detail(&komik_id, &base_url).await {
            Ok(data) => Json(DetailResponse {
                status: true,
                data,
            }),
            Err(_) => Json(DetailResponse {
                status: false,
                data: DetailData {
                    title: "".to_string(),
                    japanese_title: "".to_string(),
                    poster: "".to_string(),
                    rating: "".to_string(),
                    credit: "".to_string(),
                    r#type: "".to_string(),
                    status: "".to_string(),
                    episode_count: "".to_string(),
                    duration: "".to_string(),
                    release_date: "".to_string(),
                    studio: "".to_string(),
                    genres: vec![],
                    synopsis: "".to_string(),
                    episode_lists: vec![],
                    recommendations: vec![],
                },
            }),
        }
    }

    async fn fetch_and_parse_detail(komik_id: &str, base_url: &str) -> Result<DetailData, Box<dyn std::error::Error>> {
        let url = format!("{}/komik/{}", base_url, komik_id);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let title = document
            .select(&Selector::parse("h1.entry-title").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let japanese_title = document
            .select(&Selector::parse(".spe span:contains('Judul Alternatif:')").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().replace("Judul Alternatif:", "").trim().to_string())
            .unwrap_or_default();

        let mut poster = document
            .select(&Selector::parse(".thumb img").unwrap())
            .next()
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();
        if let Some(pos) = poster.find('?') {
            poster = poster[..pos].to_string();
        }

        let rating = document
            .select(&Selector::parse(".rtg > div > i").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let credit = document
            .select(&Selector::parse(".spe span:contains('Pengarang:')").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().replace("Pengarang:", "").trim().to_string())
            .unwrap_or_default();

        let r#type = document
            .select(&Selector::parse(".spe span:contains('Jenis Komik:') a").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let status = document
            .select(&Selector::parse(".spe span:contains('Status:')").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().replace("Status:", "").trim().to_string())
            .unwrap_or_default();

        let synopsis = document
            .select(&Selector::parse("#sinopsis > section > div > div.entry-content.entry-content-single > p").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let release_date = document
            .select(&Selector::parse("#chapter_list > ul > li:last-child > span.dt").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let episode_count = document
            .select(&Selector::parse("#chapter_list > ul > li:nth-child(1) > span.lchx").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let mut genres = Vec::new();
        for element in document.select(&Selector::parse(".genre-info a").unwrap()) {
            genres.push(element.text().collect::<String>().trim().to_string());
        }

        let mut episode_lists = Vec::new();
        for _ in document.select(&Selector::parse("#chapter_list ul li").unwrap()) {
            episode_lists.push(EpisodeList {
                quality: "default".to_string(),
            });
        }

        Ok(DetailData {
            title,
            japanese_title,
            poster,
            rating,
            credit,
            r#type,
            status,
            episode_count,
            duration: "".to_string(),
            release_date,
            studio: "".to_string(),
            genres,
            synopsis,
            episode_lists,
            recommendations: vec![],
        })
    }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(detail))
}