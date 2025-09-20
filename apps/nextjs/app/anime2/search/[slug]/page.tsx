import { APIURLSERVER } from '../../../../lib/url';
import { SearchDetailData } from '../../../../types/anime';
import SearchPage2Client from './SearchPage2Client';

export const revalidate = 60;

export default async function SearchPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const query = decodeURIComponent(slug);

  let searchResults: SearchDetailData | null = null;
  let error: string | null = null;

  try {
    const url = `/api/anime2/search?q=${encodeURIComponent(query)}`;
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
