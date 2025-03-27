'use client';
// app/search/[slug]/page.tsx
import SearchForm from '@/components/misc/SearchForm';
import CardA from '@/components/card/MediaCard';
import React, { useEffect, useState } from 'react';
import useSWR from 'swr';
import Loading from '@/components/misc/loading';
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

const fetcher = (url: string) => fetch(url).then((res) => res.json());

const SearchPage = ({ params }: { params: Promise<{ slug: string }> }) => {
  const [resolvedParams, setResolvedParams] = useState<{ slug: string } | null>(
    null
  );

  useEffect(() => {
    params.then(setResolvedParams);
  }, [params]);

  const query = resolvedParams ? decodeURIComponent(resolvedParams.slug) : '';
  const { data: searchResults, error } = useSWR<SearchDetailData>(
    resolvedParams ? `/api/anime2/search?q=${encodeURIComponent(query)}` : null,
    fetcher,
    {
      revalidateIfStale: false,
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
      dedupingInterval: 0,
      compare: (a, b) => JSON.stringify(a) === JSON.stringify(b), // Hindari infinite loop
    }
  );

  if (error) return <div>Error loading search results</div>;
  if (!searchResults) return <Loading />;

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
