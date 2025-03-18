// app/search/[slug]/page.tsx
import SearchForm from '@/components/misc/SearchForm';
import CardA from '@/components/card/MediaCard';
import { BaseUrl } from '@/lib/url';
import React from 'react';

interface Genre {
  name: string;
  slug: string;
  otakudesu_url: string;
}

interface Anime {
  title: string;
  slug: string;
  poster: string;
  description: string;
  anime_url: string;
  genres: Genre[];
  rating: string;
  type: string;
  season: string;
}

interface SearchDetailData {
  status: string;
  data: Anime[];
  pagination: {
    current_page: number;
    last_visible_page: number;
    has_next_page: boolean;
    next_page: string | null;
    has_previous_page: boolean;
    previous_page: string | null;
  };
}

const fetchSearchResults = async (query: string): Promise<SearchDetailData> => {
  try {
    const response = await fetch(
      `${BaseUrl}/api/anime2/search?q=${encodeURIComponent(query)}`
    );
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    const result: SearchDetailData = await response.json();
    return result;
  } catch (error) {
    console.error('Error fetching search results:', error);
    return { status: 'error', data: [], pagination: { current_page: 1, last_visible_page: 1, has_next_page: false, next_page: null, has_previous_page: false, previous_page: null } };
  }
};

const SearchPage = async (props: { params: Promise<{ slug: string }> }) => {
  const params = await props.params;
  const { slug } = await params;
  const query = decodeURIComponent(slug);
  const searchResults = await fetchSearchResults(query);

  return (
    <div className='p-6'>
      <h1 className='text-3xl font-bold mb-6 dark:text-white'>Search Anime</h1>
      <SearchForm
        classname='w-full mb-6'
        initialQuery={query}
        baseUrl='/anime2'
      />
      <div>
        {searchResults.data.length > 0 ? (
          <div className='flex flex-col items-center p-4'>
            <div className='grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-4'>
              {searchResults.data.map((anime) => (
                <CardA
                  key={anime.slug}
                  title={anime.title}
                  description={anime.description}
                  imageUrl={anime.poster}
                  linkUrl={anime.anime_url}
                />
              ))}
            </div>
          </div>
        ) : (
          <p className='text-gray-600'>No results found</p>
        )}
      </div>
    </div>
  );
};

export default SearchPage;
