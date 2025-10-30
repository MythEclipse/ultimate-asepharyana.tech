import React from 'react';
import Anime2PageClient from './Anime2PageClient';
import { fetchWithFallback } from '../../utils/url-utils';
import type { HomeData2 } from '../../utils/hooks/useAnime2';

export const revalidate = 60;

async function AnimePage() {
  let initialData: HomeData2 = {
    status: 'error',
    data: {
      ongoing_anime: [],
      complete_anime: [],
    },
  };
  let error: string | null = null;

  if (process.env.NODE_ENV === 'production') {
    try {
      const response = await fetchWithFallback('/api/anime2', {
        revalidate: 60,
        signal: AbortSignal.timeout(10000),
      });

      initialData = await response.json();
    } catch (err) {
      console.error('Failed to fetch anime2 data:', err);
      error = 'Failed to load anime data';
    }
  } else {
    console.warn('Skipping API fetch in development mode during build.');
  }

  return <Anime2PageClient initialData={initialData} initialError={error} />;
}

export default AnimePage;
