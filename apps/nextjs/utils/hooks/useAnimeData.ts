import { useState, useEffect } from 'react';
import { APIURLSERVER } from '../../lib/url';
import { AnimeData, CompleteAnimeData } from '../../types/anime';
import { buildAnimeUrl } from '../url-utils';

type AnimeType = 'anime' | 'anime2';

interface UseAnimeDataOptions {
  type: AnimeType;
  slug?: string;
  query?: string;
  endpoint: 'detail' | 'complete-anime' | 'ongoing-anime' | 'search';
  revalidate?: number;
}

interface UseAnimeDataResult<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
}

export function useAnimeData<T = AnimeData | CompleteAnimeData>(
  options: UseAnimeDataOptions,
): UseAnimeDataResult<T> {
  const { type, slug, query, endpoint, revalidate = 60 } = options;
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      setError(null);
      try {
        let fullUrl: string;

        if (type === 'anime') {
          // Use centralized URL builder for anime
          fullUrl = buildAnimeUrl(endpoint as any, slug, query);
        } else {
          // Fallback to legacy URL building for anime2
          let urlPath = '';
          if (endpoint === 'search' && query) {
            urlPath = `/api/${type}/search?q=${encodeURIComponent(query)}`;
          } else if (slug) {
            urlPath = `/api/${type}/${endpoint}/${slug}`;
          } else {
            throw new Error('Invalid options for fetching data.');
          }
          fullUrl = urlPath.startsWith('/') ? `${APIURLSERVER}${urlPath}` : urlPath;
        }

        const response = await fetch(fullUrl, {
          headers: {
            'Content-Type': 'application/json',
          },
          next: { revalidate },
          signal: AbortSignal.timeout(10000),
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const result = await response.json();
        setData(result);
      } catch (err) {
        console.error(`Failed to fetch ${type} ${endpoint} data:`, err);
        setError(`Failed to load ${type} data`);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [type, slug, query, endpoint, revalidate]);

  return { data, loading, error };
}
