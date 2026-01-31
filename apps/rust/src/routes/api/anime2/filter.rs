use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{default_backoff, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::models::anime2::{FilterAnimeItem, Pagination};
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::Router;
use backoff::future::retry;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct FiltersApplied {
    pub genre: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub order: String,
}

#[derive(Deserialize, ToSchema)]
pub struct FilterQuery {
    pub page: Option<u32>,
    pub genre: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub order: Option<String>,
}

// Static selectors using once_cell (replacing lazy_static)
static ITEM_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("article.bs").unwrap());
static TITLE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".tt h2").unwrap());
static IMG_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("img").unwrap());
static SCORE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".numscore").unwrap());
static STATUS_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".status").unwrap());
static TYPE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".type").unwrap());
static LINK_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("a").unwrap());
static PAGINATION_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".pagination .page-numbers:not(.next)").unwrap());
static NEXT_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".pagination .next").unwrap());
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 300;

#[utoipa::path(
    get,
    params(
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1),
        ("genre" = Option<String>, Query, description = "Parameter for resource identification", example = "sample_value"),
        ("status" = Option<String>, Query, description = "Status filter (active, inactive, pending, etc.)", example = "sample_value"),
        ("type" = Option<String>, Query, description = "Content type filter", example = "sample_value"),
        ("order" = Option<String>, Query, description = "Sort direction (ascending or descending)", example = "sample_value")
    ),
    path = "/api/anime2/filter",
    tag = "anime2",
    operation_id = "anime2_filter",
    responses(
        (status = 200, description = "Advanced multi-filter search for anime2", body = ApiResponse<Vec<FilterAnimeItem>>),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn filter(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FilterQuery>,
) -> ApiResult<Vec<FilterAnimeItem>> {
    let page = params.page.unwrap_or(1);
    let genre = params.genre.clone();
    let status = params.status.clone();
    let anime_type = params.r#type.clone();
    let order = params.order.clone().unwrap_or("update".to_string());

    info!(
        "anime2 filter: page={}, genre={:?}, status={:?}, type={:?}, order={}",
        page, genre, status, anime_type, order
    );

    let cache_key = format!(
        "anime2:filter:{}:{:?}:{:?}:{:?}:{}",
        page, genre, status, anime_type, order
    );
    let cache = Cache::new(&app_state.redis_pool);

    let genre_clone = genre.clone();
    let status_clone = status.clone();
    let type_clone = anime_type.clone();
    let order_clone = order.clone();

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (data, pagination) =
                fetch_filtered_anime(page, &genre, &status, &anime_type, &order)
                    .await
                    .map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs concurrently
            let posters: Vec<String> = data.iter().map(|i| i.poster.clone()).collect();
            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                app_state.db.clone(),
                &app_state.redis_pool,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            let mut final_data = data;
            for (i, item) in final_data.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            let meta = json!({
                "pagination": pagination,
                "filters_applied": {
                    "genre": genre_clone,
                    "status": status_clone,
                    "type": type_clone,
                    "order": order_clone,
                },
                "status": "Ok"
            });

            Ok(ApiResponse::success_with_meta(final_data, meta))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_filtered_anime(
    page: u32,
    genre: &Option<String>,
    status: &Option<String>,
    anime_type: &Option<String>,
    order: &str,
) -> Result<(Vec<FilterAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let mut url = if page > 1 {
        format!("https://alqanime.si/anime/page/{}/?order={}", page, order)
    } else {
        format!("https://alqanime.si/anime/?order={}", order)
    };

    if let Some(g) = genre {
        for genre_item in g.split(',') {
            url.push_str(&format!("&genre[]={}", genre_item.trim()));
        }
    }
    if let Some(s) = status {
        url.push_str(&format!("&status={}", s));
    }
    if let Some(t) = anime_type {
        url.push_str(&format!("&type={}", t));
    }

    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching: {}", url);
        match fetch_with_proxy(&url).await {
            Ok(response) => Ok(response.data),
            Err(e) => {
                warn!("Failed: {:?}", e);
                Err(transient(e))
            }
        }
    };

    let html = retry(backoff, fetch_operation).await?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_filter_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_filter_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<FilterAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);
    let mut anime_list = Vec::new();

    for element in document.select(&ITEM_SELECTOR) {
        let title = element
            .select(&TITLE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let poster = element
            .select(&IMG_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
            .unwrap_or("")
            .to_string();

        let score = element
            .select(&SCORE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let status = element
            .select(&STATUS_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("Unknown".to_string());

        let anime_type = element
            .select(&TYPE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("Unknown".to_string());

        let anime_url = element
            .select(&LINK_SELECTOR)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let slug = SLUG_REGEX
            .captures(&anime_url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        if !title.is_empty() {
            anime_list.push(FilterAnimeItem {
                title,
                slug,
                poster,
                score,
                status,
                r#type: anime_type,
                anime_url,
            });
        }
    }

    let last_visible_page = document
        .select(&PAGINATION_SELECTOR)
        .next_back()
        .map(|e| {
            e.text()
                .collect::<String>()
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        })
        .unwrap_or(1);

    let has_next_page = document.select(&NEXT_SELECTOR).next().is_some();
    let pagination = Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page: if has_next_page {
            Some(current_page + 1)
        } else {
            None
        },
        has_previous_page: current_page > 1,
        previous_page: if current_page > 1 {
            Some(current_page - 1)
        } else {
            None
        },
    };

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}