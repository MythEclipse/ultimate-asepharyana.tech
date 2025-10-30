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
  current_page: number;
  last_page: number;
  per_page: number;
  total: number;
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
    komikId ? `/api/komik2/detail/${komikId}` : null,
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

// Hook for chapter detail
export function useChapterDetail(chapterId: string, initialData?: ChapterDetail) {
  const { data, error, isLoading, mutate } = useSWR<ChapterDetail>(
    chapterId ? `/api/komik2/chapter/${chapterId}` : null,
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
