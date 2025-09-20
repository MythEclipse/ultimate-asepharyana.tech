import React from 'react';
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

const fetchSearchResults = async (query: string): Promise<SearchDetailData> => {
  const fetchData = async (url: string) => {
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
    return await response.json();
  };

  try {
    const response = await fetchData(
      `/api/komik2/search?query=${encodeURIComponent(query)}&page=1`,
    );
    if (response.status && response.status >= 400) {
      throw new Error('Network response was not ok');
    }
    const result: SearchDetailData = response.data;
    return result;
  } catch (error) {
    console.error('Error fetching search results:', error);
    return { status: 'error', data: [] };
  }
};

export default async function SearchPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const query = decodeURIComponent(slug);
  const searchResults = await fetchSearchResults(query);
  if (searchResults.data.length === 0 && searchResults.status === 'error') {
    return (
      <main className="p-6">
        <h1 className="text-2xl font-bold mb-4">Search Results</h1>
        <SearchForm
          classname="w-full mb-8"
          initialQuery={query}
          baseUrl="/komik"
        />
        <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
          <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
          <h2 className="text-xl font-medium text-blue-800 dark:text-blue-200">
            No results found for "{query}"
          </h2>
        </div>
      </main>
    );
  }
  return (
    <main className="p-6">
      <h1 className="text-2xl font-bold mb-4">Search Results</h1>
      <SearchForm
        classname="w-full mb-8"
        initialQuery={query}
        baseUrl="/komik"
      />
      {searchResults.data.length > 0 ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4">
          {searchResults.data.map((anime) => (
            <CardA
              key={anime.slug}
              title={anime.title}
              description={`${anime.status || 'Unknown status'} • ⭐${anime.rating || 'N/A'}`}
              imageUrl={anime.poster}
              linkUrl={`/komik/detail/${anime.slug}`}
            />
          ))}
        </div>
      ) : (
        <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
          <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
          <h2 className="text-xl font-medium text-blue-800 dark:text-blue-200">
            No results found for &quot;{query}&quot;
          </h2>
        </div>
      )}
    </main>
  );
}
