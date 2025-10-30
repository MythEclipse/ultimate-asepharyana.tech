import { APIURLSERVER } from '../../../../utils/url-utils';
import { notFound } from 'next/navigation';
import AnimeFullPageClient from './AnimeFullPageClient';
import type { AnimeEpisodeData } from '../../../../utils/hooks/useAnime';

export const revalidate = 60;

// --- MAIN COMPONENT ---
export default async function WatchAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;

  if (!slug) {
    notFound();
  }

  let initialData: AnimeEpisodeData | null = null;
  let initialError: string | null = null;

  try {
    const url = `/api/anime/full/${slug}`;
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const responseData = await response.json();
    initialData = responseData.data; // Extract nested data
  } catch (err) {
    console.error('Failed to fetch episode data on server:', err);
    initialError = 'Terjadi kesalahan saat mengambil data. Episode mungkin tidak ada atau link rusak.';
  }

  return (
    <AnimeFullPageClient
      slug={slug}
      initialData={initialData}
      initialError={initialError}
    />
  );
}
