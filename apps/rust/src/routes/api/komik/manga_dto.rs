use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct MangaData {
    pub title: String,
    pub poster: String,
    pub chapter: String,
    pub date: String,
    pub score: String,
    #[serde(rename = "type")]
    pub manga_type: String, // Renamed to avoid conflict with Rust keyword
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<u32>,
    pub has_previous_page: bool,
    pub previous_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct MangaDetail {
    pub title: String,
    #[serde(rename = "alternativeTitle")]
    pub alternative_title: String,
    pub score: String,
    pub poster: String,
    pub description: String,
    pub status: String,
    #[serde(rename = "type")]
    pub manga_type: String, // Renamed to avoid conflict with Rust keyword
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    pub author: String,
    #[serde(rename = "totalChapter")]
    pub total_chapter: String,
    #[serde(rename = "updatedOn")]
    pub updated_on: String,
    pub genres: Vec<String>,
    pub chapters: Vec<ChapterData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChapterData {
    pub chapter: String,
    pub date: String,
    pub chapter_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct MangaChapter {
    pub title: String,
    pub next_chapter_id: String,
    pub prev_chapter_id: String,
    pub images: Vec<String>,
    pub list_chapter: String,
}
