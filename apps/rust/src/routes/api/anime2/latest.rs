use crate::helpers::{default_backoff, internal_err, transient, Cache};
use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use backoff::future::retry;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime2/latest";
pub const ENDPOINT_DESCRIPTION: &str = "Get latest anime2 updates with pagination";
pub const ENDPOINT_TAG: &str = "anime2";
pub const OPERATION_ID: &str = "anime2_latest";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LatestAnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LatestAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub score: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<u32>,
    pub has_previous_page: bool,
    pub previous_page: Option<u32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LatestAnimeResponse {
    pub status: String,
    pub data: Vec<LatestAnimeItem>,
    pub pagination: Pagination,
}

#[derive(Deserialize, ToSchema)]
pub struct LatestQuery {
    pub page: Option<u32>,
}

lazy_static! {
    static ref ITEM_SELECTOR: Selector = Selector::parse("article.bs").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".tt h2").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref EPISODE_SELECTOR: Selector = Selector::parse(".epx").unwrap();
    static ref SCORE_SELECTOR: Selector = Selector::parse(".numscore").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref PAGINATION_SELECTOR: Selector =
        Selector::parse(".pagination .page-numbers:not(.next)").unwrap();
    static ref NEXT_SELECTOR: Selector = Selector::parse(".pagination .next").unwrap();
}

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

const CACHE_TTL: u64 = 120;

