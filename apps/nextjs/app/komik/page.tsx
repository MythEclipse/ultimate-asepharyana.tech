import React from 'react';
import KomikPageClient from './KomikPageClient';
import { APIURLSERVER } from '../../lib/url';

export const revalidate = 60;

export interface Komik {
  title: string;
  poster: string;
  chapter: string;
  date: string;
  reader_count: string;
  type: string;
  slug: string;
}

interface KomikData {
  data: Komik[];
}

async function HomePage() {
  const fetchData = async (url: string) => {
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  };

  let initialManga: KomikData | null = null;
  let initialManhua: KomikData | null = null;
  let initialManhwa: KomikData | null = null;
  let error: string | null = null;

  try {
    // Fetch all three endpoints concurrently
    const [mangaResponse, manhuaResponse, manhwaResponse] = await Promise.allSettled([
      fetchData('/api/komik2/manga?page=1&order=update'),
      fetchData('/api/komik2/manhua?page=1&order=update'),
      fetchData('/api/komik2/manhwa?page=1&order=update'),
    ]);

    if (mangaResponse.status === 'fulfilled') {
      initialManga = mangaResponse.value;
    }
    if (manhuaResponse.status === 'fulfilled') {
      initialManhua = manhuaResponse.value;
    }
    if (manhwaResponse.status === 'fulfilled') {
      initialManhwa = manhwaResponse.value;
    }

    // If all failed, set error
    if (mangaResponse.status === 'rejected' &&
        manhuaResponse.status === 'rejected' &&
        manhwaResponse.status === 'rejected') {
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
