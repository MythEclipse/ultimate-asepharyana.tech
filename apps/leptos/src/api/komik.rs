use crate::api::types::Pagination;
use crate::api::API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MangaItem {
    pub title: String,
    pub poster: String,
    pub chapter: String,
    pub date: String,
    pub reader_count: Option<String>,
    pub score: Option<String>,
    pub r#type: String,
    pub slug: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MangaResponse {
    pub data: Vec<MangaItem>,
    pub pagination: Pagination,
}

async fn fetch_komik_type(type_: &str, page: u32) -> Result<MangaResponse, String> {
    let client = Client::new();
    // Komik endpoint: /api/komik/{type}?page={page}
    let url = format!("{}/komik/{}?page={}", API_BASE_URL, type_, page);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        response.json::<MangaResponse>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch {}", type_))
    }
}

pub async fn fetch_manga(page: u32) -> Result<MangaResponse, String> {
    fetch_komik_type("manga", page).await
}

pub async fn fetch_manhwa(page: u32) -> Result<MangaResponse, String> {
    fetch_komik_type("manhwa", page).await
}

pub async fn fetch_manhua(page: u32) -> Result<MangaResponse, String> {
    fetch_komik_type("manhua", page).await
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter: String,
    pub date: String,
    pub chapter_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailData {
    pub title: String,
    pub poster: String,
    pub description: String,
    pub status: String,
    pub r#type: String,
    pub release_date: String,
    pub author: String,
    pub total_chapter: String,
    pub updated_on: String,
    pub genres: Vec<String>,
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KomikDetailResponse {
    pub status: bool,
    pub data: DetailData,
}

pub async fn fetch_komik_detail(komik_id: String) -> Result<DetailData, String> {
    let client = Client::new();
    let url = format!("{}/komik/detail?komik_id={}", API_BASE_URL, komik_id);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let res = response.json::<KomikDetailResponse>().await.map_err(|e| e.to_string())?;
        if res.status {
            Ok(res.data)
        } else {
            Err("Backend reported failure".to_string())
        }
    } else {
        Err("Failed to fetch komik detail".to_string())
    }
}

pub async fn search_komik(query: String, page: u32) -> Result<MangaResponse, String> {
    let client = Client::new();
    let url = format!(
        "{}/komik/search?query={}&page={}", 
        API_BASE_URL, 
        urlencoding::encode(&query),
        page
    );

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        response.json::<MangaResponse>().await.map_err(|e| e.to_string())
    } else {
        Err("Search failed".to_string())
    }
}
