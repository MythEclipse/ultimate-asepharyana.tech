use crate::helpers::{default_backoff, internal_err, parse_html, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::scraping::urls::get_komik_url;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;
use lazy_static::lazy_static;
use scraper::Selector;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the komik home endpoint.";
pub const ENDPOINT_TAG: &str = "komik";
pub const OPERATION_ID: &str = "komik_index";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<KomikHomeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct KomikHomeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub r#type: String,
    pub chapter: String,
    pub score: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct KomikHomeData {
    pub popular: Vec<KomikHomeItem>,
    pub latest: Vec<KomikHomeItem>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct KomikHomeResponse {
    pub status: String,
    pub data: KomikHomeData,
}

const CACHE_KEY: &str = "komik:index";
const CACHE_TTL: u64 = 300; // 5 minutes

#[utoipa::path(
    get,
    path = "/api/komik",
    tag = "komik",
    operation_id = "komik_index",
    responses(
        (status = 200, description = "Handles GET requests for the komik home endpoint.", body = KomikHomeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn list(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Handling request for komik index");

    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(CACHE_KEY, CACHE_TTL, || async {
            let data = fetch_komik_home().await.map_err(|e| e.to_string())?;
            Ok(KomikHomeResponse {
                status: "Ok".to_string(),
                data,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response))
}

async fn fetch_komik_home() -> Result<KomikHomeData, Box<dyn std::error::Error + Send + Sync>> {
    let url = get_komik_url();
    let backoff = default_backoff();

    let fetch_operation = || async {
        info!("Fetching URL: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => {
                info!("Successfully fetched URL: {}", url);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch URL: {}, error: {:?}", url, e);
                Err(transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;

    tokio::task::spawn_blocking(move || parse_komik_home(&html)).await?
}

lazy_static! {
    static ref POPULAR_SELECTOR: Selector = Selector::parse("#Populer .pop-manga, .pop-manga").unwrap();
    static ref LATEST_SELECTOR: Selector = Selector::parse(".listupd .bge, .bge").unwrap();

    // Common Selectors
    static ref TITLE_SELECTOR: Selector = Selector::parse("h3, .judul").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref TYPE_SELECTOR: Selector = Selector::parse(".type, .tpe1_inf").unwrap();
    static ref CHAPTER_SELECTOR: Selector = Selector::parse(".chapter, .new1, .lch").unwrap();
    static ref SCORE_SELECTOR: Selector = Selector::parse(".score, .numscore").unwrap();
}

fn parse_komik_home(html: &str) -> Result<KomikHomeData, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);

    // Parse Popular
    let mut popular = Vec::new();
    // Assuming there is a section for popular. Adjust selector if needed.
    // If #Populer exists, iterate children.
    for element in document.select(&POPULAR_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let slug = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                href.trim_end_matches('/')
                    .split('/')
                    .last()
                    .unwrap_or("")
                    .to_string()
            })
            .unwrap_or_default();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src").or_else(|| e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let r#type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Manga".to_string()); // Default to Manga

        let chapter = element
            .select(&CHAPTER_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let score = element
            .select(&SCORE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        if !title.is_empty() {
            popular.push(KomikHomeItem {
                title,
                slug,
                poster,
                r#type,
                chapter,
                score,
            });
        }
    }

    // Parse Latest
    let mut latest = Vec::new();
    for element in document.select(&LATEST_SELECTOR).take(12) {
        // Limit to 12 as per frontend array(12) hint
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let slug = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                href.trim_end_matches('/')
                    .split('/')
                    .last()
                    .unwrap_or("")
                    .to_string()
            })
            .unwrap_or_default();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src").or_else(|| e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let r#type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Manga".to_string());

        let chapter = element
            .select(&CHAPTER_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let score = "0.0".to_string(); // Latest list often doesn't show score on cards

        if !title.is_empty() {
            latest.push(KomikHomeItem {
                title,
                slug,
                poster,
                r#type,
                chapter,
                score,
            });
        }
    }

    Ok(KomikHomeData { popular, latest })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(list))
}