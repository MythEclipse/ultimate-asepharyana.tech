import React from 'react';
import { notFound } from 'next/navigation';
import { fetchWithFallback } from '../../../../utils/url-utils';
import CompleteAnime2PageClient from './CompleteAnime2PageClient';
import type { CompleteAnimeData2 } from '../../../../utils/hooks/useAnime2';

export const revalidate = 60;

async function AnimePage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;

  if (!slug) {
    notFound();
  }

  let initialData: CompleteAnimeData2 | null = null;
  let initialError: string | null = null;

  try {
    const url = `/api/anime2/complete-anime/${slug}`;
    const response = await fetchWithFallback(url, {
      revalidate: 60,
      signal: AbortSignal.timeout(10000),
    });

    initialData = await response.json();
  } catch (err) {
    console.error('Failed to fetch complete anime2 data:', err);
    initialError = 'Failed to load complete anime data';
  }

  return (
    <CompleteAnime2PageClient
      initialData={initialData}
      initialError={initialError}
      slug={slug}
    />
  );
}

export default AnimePage;
