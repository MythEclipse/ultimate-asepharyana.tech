import { APIURLSERVER } from '../../../../utils/url-utils';
export const revalidate = 60;

import { AnimeData } from '../../../../types/anime';
import ErrorLoadingDisplay from '../../../../components/shared/ErrorLoadingDisplay';
import Anime2DetailPageClient from './Anime2DetailPageClient';

export default async function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;

  if (!slug) {
    return <ErrorLoadingDisplay type="error" title="Invalid URL" message="The slug is missing." />;
  }

  let initialData: AnimeData | null = null;
  let initialError: string | null = null;

  try {
    const url = `/api/anime2/detail/${slug}`;
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
    const result = await response.json();
    initialData = result.data; // Assuming the API returns { data: AnimeData }
  } catch (err) {
    console.error('Failed to fetch anime detail data on server:', err);
    initialError = 'Failed to load anime data';
  }

  return (
    <Anime2DetailPageClient initialData={initialData} initialError={initialError} />
  );
}
