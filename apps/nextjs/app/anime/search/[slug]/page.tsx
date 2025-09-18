import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import SearchForm from '../../../../components/misc/SearchForm';
import CardA from '../../../../components/anime/MediaCard';
import { Info } from 'lucide-react';
import { APIURLSERVER } from '../../../../lib/url';

export const revalidate = 60;

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

async function SearchPage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const query = decodeURIComponent(Array.isArray(slug) ? slug[0] : slug);

  if (!slug) {
    notFound();
  }

  let searchResults: SearchDetailData = { status: 'error', data: [] };

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
    searchResults = { status: 'error', data: [] };
  }

  return (
    <main className="p-6">
      <h1 className="text-2xl font-bold mb-4">Search Results</h1>
      <SearchForm
        classname="w-full mb-8"
        initialQuery={query}
        baseUrl="/anime"
      />
      {searchResults.data.length > 0 ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4">
          {searchResults.data.map((anime) => (
            <CardA
              key={anime.slug}
              title={anime.title}
              description={`${anime.status || 'Unknown status'} • ⭐${anime.rating || 'N/A'}`}
              imageUrl={anime.poster}
              linkUrl={`/anime/detail/${anime.slug}`}
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

export default SearchPage;
