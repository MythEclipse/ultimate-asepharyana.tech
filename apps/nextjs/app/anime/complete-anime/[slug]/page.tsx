import React from 'react';
import { notFound } from 'next/navigation';
import { fetchWithFallback } from '../../../../utils/url-utils';
import { CompleteAnimeData } from '../../../../types/anime';
import CompleteAnimePageClient from './CompleteAnimePageClient';

export const revalidate = 60;

async function AnimePage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;

  if (!slug) {
    notFound();
  }

  let initialData: CompleteAnimeData | null = null;
  let initialError: string | null = null;

  try {
    const url = `/api/anime/complete-anime/${slug}`;
    const response = await fetchWithFallback(url, {
      revalidate,
      signal: AbortSignal.timeout(10000),
    });

    initialData = await response.json();
  } catch (err) {
    console.error('Failed to fetch complete anime data:', err);
    initialError = 'Failed to load complete anime data';
  }

  return (
    <CompleteAnimePageClient
      initialData={initialData}
      initialError={initialError}
      slug={slug}
    />
  );
}

export default AnimePage;
