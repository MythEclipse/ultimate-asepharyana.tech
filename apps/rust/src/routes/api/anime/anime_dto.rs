use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeData {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub status: String,
    pub rating: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeDetail {
    pub title: String,
    pub poster: String,
    pub genres: Vec<String>,
    pub status: String,
    pub rating: String,
    pub producer: String,
    pub type_anime: String,
    pub total_episode: String,
    pub duration: String,
    pub release_date: String,
    pub studio: String,
    pub synopsis: String,
    pub episodes: Vec<AnimeEpisode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisode {
    pub episode_title: String,
    pub episode_url: String,
    pub uploaded_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisodeImage {
    pub image_url: String,
}
