import { fetchWithFallback } from '../../../../utils/url-utils';
import SearchPage2Client from './SearchPage2Client';
import type { SearchDetailData2 } from '../../../../utils/hooks/useAnime2';

export const revalidate = 60;

export default async function SearchPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const query = decodeURIComponent(slug);

  let searchResults: SearchDetailData2 | null = null;
  let error: string | null = null;

  try {
    const url = `/api/anime2/search?q=${encodeURIComponent(query)}`;
    const response = await fetchWithFallback(url, {
      revalidate: 60,
      signal: AbortSignal.timeout(10000),
    });

    searchResults = await response.json();
  } catch (_e) {
    searchResults = { status: 'error', data: [] };
    error = 'Failed to load search results';
  }

  return (
    <SearchPage2Client
      initialData={searchResults}
      initialError={error}
      query={query}
    />
  );
}
