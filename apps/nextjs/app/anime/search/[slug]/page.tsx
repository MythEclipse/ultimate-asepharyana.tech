import { notFound } from 'next/navigation';
import { fetchWithFallback } from '../../../../utils/url-utils';
import { SearchDetailData } from '../../../../types/anime';
import SearchPageClient from './SearchPageClient';

export const revalidate = 60;

async function SearchPage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const query = decodeURIComponent(Array.isArray(slug) ? slug[0] : slug);

  if (!slug) {
    notFound();
  }

  let searchResults: SearchDetailData | null = null;
  let error: string | null = null;

  try {
    const url = `/api/anime/search?q=${encodeURIComponent(query)}`;
    const response = await fetchWithFallback(url, {
      revalidate,
      signal: AbortSignal.timeout(10000),
    });

    searchResults = await response.json();
  } catch (err) {
    console.error('Failed to fetch search results:', err);
    error = 'Failed to load search results';
  }

  return (
    <SearchPageClient
      initialData={searchResults}
      initialError={error}
      query={query}
    />
  );
}

export default SearchPage;
