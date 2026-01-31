use crate::helpers::parse_html;
use crate::helpers::scraping::{attr, attr_from, attr_from_or, extract_slug, selector, text, text_from_or};
use scraper::{Html, Selector};
use crate::models::anime2::*;

// ============================================================================
// SELECTORS
// ============================================================================

/// Common selectors used across anime parsing
pub struct AnimeSelectors {
    pub item: Selector,
    pub title: Selector,
    pub link: Selector,
    pub img: Selector,
    pub episode: Selector,
    pub score: Selector,
    pub status: Selector,
    pub genre: Selector,
    pub rating: Selector,
    pub type_sel: Selector,
    pub season: Selector,
    pub desc: Selector,
}

impl AnimeSelectors {
    pub fn new() -> Self {
        Self {
            item: selector("article.bs").unwrap(),
            title: selector(".tt h2").unwrap(),
            link: selector("a").unwrap(),
            img: selector("img").unwrap(),
            episode: selector(".epx").unwrap(),
            score: selector(".numscore").unwrap(),
            status: selector(".status").unwrap(),
            genre: selector(".genres a").unwrap(),
            rating: selector(".score").unwrap(),
            type_sel: selector(".typez").unwrap(),
            season: selector(".season").unwrap(),
            desc: selector(".data .typez").unwrap(),
        }
    }
}

impl Default for AnimeSelectors {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Extract poster URL from an element, checking both src and data-src attributes
pub fn extract_poster(element: &scraper::ElementRef, img_selector: &Selector) -> String {
    element
        .select(img_selector)
        .next()
        .and_then(|e| attr(&e, "src").or(attr(&e, "data-src")))
        .unwrap_or_default()
}

// ============================================================================
// ANIME PARSERS
// ============================================================================

/// Parse ongoing anime items from HTML
pub fn parse_ongoing_anime(
    html: &str,
) -> Result<Vec<OngoingAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let selectors = AnimeSelectors::new();
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let href = attr_from_or(&element, &selectors.link, "href", "");
        let slug = extract_slug(&href);
        let poster = extract_poster(&element, &selectors.img);
        let current_episode = text_from_or(&element, &selectors.episode, "N/A");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");

        items.push(OngoingAnimeItem {
            title,
            slug,
            poster,
            current_episode,
            anime_url,
        });
    }

    Ok(items)
}

/// Parse ongoing anime items with score from HTML
pub fn parse_ongoing_anime_with_score(
    html: &str,
) -> Result<Vec<OngoingAnimeItemWithScore>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let selectors = AnimeSelectors::new();
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let poster = extract_poster(&element, &selectors.img);
        let score = text_from_or(&element, &selectors.score, "N/A");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");
        let slug = extract_slug(&anime_url);

        items.push(OngoingAnimeItemWithScore {
            title,
            slug,
            poster,
            score,
            anime_url,
        });
    }

    Ok(items)
}

/// Parse complete anime items from HTML
pub fn parse_complete_anime(
    html: &str,
) -> Result<Vec<CompleteAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let selectors = AnimeSelectors::new();
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let href = attr_from_or(&element, &selectors.link, "href", "");
        let slug = extract_slug(&href);
        let poster = extract_poster(&element, &selectors.img);
        let episode_count = text_from_or(&element, &selectors.episode, "N/A");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");

        items.push(CompleteAnimeItem {
            title,
            slug,
            poster,
            episode_count,
            anime_url,
        });
    }

    Ok(items)
}

/// Parse latest anime items from HTML
pub fn parse_latest_anime(
    html: &str,
) -> Result<Vec<LatestAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let selectors = AnimeSelectors::new();
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let poster = extract_poster(&element, &selectors.img);
        let current_episode = text_from_or(&element, &selectors.episode, "N/A");
        let score = text_from_or(&element, &selectors.score, "N/A");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");
        let slug = extract_slug(&anime_url);

        items.push(LatestAnimeItem {
            title,
            slug,
            poster,
            current_episode,
            score,
            anime_url,
        });
    }

    Ok(items)
}

/// Parse search results from HTML
pub fn parse_search_anime(
    html: &str,
) -> Result<Vec<SearchAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let selectors = AnimeSelectors::new();
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let href = attr_from(&element, &selectors.link, "href").unwrap_or_default();
        let slug = extract_slug(&href);
        let poster = extract_poster(&element, &selectors.img);
        let description = text_from_or(&element, &selectors.desc, "");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");
        let genres = element.select(&selectors.genre).map(|e| text(&e)).collect();
        let rating = text_from_or(&element, &selectors.rating, "");
        let r#type = text_from_or(&element, &selectors.type_sel, "");
        let season = text_from_or(&element, &selectors.season, "");

        items.push(SearchAnimeItem {
            title,
            slug,
            poster,
            description,
            anime_url,
            genres,
            rating,
            r#type,
            season,
        });
    }

    Ok(items)
}

/// Parse genre-filtered anime items from HTML
pub fn parse_genre_anime(
    html: &str,
) -> Result<Vec<GenreAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let selectors = AnimeSelectors::new();
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let poster = extract_poster(&element, &selectors.img);
        let score = text_from_or(&element, &selectors.score, "N/A");
        let status = text_from_or(&element, &selectors.status, "Unknown");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");
        let slug = extract_slug(&anime_url);

        items.push(GenreAnimeItem {
            title,
            slug,
            poster,
            score,
            status,
            anime_url,
        });
    }

    Ok(items)
}

// ============================================================================
// PAGINATION PARSERS
// ============================================================================

/// Parse pagination from HTML document
pub fn parse_pagination(document: &Html, current_page: u32) -> Pagination {
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagination .next").unwrap();

    let last_visible_page = document
        .select(&pagination_selector)
        .next_back()
        .and_then(|e| text(&e).trim().parse::<u32>().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&next_selector).next().is_some();
    let next_page = if has_next_page {
        Some(current_page + 1)
    } else {
        None
    };

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some(current_page - 1)
    } else {
        None
    };

    Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    }
}

/// Parse pagination with string-based page numbers (for search results)
pub fn parse_pagination_with_string(document: &Html, current_page: u32) -> PaginationWithStringPages {
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").unwrap();
    let next_selector = selector(".pagination .next").unwrap();

    let last_visible_page = document
        .select(&pagination_selector)
        .last()
        .and_then(|e| text(&e).trim().parse::<u32>().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&next_selector).next().is_some();

    let next_page = if has_next_page {
        document
            .select(&next_selector)
            .next()
            .and_then(|e| attr(&e, "href"))
            .and_then(|href| href.split("/page/").nth(1).map(|s| s.to_string()))
            .and_then(|s| s.split('/').next().map(|s| s.to_string()))
    } else {
        None
    };

    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page {
        Some((current_page - 1).to_string())
    } else {
        None
    };

    PaginationWithStringPages {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    }
}
