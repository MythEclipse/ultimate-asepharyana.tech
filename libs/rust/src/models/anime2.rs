use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime2Data {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub description: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub rating: String,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub season: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime2Detail {
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
    pub episodes: Vec<Anime2Episode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime2Episode {
    pub episode_title: String,
    pub episode_url: String,
    pub uploaded_on: String,
}
