use crate::api::types::{Pagination, ApiResponse};
use crate::api::API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use urlencoding;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OngoingAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub score: String,
    pub anime_url: String,
    // Helper for frontend compatibility if needed
    // pub current_episode: Option<String>, 
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompleteAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OngoingAnimeResponse {
    pub pagination: Pagination,
    pub status: String,
}

pub async fn fetch_ongoing_anime(page: u32) -> Result<(Vec<OngoingAnimeItem>, Pagination), String> {
    let client = Client::new();
    // Using slug as page number
    let url = format!("{}/anime2/ongoing-anime/{}", API_BASE_URL, page);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_response = response.json::<ApiResponse<Vec<OngoingAnimeItem>>>().await.map_err(|e| e.to_string())?;
        if let Some(data) = api_response.data {
             let pagination = if let Some(meta) = api_response.meta {
                serde_json::from_value::<OngoingAnimeResponse>(meta)
                    .map(|r| r.pagination)
                    .unwrap_or_else(|_| Pagination {
                        current_page: 1, last_visible_page: 1, has_next_page: false, next_page: None, has_previous_page: false, previous_page: None
                    })
            } else {
                 Pagination {
                    current_page: 1, last_visible_page: 1, has_next_page: false, next_page: None, has_previous_page: false, previous_page: None
                }
            };
            Ok((data, pagination))
        } else {
            Err("No data returned".to_string())
        }
    } else {
        Err("Failed to fetch ongoing anime".to_string())
    }
}

pub async fn fetch_complete_anime(page: u32) -> Result<(Vec<CompleteAnimeItem>, Pagination), String> {
    let client = Client::new();
    let url = format!("{}/anime2/complete-anime/{}", API_BASE_URL, page);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_response = response.json::<ApiResponse<Vec<CompleteAnimeItem>>>().await.map_err(|e| e.to_string())?;
         if let Some(data) = api_response.data {
             let pagination = if let Some(meta) = api_response.meta {
                serde_json::from_value::<OngoingAnimeResponse>(meta)
                    .map(|r| r.pagination)
                    .unwrap_or_else(|_| Pagination {
                        current_page: 1, last_visible_page: 1, has_next_page: false, next_page: None, has_previous_page: false, previous_page: None
                    })
            } else {
                 Pagination {
                    current_page: 1, last_visible_page: 1, has_next_page: false, next_page: None, has_previous_page: false, previous_page: None
                }
            };
            Ok((data, pagination))
        } else {
             Err("No data returned".to_string())
        }
    } else {
        Err("Failed to fetch complete anime".to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub anime_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpisodeList {
    pub episode: String,
    pub slug: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recommendation {
    pub title: String,
    pub slug: String,
    pub poster: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimeDetailData {
    pub title: String,
    pub alternative_title: String,
    pub poster: String,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub release_date: String,
    pub studio: String,
    pub genres: Vec<Genre>,
    pub synopsis: String,
    pub episode_lists: Vec<EpisodeList>,
    pub batch: Vec<EpisodeList>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub status: String,
    pub rating: String,
}

pub async fn fetch_anime_detail(slug: String) -> Result<AnimeDetailData, String> {
    let client = Client::new();
    let url = format!("{}/anime/detail/{}", API_BASE_URL, slug);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_res = response.json::<ApiResponse<AnimeDetailData>>().await.map_err(|e| e.to_string())?;
        api_res.data.ok_or_else(|| "No data found".to_string())
    } else {
        Err("Failed to fetch anime detail".to_string())
    }
}

pub async fn search_anime(query: String) -> Result<Vec<SearchAnimeItem>, String> {
    let client = Client::new();
    let url = format!("{}/anime/search?q={}", API_BASE_URL, urlencoding::encode(&query));

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_res = response.json::<ApiResponse<Vec<SearchAnimeItem>>>().await.map_err(|e| e.to_string())?;
        api_res.data.ok_or_else(|| "No results".to_string())
    } else {
        Err("Search failed".to_string())
    }
}
