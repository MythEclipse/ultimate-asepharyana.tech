use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub name: String,
    pub slug: String,
    pub anime_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodeListItem {
    pub episode: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub status: String,
    pub r#type: String, // 'type' is a reserved keyword, using r#type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeDetailResponseData {
    pub title: String,
    pub alternative_title: String,
    pub poster: String,
    pub r#type: String, // 'type' is a reserved keyword, using r#type
    pub release_date: String,
    pub status: String,
    pub synopsis: String,
    pub studio: String,
    pub genres: Vec<Genre>,
    pub producers: Vec<String>,
    pub recommendations: Vec<Recommendation>,
    pub batch: Vec<EpisodeListItem>,
    pub episode_lists: Vec<EpisodeListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeDetailResponse {
    pub status: String,
    pub data: AnimeDetailResponseData,
}
