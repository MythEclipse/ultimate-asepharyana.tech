import React from 'react';
import AnimePageClient from './AnimePageClient';
import { serverFetch } from '../../utils/serverFetch';

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
    initialData = await serverFetch('/api/anime/');
  } catch (err) {
    console.error('Failed to fetch anime data on server:', err);
    error = 'Failed to load anime data';
  }

  return <AnimePageClient initialData={initialData} initialError={error} />;
}

export default AnimePage;
