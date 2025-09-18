import SearchForm from '../../../../components/misc/SearchForm';
import CardA from '../../../../components/anime/MediaCard'; // Changed to default import
import { Info } from 'lucide-react';
import { APIURLSERVER } from '../../../../lib/url';

interface Genre {
  name: string;
  slug: string;
  otakudesu_url: string;
}
interface Anime {
  title: string;
  slug: string;
  poster: string;
  genres?: Genre[];
  status?: string;
  rating?: string;
  episode_count?: number;
  last_release_date?: string;
  url?: string;
}

interface SearchDetailData {
  status: string;
  data: Anime[];
}

export const revalidate = 60;

export default async function SearchPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const query = decodeURIComponent(slug);

  let searchResults: SearchDetailData;

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
  } catch (error) {
    searchResults = { status: 'error', data: [] };
  }

  return (
    <main className="p-6">
      <h1 className="text-2xl font-bold mb-4">Search Results</h1>
      <SearchForm
        classname="w-full mb-8"
        initialQuery={query}
        baseUrl="/anime2"
      />
      {searchResults.data.length > 0 ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4">
          {searchResults.data.map((anime) => (
            <CardA
              key={anime.slug}
              title={anime.title}
              description={`${anime.status || 'Unknown status'} • ⭐${anime.rating || 'N/A'}`}
              imageUrl={anime.poster}
              linkUrl={`/anime2/detail/${anime.slug}`}
            />
          ))}
        </div>
      ) : (
        <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
          <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
          <h2 className="text-xl font-medium text-blue-800 dark:text-blue-200">
            No results found for "{query}"
          </h2>
        </div>
      )}
    </main>
  );
}
