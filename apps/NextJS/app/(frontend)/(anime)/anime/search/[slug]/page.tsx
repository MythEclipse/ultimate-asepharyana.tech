// app/search/[slug]/page.tsx
import SearchForm from '@/components/misc/SearchForm';
import CardA from '@/components/card/MediaCard';
import { BaseUrl } from '@/lib/url';
import React from 'react';
import { Search, Info } from 'lucide-react';
export const dynamic = 'force-dynamic';
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

const fetchSearchResults = async (query: string): Promise<SearchDetailData> => {
  try {
    const response = await fetch(
      `${BaseUrl}/api/anime/search?q=${encodeURIComponent(query)}`
    );
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    const result: SearchDetailData = await response.json();
    return result;
  } catch (error) {
    console.error('Error fetching search results:', error);
    return { status: 'error', data: [] };
  }
};

const SearchPage = async (props: { params: Promise<{ slug: string }> }) => {
  const params = await props.params;
  const { slug } = await params;
  const query = decodeURIComponent(slug);
  const searchResults = await fetchSearchResults(query);

  return (
    <main className='min-h-screen p-6 bg-background dark:bg-dark'>
      <div className='max-w-7xl mx-auto space-y-8'>
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-purple-100 dark:bg-purple-900/50 rounded-xl'>
            <Search className='w-8 h-8 text-purple-600 dark:text-purple-400' />
          </div>
          <h1 className='text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent'>
            Search Results
          </h1>
        </div>

        <SearchForm classname='w-full' initialQuery={query} baseUrl='/anime' />

        {searchResults.data.length > 0 ? (
          <div className='grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4'>
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
          <div className='p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4'>
            <Info className='w-8 h-8 text-blue-600 dark:text-blue-400' />
            <h2 className='text-xl font-medium text-blue-800 dark:text-blue-200'>
              No results found for &quot;{query}&quot;
            </h2>
          </div>
        )}
      </div>
    </main>
  );
};

export default SearchPage;
