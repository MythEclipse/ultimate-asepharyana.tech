use crate::core::types::ApiResponse;
use crate::helpers::{parse_html, Cache, fetch_html_with_retry, text_from_or, attr_from_or, selector, extract_slug, attr_from};

use crate::routes::AppState;
use crate::utils::error::AppError;
use crate::scraping::urls::get_otakudesu_url;
use axum::extract::State;
use axum::{response::IntoResponse, Json, Router};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompleteAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeData {
    pub ongoing_anime: Vec<OngoingAnimeItem>,
    pub complete_anime: Vec<CompleteAnimeItem>,
}

pub type AnimeDataResponse = ApiResponse<AnimeData>;
pub type EmptyResponse = ApiResponse<()>;

use crate::helpers::cache_ttl::CACHE_TTL_VERY_SHORT;
const CACHE_TTL: u64 = CACHE_TTL_VERY_SHORT; // 5 minutes

#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "anime",
    operation_id = "anime_index",
    responses(
        (status = 200, description = "Handles GET requests for the anime endpoint.", body = AnimeDataResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn anime(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let start_time = std::time::Instant::now();
    info!("Handling request for anime index");

    let cache = Cache::new(&app_state.redis_pool);

    // Clean caching with get_or_set pattern
    let response = cache
        .get_or_set("anime:index", CACHE_TTL, || async {
            let mut data = fetch_anime_data()
                .await
                .map_err(|e| format!("Fetch error: {}", e))?;

            // Convert all poster URLs to CDN URLs
            // Fire-and-forget background caching for posters to ensure max API speed
            let db = app_state.db.clone();
            let redis = app_state.redis_pool.clone();

            let ongoing_posters: Vec<String> = data
                .ongoing_anime
                .iter()
                .map(|i| i.poster.clone())
                .collect();
            let complete_posters: Vec<String> = data
                .complete_anime
                .iter()
                .map(|i| i.poster.clone())
                .collect();

            let ongoing_len = ongoing_posters.len();

            let all_posters = [ongoing_posters, complete_posters].concat();
            let cached_posters = crate::helpers::image_cache::cache_image_urls_batch_lazy(
                db.clone(),
                &redis,
                all_posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            // Update ongoing anime posters
            for (i, item) in data.ongoing_anime.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(i) {
                    item.poster = url.clone();
                }
            }

            // Update complete anime posters
            for (i, item) in data.complete_anime.iter_mut().enumerate() {
                if let Some(url) = cached_posters.get(ongoing_len + i) {
                    item.poster = url.clone();
                }
            }

            Ok(ApiResponse::success(data))
        })
        .await
        .map_err(|e| AppError::Other(e.to_string()))?;

    info!("Anime index completed in {:?}", start_time.elapsed());
    Ok(Json(response))
}

async fn fetch_anime_data() -> Result<AnimeData, Box<dyn std::error::Error + Send + Sync>> {
    let ongoing_url = format!("{}/ongoing-anime/", get_otakudesu_url());
    let complete_url = format!("{}/complete-anime/", get_otakudesu_url());

    let (ongoing_html, complete_html) = tokio::join!(
        fetch_html_with_retry(&ongoing_url),
        fetch_html_with_retry(&complete_url)
    );

    let ongoing_html = ongoing_html?;
    let complete_html = complete_html?;

    let ongoing_anime =
        tokio::task::spawn_blocking(move || parse_ongoing_anime(&ongoing_html)).await??;
    let complete_anime =
        tokio::task::spawn_blocking(move || parse_complete_anime(&complete_html)).await??;

    Ok(AnimeData {
        ongoing_anime,
        complete_anime,
    })
}



fn parse_ongoing_anime(
    html: &str,
) -> Result<Vec<OngoingAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut ongoing_anime = Vec::new();

    let venz_selector = selector(".venz ul li").unwrap();
    let title_selector = selector(".thumbz h2.jdlflm").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epz").unwrap();

    for element in document.select(&venz_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let href = attr_from(&element, &link_selector, "href").unwrap_or_default();
        let slug = extract_slug(&href);

        let poster = attr_from_or(&element, &img_selector, "src", "");

        let current_episode = text_from_or(&element, &episode_selector, "N/A");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        if !title.is_empty() {
            ongoing_anime.push(OngoingAnimeItem {
                title,
                slug,
                poster,
                current_episode,
                anime_url,
            });
        }
    }
    Ok(ongoing_anime)
}

fn parse_complete_anime(
    html: &str,
) -> Result<Vec<CompleteAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut complete_anime = Vec::new();

    let venz_selector = selector(".venz ul li").unwrap();
    let title_selector = selector(".thumbz h2.jdlflm").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epz").unwrap();
    
    for element in document.select(&venz_selector) {
        let title = text_from_or(&element, &title_selector, "");

        let href = attr_from(&element, &link_selector, "href").unwrap_or_default();
        let slug = extract_slug(&href);

        let poster = attr_from_or(&element, &img_selector, "src", "");

        let episode_count = text_from_or(&element, &episode_selector, "N/A");

        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        if !title.is_empty() {
            complete_anime.push(CompleteAnimeItem {
                title,
                slug,
                poster,
                episode_count,
                anime_url,
            });
        }
    }
    Ok(complete_anime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ongoing_anime() {
        let html = r#"
        <div class="venz">
            <ul>
                <li>
                    <div class="thumbz">
                        <h2 class="jdlflm">One Piece</h2>
                    </div>
                    <div class="epz">Episode 1000</div>
                    <a href="https://otakudesu.cloud/anime/one-piece-slug/"></a>
                    <img src="https://example.com/op.jpg" />
                </li>
            </ul>
        </div>
        "#;

        let result = parse_ongoing_anime(html).expect("Failed to parse ongoing anime");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "One Piece");
        assert_eq!(result[0].slug, "one-piece-slug");
        assert_eq!(result[0].current_episode, "Episode 1000");
        assert_eq!(result[0].poster, "https://example.com/op.jpg");
    }

    #[test]
    fn test_parse_complete_anime() {
        let html = r#"
        <div class="venz">
            <ul>
                <li>
                    <div class="thumbz">
                        <h2 class="jdlflm">Naruto</h2>
                    </div>
                    <div class="epz">500 Episodes</div>
                    <a href="https://otakudesu.cloud/anime/naruto-slug/"></a>
                    <img src="https://example.com/naruto.jpg" />
                </li>
            </ul>
        </div>
        "#;

        let result = parse_complete_anime(html).expect("Failed to parse complete anime");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "Naruto");
        assert_eq!(result[0].slug, "naruto-slug");
        assert_eq!(result[0].episode_count, "500 Episodes");
        assert_eq!(result[0].poster, "https://example.com/naruto.jpg");
    }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}