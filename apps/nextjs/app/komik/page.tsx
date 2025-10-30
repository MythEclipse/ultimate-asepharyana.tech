import React from 'react';
import KomikPageClient from './KomikPageClient';
import { fetchKomikData } from '../../lib/komikFetcher';
import type { KomikData } from '../../utils/hooks/useKomik';

export const revalidate = 60;

async function HomePage() {
  let initialManga: KomikData | null = null;
  let initialManhua: KomikData | null = null;
  let initialManhwa: KomikData | null = null;
  let error: string | null = null;

  try {
    // Fetch all three endpoints concurrently
    const [mangaResponse, manhuaResponse, manhwaResponse] =
      await Promise.allSettled([
        fetchKomikData('/api/komik2/manga?page=1&order=update', revalidate, 10000),
        fetchKomikData('/api/komik2/manhua?page=1&order=update', revalidate, 10000),
        fetchKomikData('/api/komik2/manhwa?page=1&order=update', revalidate, 10000),
      ]);

    if (mangaResponse.status === 'fulfilled') {
      initialManga = mangaResponse.value as KomikData;
    }
    if (manhuaResponse.status === 'fulfilled') {
      initialManhua = manhuaResponse.value as KomikData;
    }
    if (manhwaResponse.status === 'fulfilled') {
      initialManhwa = manhwaResponse.value as KomikData;
    }

    // If all failed, set error
    if (
      mangaResponse.status === 'rejected' &&
      manhuaResponse.status === 'rejected' &&
      manhwaResponse.status === 'rejected'
    ) {
      error = 'Failed to load komik data';
    }
  } catch (err) {
    console.error('Failed to fetch komik data on server:', err);
    error = 'Failed to load komik data';
  }

  return (
    <KomikPageClient
      manga={initialManga}
      manhua={initialManhua}
      manhwa={initialManhwa}
      error={error}
    />
  );
}

export default HomePage;
