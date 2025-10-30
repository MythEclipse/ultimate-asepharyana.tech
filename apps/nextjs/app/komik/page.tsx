import React from 'react';
import KomikPageClient from './KomikPageClient';
import { fetchWithFallback } from '../../utils/url-utils';
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
        fetchWithFallback('/api/komik2/manga?page=1&order=update', {
          revalidate,
          signal: AbortSignal.timeout(10000),
        }),
        fetchWithFallback('/api/komik2/manhua?page=1&order=update', {
          revalidate,
          signal: AbortSignal.timeout(10000),
        }),
        fetchWithFallback('/api/komik2/manhwa?page=1&order=update', {
          revalidate,
          signal: AbortSignal.timeout(10000),
        }),
      ]);

    if (mangaResponse.status === 'fulfilled') {
      const data = await mangaResponse.value.json();
      // Extract only the data array, not the pagination
      initialManga = { data: data.data };
    }
    if (manhuaResponse.status === 'fulfilled') {
      const data = await manhuaResponse.value.json();
      initialManhua = { data: data.data };
    }
    if (manhwaResponse.status === 'fulfilled') {
      const data = await manhwaResponse.value.json();
      initialManhwa = { data: data.data };
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
