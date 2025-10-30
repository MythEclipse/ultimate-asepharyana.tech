import useSWR from 'swr';
import { APIURL } from '../url-utils';
import type {
  AnimeData,
  SearchDetailData,
  CompleteAnimeData,
} from '../../types/anime';

// Types for API responses
export interface HomeData {
  status: string;
  data: {
    ongoing_anime: OngoingAnime[];
    complete_anime: CompleteAnime[];
  };
}

export interface OngoingAnime {
  title: string;
  slug: string;
  poster: string;
  current_episode: string;
  anime_url: string;
}

export interface CompleteAnime {
  title: string;
  slug: string;
  poster: string;
  episode_count: string;
  anime_url: string;
  current_episode: string;
}

export interface AnimeEpisodeData {
  episode: string;
  stream_url: string;
  download_urls: Record<string, DownloadLink[]>;
  has_next_episode: boolean;
  next_episode: EpisodeInfo | null;
  has_previous_episode: boolean;
  previous_episode: EpisodeInfo | null;
}

export interface DownloadLink {
  server: string;
  url: string;
}

export interface EpisodeInfo {
  slug: string;
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

// Hook for home page anime data
export function useAnimeHome(initialData?: HomeData) {
  const { data, error, isLoading, mutate } = useSWR<HomeData>(
    '/api/anime',
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000, // 1 minute
    },
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for anime detail
export function useAnimeDetail(slug: string, initialData?: AnimeData) {
  const { data, error, isLoading, mutate } = useSWR<{ data: AnimeData }>(
    slug ? `/api/anime/detail/${slug}` : null,
    fetcher,
    {
      fallbackData: initialData ? { data: initialData } : undefined,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    },
  );

  return {
    data: data?.data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for anime search
export function useAnimeSearch(query: string, initialData?: SearchDetailData) {
  const { data, error, isLoading, mutate } = useSWR<SearchDetailData>(
    query ? `/api/anime/search?q=${encodeURIComponent(query)}` : null,
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 30000, // 30 seconds for search
    },
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for ongoing anime list
export function useOngoingAnime(page: string, initialData?: CompleteAnimeData) {
  const { data, error, isLoading, mutate } = useSWR<CompleteAnimeData>(
    page ? `/api/anime/ongoing-anime/${page}` : null,
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    },
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for complete anime list
export function useCompleteAnime(
  page: string,
  initialData?: CompleteAnimeData,
) {
  const { data, error, isLoading, mutate } = useSWR<CompleteAnimeData>(
    page ? `/api/anime/complete-anime/${page}` : null,
    fetcher,
    {
      fallbackData: initialData,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    },
  );

  return {
    data,
    error,
    isLoading,
    mutate,
  };
}

// Hook for anime episode (full)
export function useAnimeEpisode(slug: string, initialData?: AnimeEpisodeData) {
  const { data, error, isLoading, mutate } = useSWR<{ data: AnimeEpisodeData }>(
    slug ? `/api/anime/full/${slug}` : null,
    fetcher,
    {
      fallbackData: initialData ? { data: initialData } : undefined,
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
      dedupingInterval: 60000,
    },
  );

  return {
    data: data?.data,
    error,
    isLoading,
    mutate,
  };
}
