import React from 'react';
import AnimePageClient from './AnimePageClient';
import { APIURLSERVER } from '../../lib/url';

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
  let initialData: HomeData | null = null;
  let error: string | null = null;

  try {
    const url = '/api/anime';
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    console.log('Fetching data from URL:', fullUrl); // Debug log
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    initialData = await response.json();
  } catch (err) {
    console.error('Failed to fetch anime data on server:', err);
    error = 'Failed to load anime data';
  }

  return <AnimePageClient data={initialData} error={error} />;
}

export default AnimePage;
