import useSWR from 'swr';
import { APIURL } from '../url-utils';

// Types for Komik API responses
export interface Komik {
  title: string;
  poster: string;
  chapter: string;
  date: string;
  reader_count: string;
  type: string;
  slug: string;
}

export interface KomikData {
  data: Komik[];
}

export interface KomikPaginationData {
  data: Komik[];
  pagination: {
    current_page: number;
    last_visible_page: number;
    has_next_page: boolean;
    next_page: number | null;
    has_previous_page: boolean;
    previous_page: number | null;
  };
}

export interface KomikDetail {
  title: string;
  alternative_title?: string;
  poster: string;
  type: string;
  status: string;
  author: string;
  synopsis: string;
  genres: string[];
  chapters: Chapter[];
}

export interface Chapter {
  title: string;
  slug: string;
  date: string;
}

export interface ChapterDetail {
  title: string;
  images: string[];
  has_next: boolean;
  has_prev: boolean;
  next_chapter?: string;
  prev_chapter?: string;
}

export interface KomikSearchData {
  data: Komik[];
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

// API response interface for komik detail
interface KomikDetailApiResponse {
  status: boolean;
  data: {
    title: string;
    alternative_title?: string;
    poster: string;
    type: string;
    status: string;
    author: string;
    description: string;
    genres: string[];
    chapters: {
      chapter: string;
      chapter_id: string;
      date: string;
    }[];
  };
}

// Fetcher for komik detail with data transformation
const detailFetcher = async (url: string): Promise<KomikDetail> => {
  const fullUrl = url.startsWith('/') ? `${APIURL}${url}` : url;
  const response = await fetch(fullUrl, {
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const result: KomikDetailApiResponse = await response.json();

  // Transform API response to match KomikDetail interface
  if (result.status && result.data) {
    return {
      title: result.data.title,
      alternative_title: result.data.alternative_title,
      poster: result.data.poster,
      type: result.data.type,
      status: result.data.status,
      author: result.data.author,
      synopsis: result.data.description, // Map 'description' to 'synopsis'
      genres: result.data.genres,
      chapters: result.data.chapters.map((ch) => ({
        title: `Chapter ${ch.chapter}`, // Transform 'chapter' to 'title'
        slug: ch.chapter_id, // Map 'chapter_id' to 'slug'
        date: ch.date,
      })),
    };
  }

  throw new Error('Invalid response structure');
};

// Hook for manga list
export function useMangaList(page = 1, order = 'update', initialData?: KomikData) {
  const { data, error, isLoading, mutate } = useSWR<KomikData>(
    `/api/komik2/manga?page=${page}&order=${order}`,
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

// Hook for manhua list
export function useManhuaList(page = 1, order = 'update', initialData?: KomikData) {
  const { data, error, isLoading, mutate } = useSWR<KomikData>(
    `/api/komik2/manhua?page=${page}&order=${order}`,
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

// Hook for manhwa list
export function useManhwaList(page = 1, order = 'update', initialData?: KomikData) {
  const { data, error, isLoading, mutate } = useSWR<KomikData>(
    `/api/komik2/manhwa?page=${page}&order=${order}`,
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

// Hook for manga pagination
export function useMangaPage(pageNumber: number, initialData?: KomikPaginationData) {
  const { data, error, isLoading, mutate } = useSWR<KomikPaginationData>(
    pageNumber ? `/api/komik2/manga?page=${pageNumber}&order=update` : null,
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

// Hook for manhua pagination
export function useManhuaPage(pageNumber: number, initialData?: KomikPaginationData) {
  const { data, error, isLoading, mutate } = useSWR<KomikPaginationData>(
    pageNumber ? `/api/komik2/manhua?page=${pageNumber}&order=update` : null,
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

// Hook for manhwa pagination
export function useManhwaPage(pageNumber: number, initialData?: KomikPaginationData) {
  const { data, error, isLoading, mutate } = useSWR<KomikPaginationData>(
    pageNumber ? `/api/komik2/manhwa?page=${pageNumber}&order=update` : null,
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

// Hook for komik detail
export function useKomikDetail(komikId: string, initialData?: KomikDetail) {
  const { data, error, isLoading, mutate } = useSWR<KomikDetail>(
    komikId ? `/api/komik2/detail?komik_id=${komikId}` : null,
    detailFetcher, // Use detailFetcher for automatic transformation
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

// Hook for chapter detail
export function useChapterDetail(chapterId: string, initialData?: ChapterDetail) {
  const { data, error, isLoading, mutate } = useSWR<ChapterDetail>(
    chapterId ? `/api/komik2/chapter?chapter_url=${chapterId}` : null,
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

// Hook for komik search
export function useKomikSearch(query: string, initialData?: KomikSearchData) {
  const { data, error, isLoading, mutate } = useSWR<KomikSearchData>(
    query ? `/api/komik2/search?q=${encodeURIComponent(query)}` : null,
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
