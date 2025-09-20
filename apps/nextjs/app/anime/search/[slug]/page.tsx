import { notFound } from 'next/navigation';
import { APIURLSERVER } from '../../../../lib/url';
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
    searchResults = await response.json();
  } catch (err) {
    console.error('Failed to fetch search results on server:', err);
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
