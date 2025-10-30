import { fetchWithFallback } from '../../../../utils/url-utils';
export const revalidate = 60;

import ErrorLoadingDisplay from '../../../../components/shared/ErrorLoadingDisplay';
import Anime2DetailPageClient from './Anime2DetailPageClient';
import type { Anime2Data } from '../../../../utils/hooks/useAnime2';

export default async function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;

  if (!slug) {
    return <ErrorLoadingDisplay type="error" title="Invalid URL" message="The slug is missing." />;
  }

  let initialData: Anime2Data | null = null;
  let initialError: string | null = null;

  try {
    const url = `/api/anime2/detail/${slug}`;
    const response = await fetchWithFallback(url, {
      revalidate: 60,
      signal: AbortSignal.timeout(10000),
    });

    const result = await response.json();
    initialData = result.data; // Assuming the API returns { data: AnimeData }
  } catch (err) {
    console.error('Failed to fetch anime detail data:', err);
    initialError = 'Failed to load anime data';
  }

  return (
    <Anime2DetailPageClient slug={slug} initialData={initialData} initialError={initialError} />
  );
}
