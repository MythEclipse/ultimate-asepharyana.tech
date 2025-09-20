export interface Genre {
  name: string;
  slug: string;
}

export interface Episode {
  episode: string;
  slug: string;
}

export interface DownloadLink {
  name: string;
  url: string;
}

export interface DownloadResolution {
  resolution: string;
  links: DownloadLink[];
}

export interface Recommendation {
  slug: string;
  title: string;
  poster: string;
  type?: string; // Optional for anime, required for anime2
}

export interface AnimeData {
  title: string;
  alternative_title?: string;
  poster: string;
  type: string;
  status: string;
  release_date: string;
  studio: string;
  synopsis: string;
  genres: Genre[];
  producers?: string[]; // Specific to anime
  episode_lists?: Episode[]; // Specific to anime
  batch?: DownloadResolution[]; // Specific to anime2
  downloads?: DownloadResolution[]; // Specific to anime2
  recommendations: Recommendation[];
}

export interface SearchDetailData {
  status: string;
  data: Anime[];
}

export interface CompleteAnimeData {
  status: string;
  data: Anime[];
  pagination: Pagination;
}

export interface Anime {
  title: string;
  slug: string;
  poster: string;
  episode?: string;
  anime_url?: string;
  rating?: string;
  status?: string; // Added for search results
  last_release_date?: string;
  current_episode?: string;
  release_day?: string;
  newest_release_date?: string;
}

export interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}
