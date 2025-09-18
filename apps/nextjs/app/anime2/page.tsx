import React from 'react';
import Anime2PageClient from './Anime2PageClient';
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
}

async function AnimePage() {
  let initialData: HomeData | null = null;
  let error: string | null = null;

  try {
    initialData = await serverFetch('/api/anime2/');
  } catch (err) {
    console.error('Failed to fetch anime2 data on server:', err);
    error = 'Failed to load anime data';
  }

  return <Anime2PageClient initialData={initialData} initialError={error} />;
}

export default AnimePage;
