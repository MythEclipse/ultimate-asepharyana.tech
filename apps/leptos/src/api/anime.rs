use crate::api::types::{Pagination, ApiResponse};
use crate::api::API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use urlencoding;
use leptos::logging;

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
            logging::log!("Fetched ongoing anime: {} items", data.len());
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
pub struct DownloadLinkItem {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadGroup {
    pub resolution: String,
    pub links: Vec<DownloadLinkItem>,
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
    #[serde(default)]
    pub episode_lists: Vec<EpisodeList>,
    #[serde(default)]
    pub batch: Vec<EpisodeList>,
    #[serde(default)]
    pub recommendations: Vec<Recommendation>,
    #[serde(default)]
    pub downloads: Vec<DownloadGroup>,
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
    // Strict Anime1
    let url = format!("{}/anime/detail/{}", API_BASE_URL, slug);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_res = response.json::<ApiResponse<AnimeDetailData>>().await.map_err(|e| e.to_string())?;
        api_res.data.ok_or_else(|| "No data found".to_string())
    } else {
        Err("Failed to fetch anime detail".to_string())
    }
}

pub async fn fetch_anime2_detail(slug: String) -> Result<AnimeDetailData, String> {
    let client = Client::new();
    // Strict Anime2
    let url = format!("{}/anime2/detail/{}", API_BASE_URL, slug);
    if let Ok(response) = client.get(&url).send().await {
        if response.status().is_success() {
            let api_res = response.json::<ApiResponse<AnimeDetailData>>().await.map_err(|e| e.to_string())?;
            return api_res.data.ok_or_else(|| "No data found".to_string());
        }
    }
    Err("Failed to fetch anime2 detail".to_string())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimeInfo {
    pub slug: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpisodeInfo {
    pub slug: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadLink {
    pub server: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimeFullData {
    pub episode: String,
    pub episode_number: String,
    pub anime: AnimeInfo,
    pub has_next_episode: bool,
    pub has_previous_episode: bool,
    pub stream_url: String,
    #[serde(default)]
    pub download_urls: std::collections::HashMap<String, Vec<DownloadLink>>,
    pub image_url: String,
    pub next_episode: Option<EpisodeInfo>,
    pub previous_episode: Option<EpisodeInfo>,
}

pub async fn fetch_anime_stream(slug: String) -> Result<AnimeFullData, String> {
    let client = Client::new();
    let url = format!("{}/anime/full/{}", API_BASE_URL, slug);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_res = response.json::<ApiResponse<AnimeFullData>>().await.map_err(|e| e.to_string())?;
        api_res.data.ok_or_else(|| "No data found".to_string())
    } else {
        Err("Failed to fetch stream data".to_string())
    }
}

pub async fn fetch_anime2_stream(slug: String) -> Result<AnimeFullData, String> {
    // For Anime2, we prioritize fetching the Detail page to get download links
    // because standard stream endpoints often return empty for Anime2 items.
    
    // 1. Parse slug to get anime_slug and episode info
    // slug format: title-slug-episode-XX
    if let Some((anime_slug, ep_suffix)) = slug.rsplit_once("-episode-") {
        let ep_num = ep_suffix; 
        
        // 2. Fetch Detail
        let detail = fetch_anime2_detail(anime_slug.to_string()).await?;

        // 3. Construct "Fake" AnimeFullData from Detail
        let mut download_urls = std::collections::HashMap::new();
        
        for group in detail.downloads {
             let res_clean = group.resolution.replace("Episode ", "").trim().to_string();
             // Match "01" vs "1"
             let match_found = if let (Ok(a), Ok(b)) = (res_clean.parse::<u32>(), ep_num.parse::<u32>()) {
                a == b
             } else {
                res_clean == ep_num
             };

             if match_found {
                let links: Vec<DownloadLink> = group.links.iter().map(|l| DownloadLink {
                    server: l.name.clone(),
                    url: l.url.clone()
                }).collect();
                download_urls.insert(group.resolution, links);
             }
        }

        // We don't have next/prev/stream_url easily without more logic or scraping,
        // but for now we verify the concept.
        // To construct a valid AnimeFullData, we need to fill the fields.
        
        Ok(AnimeFullData {
            episode: format!("Episode {}", ep_num),
            episode_number: ep_num.to_string(),
            anime: AnimeInfo { slug: anime_slug.to_string() },
            has_next_episode: false, // TODO: logic to check episode_lists if available
            has_previous_episode: false,
            stream_url: "".to_string(),
            download_urls,
            image_url: detail.poster,
            next_episode: None,
            previous_episode: None,
        })
    } else {
        Err("Invalid slug format for Anime2".to_string())
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
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_tamon() {
        let json_str = r#"{"status":"Ok","data":{"title":"Tamon-kun Ima Docchi!? Episode (08) Indo Sub","alternative_title":"Tamon's B-Side, 多聞くん今どっち!?, Which Face Does Tamon Have Now?","poster":"https://i0.wp.com/alqanime.net/wp-content/uploads/2026/01/Tamon-kun-Ima-Docchi-200x300.jpg","poster2":"https://i1.wp.com/alqanime.net/wp-content/uploads/2026/01/Tamon-kun-Ima-Docchi-Sub-Indo.jpg","type":"TV","release_date":"Dirilis: 2026","status":"Status: Ongoing","synopsis":"Utage Kinoshita adalah siswa SMA, penggemar berat Tamon Fukuhara—anggota grup idol populer F/ACE. Suatu hari, secara kebetulan ternyata tempat kerja paruh waktunya adalah… rumahnya Tamon!?","studio":"J.C.Staff","genres":[{"name":"Comedy","slug":"comedy","anime_url":"https://alqanime.net/tag/comedy/"},{"name":"Idols (Male)","slug":"idols-male","anime_url":"https://alqanime.net/tag/idols-male/"},{"name":"Romance","slug":"romance","anime_url":"https://alqanime.net/tag/romance/"},{"name":"Shoujo","slug":"shoujo","anime_url":"https://alqanime.net/tag/shoujo/"}],"producers":[],"recommendations":[],"batch":[],"ova":[],"downloads":[{"resolution":"Episode 08","links":[{"name":"360p - AceFile","url":"https://acefile.co/f/111233999/alqanime_tamonb_08_360p-mp4"},{"name":"360p - GoFIle","url":"https://gofile.io/d/EngIyY"},{"name":"360p - ReShare","url":"https://reshare.pm/d/102400/alqanimetamonb08360pmp4"},{"name":"480p - AceFile","url":"https://acefile.co/f/111234000/alqanime_tamonb_08_480p-mp4"},{"name":"480p - GoFile","url":"https://gofile.io/d/1m0IsG"},{"name":"480p - ReShare","url":"https://reshare.pm/d/102403/alqanimetamonb08480pmp4"},{"name":"720p - AceFile","url":"https://acefile.co/f/111234002/alqanime_tamonb_08_720p-mp4"},{"name":"720p - GoFile","url":"https://gofile.io/d/kYk1j5"},{"name":"720p - ReShare","url":"https://reshare.pm/d/102401/alqanimetamonb08720pmp4"},{"name":"1080p - AceFile","url":"https://acefile.co/f/111234005/alqanime_tamonb_08_1080p-mp4"},{"name":"1080p - GoFile","url":"https://gofile.io/d/lpnWHk"},{"name":"1080p - ReShare","url":"https://reshare.pm/d/102402/alqanimetamonb081080pmp4"}]}]}}"#;

        let res: Result<ApiResponse<AnimeDetailData>, _> = from_str(json_str);
        if let Err(e) = res {
            panic!("Deserialization failed: {}", e);
        } else {
             println!("Deserialization success!");
        }
    }
}
