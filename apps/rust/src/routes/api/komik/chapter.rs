//! Handler for the chapter endpoint.
    #![allow(dead_code)]

    use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use scraper::{Html, Selector};
    use rust_lib::fetch_with_proxy::fetch_with_proxy_only;
    use rust_lib::komik_base_url::get_cached_komik_base_url;
    use tracing::{info, error};

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/api/komik/chapter";
    pub const ENDPOINT_DESCRIPTION: &str = "Retrieves chapter data for a specific komik chapter.";
    pub const ENDPOINT_TAG: &str = "komik";
    pub const OPERATION_ID: &str = "komik_chapter";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<ChapterResponse>";

    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct ChapterData {
        pub title: String,
        pub next_chapter_id: String,
        pub prev_chapter_id: String,
        pub images: Vec<String>,
    }

    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct ChapterResponse {
        pub message: String,
        pub data: ChapterData,
    }

    #[derive(Deserialize)]
    pub struct ChapterQuery {
        pub chapter_url: Option<String>,
    }

    #[utoipa::path(
    get,
    path = "/api/api/komik/chapter",
    tag = "komik",
    operation_id = "komik_chapter",
    responses(
        (status = 200, description = "Retrieves chapter data for a specific komik chapter.", body = ChapterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn chapter(Query(params): Query<ChapterQuery>) -> impl IntoResponse {
        let chapter_url = params.chapter_url.unwrap_or_default();

        match get_cached_komik_base_url(false).await {
            Ok(base_url) => {
                match fetch_and_parse_chapter(&chapter_url, &base_url).await {
                    Ok(data) => {
                        info!("[komik][chapter] Success for chapter_url: {}", chapter_url);
                        Json(ChapterResponse {
                            message: "Chapter data retrieved successfully".to_string(),
                            data,
                        })
                    },
                    Err(e) => {
                        error!("[komik][chapter] Error parsing chapter for {}: {:?}", chapter_url, e);
                        Json(ChapterResponse {
                            message: "Failed to fetch chapter data".to_string(),
                            data: ChapterData {
                                title: "".to_string(),
                                next_chapter_id: "".to_string(),
                                prev_chapter_id: "".to_string(),
                                images: vec![],
                            },
                        })
                    },
                }
            },
            Err(e) => {
                error!("[komik][chapter] Error getting base URL: {:?}", e);
                Json(ChapterResponse {
                    message: "Failed to get base URL".to_string(),
                    data: ChapterData {
                        title: "".to_string(),
                        next_chapter_id: "".to_string(),
                        prev_chapter_id: "".to_string(),
                        images: vec![],
                    },
                })
            }
        }
    }

    async fn fetch_and_parse_chapter(chapter_url: &str, base_url: &str) -> Result<ChapterData, Box<dyn std::error::Error>> {
        let url = format!("{}/chapter/{}", base_url, chapter_url);
        info!("[fetch_and_parse_chapter] Fetching URL: {}", url);

        let response = fetch_with_proxy_only(&url).await?;
        let html = response.data;
        let document = Html::parse_document(&html);

        let title = document
            .select(&Selector::parse(".entry-title").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let prev_chapter_element = document
            .select(&Selector::parse(".nextprev a[rel=\"prev\"]").unwrap())
            .next();
        let prev_chapter_id = if let Some(element) = prev_chapter_element {
            element.value().attr("href")
                .and_then(|href| href.split('/').nth(3))
                .unwrap_or("")
                .to_string()
        } else {
            "".to_string()
        };

        let next_chapter_element = document
            .select(&Selector::parse(".nextprev a[rel=\"next\"]").unwrap())
            .next();
        let next_chapter_id = if let Some(element) = next_chapter_element {
            element.value().attr("href")
                .and_then(|href| href.split('/').nth(3))
                .unwrap_or("")
                .to_string()
        } else {
            "".to_string()
        };

        let mut images = Vec::new();
        for element in document.select(&Selector::parse("#chimg-auh img").unwrap()) {
            if let Some(src) = element.value().attr("src") {
                images.push(src.to_string());
            }
        }

        info!("[fetch_and_parse_chapter] Successfully parsed chapter for {}", chapter_url);
        Ok(ChapterData {
            title,
            next_chapter_id,
            prev_chapter_id,
            images,
        })
    }

    /// Handles GET requests for the komik/chapter endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(chapter))
}