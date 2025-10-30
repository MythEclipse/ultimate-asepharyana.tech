import React from 'react';
import AnimePageClient from './AnimePageClient';
import { fetchWithFallback } from '../../utils/url-utils';

export const revalidate = 60;

interface HomeData {
  status: string;
  data: {
    ongoing_anime: OngoingAnime[];
    complete_anime: CompleteAnime[];
  };
}

interface OngoingAnime {
  title: string;
  slug: string;
  poster: string;
  current_episode: string;
  anime_url: string;
}

interface CompleteAnime {
  title: string;
  slug: string;
  poster: string;
  episode_count: string;
  anime_url: string;
  current_episode: string;
}

async function AnimePage() {
  let initialData: HomeData = {
    status: 'error',
    data: {
      ongoing_anime: [],
      complete_anime: [],
    },
  };
  let error: string | null = null;

  if (process.env.NODE_ENV === 'production') {
    try {
      const response = await fetchWithFallback('/api/anime', {
        revalidate,
        signal: AbortSignal.timeout(10000),
      });

      initialData = await response.json();
    } catch (err) {
      console.error('Failed to fetch anime data:', err);
      error = 'Failed to load anime data';
    }
  } else {
    console.warn('Skipping API fetch in development mode during build.');
  }

  return <AnimePageClient initialData={initialData} initialError={error} />;
}

export default AnimePage;