#[utoipa::path(
    get,
    params(
        ("page" = Option<u32>, Query, description = "Page number for pagination (starts from 1)", example = 1, minimum = 1)
    ),
    path = "/api/anime2/latest",
    tag = "anime2",
    operation_id = "anime2_latest",
    responses(
        (status = 200, description = "Get latest anime2 updates with pagination", body = LatestAnimeResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn latest(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<LatestQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    info!("anime2 latest request, page: {}", page);

    let cache_key = format!("anime2:latest:{}", page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (anime_list, pagination) =
                fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Convert all poster URLs to CDN URLs (returns original + background cache)
            // Trigger background caching for all posters and return immediately
            // This ensures cold start is fast (returning original URLs) while caching happens in background
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            // Extract all poster URLs
            let posters: Vec<String> = anime_list
                .iter()
                .map(|item| item.poster.clone())
                .filter(|p| !p.is_empty())
                .collect();

            // Trigger lazy batch caching
            crate::helpers::image_cache::cache_image_urls_batch_lazy(&db, &redis, posters);

            // For current response, try to use cached version if available (best effort without waiting)
            // Note: On first load, this will likely return original URLs, which is fine for speed.
            // On second load, it will return cached URLs.
            // We do a quick check against Redis/DB here? No, that would slow it down again.
            // The lazy batch function above spawns a background task.
            // We just return the anime_list with original URLs.
            // The frontend will load original URLs this time.

            // OPTIONAL: If we want "best effort" check, we could do it, but to guarantee <10s, we return immediately.
            // Let's rely on the next refresh to pick up cached URLs.
            // Or better: use `get_cached_or_original` which is fast?
            // `get_cached_or_original` spawns a background task if MISS.
            // So we can map over items fast?

            // Actually, `get_cached_or_original` checks Redis (fast) and DB (fast-ish).
            // If the delay was 10s, it might be the Browser or Sync processing.
            // But let's stick to true "lazy" to be safe.
            // We will just return the original list. The `cache_image_urls_batch_lazy` handles the background work.
            // Wait - we need to return cached URLs if they DO exist?
            // Yes, user wants speed. If cached, it is fast. If not, return original.

            // Let's use `cache_image_urls_batch_lazy` which returns the original URLs passed in
            // BUT it doesn't modify our struct.

            // Correct approach for max speed:
            // 1. Spawn background caching for all.
            // 2. Return what we have.
            // BUT, if we have cached urls, we should use them?
            // `join_all_limited` was checking 20 items. 20 Redis GETs = ~10ms.
            // The 10s delay IS the browser/scraping. The caching adds maybe 100ms.
            // HOWEVER, to be absolutely sure we don't block, we can skip the check on cold start.

            // Let's attempt to use "best effort" check using a fast parallel lookup without the `upload` part?
            // `get_cached_or_original` DOES check Redis/DB.
            // If the user said "10s", that's scraping.
            // But to reduce ANY overhead, let's make the poster processing completely background on first load
            // UNLESS we already know it's cached.

            // Decision: Use `get_cached_or_original` but ensure it doesn't block on uploads.
            // `get_cached_or_original` spawns background task on MISS. So it IS non-blocking for upload.
            // So the blocking part is just Redis/DB roundtrip.
            // We will keep `join_all_limited` but maybe increase concurrency to 50?
            // OR - maybe the `upload` part WAS blocking?
            // `get_cached_or_original` implementation:
            // ...
            // Not cached and not being cached -> start background caching -> return original.
            // So it IS lazy.

            // Why did I think it was slow?
            // Maybe `join_all_limited` overhead?
            // Let's try to remove `join_all_limited` and just map with `get_cached_or_original` concurrently
            // without the limit, or just iterating?
            // Iterating 20 async calls sequentially is slow.
            // `join_all` is fast.

            // Let's simply replace with `join_all_limited(..., 50)` (process all at once)
            // AND ensure we aren't waiting for the background task spawn? Use `tokio::spawn`?
            // `get_cached_or_original` calls `tokio::spawn`.

            // The only way to be faster is to NOT check Redis/DB at all on first load.
            // But that means even if cached, we might show original? No, we want cached if available.

            // I will optimize by processing ALL 20 items in parallel (concurrency 20 or more).
            // And I will verify `get_cached_or_original` is indeed non-blocking on upload.

            // Wait, looking at `get_cached_or_original` in `image_cache.rs` (Step 531):
            // It calls `get_cdn_url` (Redis Get -> DB Find).
            // This is fast.
            // If miss, it calls `tokio::spawn` and returns original.

            // SO: My previous code WAS good.
            // The 10s delay IS the scraping.
            // `fetch_latest_anime` calls `fetch_with_proxy` (anime2 uses proxy/scraper).
            // `anime2` uses `headless_chrome`.

            // To make `anime2` faster on cold start, we must cache the *anime list itself* (which we do, 120s TTL).
            // But the *first* hit will always be slow.
            // Unless we pre-warm it?
            // Or... the user implies "it was slow, now it is fast".
            // "itukan cepat karena sudah di cache, pas awalnya tidak, bisa sampe 10s"
            // Translation: "That is fast because it is already cached, at first it wasn't, could be up to 10s"

            // User understands it's fast *because* of cache.
            // They are complaining about the *cold* start.
            // If I can't speed up cold start (scraping), I must tell them "That is unavoidable for fresh data".
            // BUT, I can make sure *posters* don't make it 12s instead of 10s.

            // Let's make sure we are not waiting on anything unnecessary.

            // I will replace `join_all_limited` with a simple mapping that triggers background checks
            // BUT converts to cached URL if fast retrieval is possible.
            // Actually, `join_all_limited` with 20 items and concurrency 20 IS `join_all`.
            // So it is already max parallel.

            // I will update the code to clearly "fire and forget" the poster checks in a way
            // that emphasizes we prioritize the RESPONSE over the poster replacement on cold start.
            // Actually, if we just return original URLs on cold start, it's fine.
            // The frontend will load them (original).
            // Background task caches them.
            // Next refresh -> Cached URLs.

            // Modified plan:
            // Identify if querying Redis/DB for 20 items is "slow" (e.g. 500ms).
            // If so, we can skip it and just fire background tasks.
            // usage of `cache_image_urls_batch_lazy` returns original URLs immediately.
            // This skips the Redis/DB check entirely for the response thread.
            // PRO: Response is faster (saves Redis/DB RTT).
            // CON: First load always uses original URLs (hotlinking).
            // This seems like what the user wants ("make it fast").

            // I will switch to `cache_image_urls_batch_lazy` for `anime2` and `anime`.
            // This eliminates the Redis/DB check latency from the response time.

            // Extract posters
            let posters: Vec<String> = anime_list.iter().map(|i| i.poster.clone()).collect();

            // Fire background processing
            crate::helpers::image_cache::cache_image_urls_batch_lazy(&db, &redis, posters);

            // We return `anime_list` as is (with original posters).
            // Effectively 0ms added latency.

            Ok(LatestAnimeResponse {
                status: "Ok".to_string(),
                data: anime_list,
                pagination,
            })
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(Json(response).into_response())
}

async fn fetch_latest_anime(
    page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://alqanime.si/anime/page/{}/?status=&type=&order=latest",
        page
    );

    let backoff = default_backoff();
    let fetch_operation = || -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, backoff::Error<std::io::Error>>> + Send>> {
        let url = url.clone();
        Box::pin(async move {
            info!("Fetching: {}", url);
            match fetch_with_proxy(&url).await {
                Ok(response) => Ok(response.data),
                Err(e) => {
                    warn!("Failed: {:?}", e);
                    Err(transient(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )))
                }
            }
        })
    };

    let html = retry(backoff, fetch_operation).await.map_err(|e| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )) as Box<dyn std::error::Error + Send + Sync>
    })?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_latest_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_latest_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
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

        let current_episode = element
            .select(&EPISODE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

        let score = element
            .select(&SCORE_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or("N/A".to_string());

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
            anime_list.push(LatestAnimeItem {
                title,
                slug,
                poster,
                current_episode,
                score,
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
    router.route(ENDPOINT_PATH, get(latest))
}