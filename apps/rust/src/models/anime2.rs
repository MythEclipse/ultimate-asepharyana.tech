use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ============================================================================
// PAGINATION MODELS
// ============================================================================

/// Common pagination structure used across all anime2 endpoints
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Pagination {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<u32>,
    pub has_previous_page: bool,
    pub previous_page: Option<u32>,
}

impl Pagination {
    /// Create pagination with string-based next/previous pages
    pub fn with_string_pages(
        current_page: u32,
        last_visible_page: u32,
        has_next_page: bool,
        next_page: Option<String>,
        has_previous_page: bool,
        previous_page: Option<String>,
    ) -> PaginationWithStringPages {
        PaginationWithStringPages {
            current_page,
            last_visible_page,
            has_next_page,
            next_page,
            has_previous_page,
            previous_page,
        }
    }
}

/// Pagination variant with string-based page numbers (used in search endpoint)
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PaginationWithStringPages {
    pub current_page: u32,
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub next_page: Option<String>,
    pub has_previous_page: bool,
    pub previous_page: Option<String>,
}

// ============================================================================
// ANIME ITEM MODELS
// ============================================================================

/// Anime item for ongoing anime listings
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub anime_url: String,
}

/// Anime item for ongoing anime with score (used in paginated ongoing lists)
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct OngoingAnimeItemWithScore {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub score: String,
    pub anime_url: String,
}

/// Anime item for complete anime listings
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompleteAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

/// Anime item for latest anime listings with episode and score
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LatestAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub score: String,
    pub anime_url: String,
}

/// Anime item for search results with full metadata
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SearchAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub description: String,
    pub anime_url: String,
    pub genres: Vec<String>,
    pub rating: String,
    pub r#type: String,
    pub season: String,
}

/// Anime item for genre filtering
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GenreAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub score: String,
    pub status: String,
    pub anime_url: String,
}

// ============================================================================
// TRAITS
// ============================================================================

/// Trait for extracting poster URLs from anime items
pub trait HasPoster {
    fn poster(&self) -> &str;
    fn set_poster(&mut self, url: String);
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

impl HasPoster for OngoingAnimeItem {
    fn poster(&self) -> &str {
        &self.poster
    }
    fn set_poster(&mut self, url: String) {
        self.poster = url;
    }
}

impl HasPoster for OngoingAnimeItemWithScore {
    fn poster(&self) -> &str {
        &self.poster
    }
    fn set_poster(&mut self, url: String) {
        self.poster = url;
    }
}

impl HasPoster for CompleteAnimeItem {
    fn poster(&self) -> &str {
        &self.poster
    }
    fn set_poster(&mut self, url: String) {
        self.poster = url;
    }
}

impl HasPoster for LatestAnimeItem {
    fn poster(&self) -> &str {
        &self.poster
    }
    fn set_poster(&mut self, url: String) {
        self.poster = url;
    }
}

impl HasPoster for SearchAnimeItem {
    fn poster(&self) -> &str {
        &self.poster
    }
    fn set_poster(&mut self, url: String) {
        self.poster = url;
    }
}

impl HasPoster for GenreAnimeItem {
    fn poster(&self) -> &str {
        &self.poster
    }
    fn set_poster(&mut self, url: String) {
        self.poster = url;
    }
}
