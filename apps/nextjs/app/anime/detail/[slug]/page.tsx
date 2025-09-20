import React from 'react';
import AnimeDetailPageClient from './AnimeDetailPageClient';
import { APIURLSERVER } from '../../../../lib/url';
import { AnimeData } from '../../../../types/anime';

export const revalidate = 60;

async function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  let initialData: AnimeData | null = null;
  let error: string | null = null;

  try {
    const url = `/api/anime/detail/${slug}`;
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
    initialData = responseData.data; // Extract the nested data
  } catch (err) {
    console.error('Failed to fetch anime detail data on server:', err);
    error = 'Failed to load anime data';
  }

  return (
    <AnimeDetailPageClient
      slug={slug}
      initialData={initialData}
      initialError={error}
    />
  );
}

export default DetailAnimePage;
