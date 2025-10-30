import useSWR from 'swr';
import { APIURL } from '../url-utils';

// Types for API responses
export interface HomeData2 {
  status: string;
  data: {
    ongoing_anime: OngoingAnime2[];
    complete_anime: CompleteAnime2[];
  };
}

export interface OngoingAnime2 {
  title: string;
  slug: string;
  poster: string;
  current_episode: string;
  anime_url: string;
}

export interface CompleteAnime2 {
  title: string;
  slug: string;
  poster: string;
  episode_count: string;
  anime_url: string;
}

export interface Anime2Data {
  title: string;
  alternative_title?: string;
  poster: string;
  type: string;
  status: string;
  release_date: string;
  studio: string;
  synopsis: string;
  genres: Genre2[];
  batch?: DownloadResolution2[];
  downloads?: DownloadResolution2[];
  recommendations: Recommendation2[];
}

export interface Genre2 {
  name: string;
  slug: string;
}

export interface DownloadResolution2 {
  resolution: string;
  links: DownloadLink2[];
}

export interface DownloadLink2 {
  name: string;
  url: string;
}

export interface Recommendation2 {
  slug: string;
  title: string;
  poster: string;
  type: string;
}

export interface SearchDetailData2 {
  status: string;
  data: Anime2[];
}

export interface CompleteAnimeData2 {
  status: string;
  data: Anime2[];
  pagination: Pagination2;
}

export interface Anime2 {
  title: string;
  slug: string;
  poster: string;
  episode?: string;
  anime_url?: string;
  rating?: string;
  status?: string;
  last_release_date?: string;
  current_episode?: string;
  release_day?: string;
  newest_release_date?: string;
}

export interface Pagination2 {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}

// Fetcher function for SWR
const fetcher = async (url: string) => {
  const fullUrl = url.startsWith('/') ? `${APIURL}${url}` : url;
  const response = await fetch(fullUrl, {
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return response.json();
};

// Hook for home page anime2 data
export function useAnime2Home(initialData?: HomeData2) {
  const { data, error, isLoading, mutate } = useSWR<HomeData2>(
    '/api/anime2',
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000, // 1 minute
    }
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for anime2 detail
export function useAnime2Detail(slug: string, initialData?: Anime2Data) {
  const { data, error, isLoading, mutate } = useSWR<{ data: Anime2Data }>(
    slug ? `/api/anime2/detail/${slug}` : null,
    fetcher,
    {
      fallbackData: initialData ? { data: initialData } : undefined,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    }
  );

  return {
    data: data?.data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for anime2 search
export function useAnime2Search(query: string, initialData?: SearchDetailData2) {
  const { data, error, isLoading, mutate } = useSWR<SearchDetailData2>(
    query ? `/api/anime2/search?q=${encodeURIComponent(query)}` : null,
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 30000, // 30 seconds for search
    }
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for ongoing anime2 list
export function useOngoingAnime2(page: string, initialData?: CompleteAnimeData2) {
  const { data, error, isLoading, mutate } = useSWR<CompleteAnimeData2>(
    page ? `/api/anime2/ongoing-anime/${page}` : null,
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    }
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for complete anime2 list
export function useCompleteAnime2(page: string, initialData?: CompleteAnimeData2) {
  const { data, error, isLoading, mutate } = useSWR<CompleteAnimeData2>(
    page ? `/api/anime2/complete-anime/${page}` : null,
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    }
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}
