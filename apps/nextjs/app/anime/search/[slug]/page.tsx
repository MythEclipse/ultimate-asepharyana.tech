'use client';
import React, { useState, useEffect } from 'react';
import { useParams } from 'next/navigation';
import SearchForm from '../../../../components/misc/SearchForm';
import CardA from '../../../../components/anime/MediaCard'; // Changed to default import
import { Info, AlertTriangle } from 'lucide-react';

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
      `/api/anime/search?q=${encodeURIComponent(query)}`
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
const SearchPage = () => {
  const params = useParams();
  const slug = params?.slug || ''; // Access slug safely
  const query = decodeURIComponent(Array.isArray(slug) ? slug[0] : slug);

  const [searchResults, setSearchResults] = useState<SearchDetailData>({
    status: '',
    data: [],
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (slug) { // Conditionally fetch data
      fetchSearchResults(query).then((result) => {
        setSearchResults(result);
        setLoading(false);
      });
    } else {
      setLoading(false); // If no slug, stop loading and show no results
    }
  }, [query, slug]); // Add slug to dependency array

  if (!slug) {
    return (
      <main className='p-6'>
        <h1 className='text-2xl font-bold mb-4'>Search Results</h1>
        <SearchForm
          classname='w-full mb-8'
          initialQuery={query}
          baseUrl='/anime'
        />
        <div className='p-6 bg-yellow-100 dark:bg-yellow-900/30 rounded-2xl flex items-center gap-4'>
          <AlertTriangle className='w-8 h-8 text-yellow-600 dark:text-yellow-400' />
          <h2 className='text-xl font-medium text-yellow-800 dark:text-yellow-200'>
            Please enter a search query.
          </h2>
        </div>
      </main>
    );
  }
  if (loading) {
    return (
      <main className='p-6'>
        <h1 className='text-2xl font-bold mb-4'>Search Results</h1>
        <SearchForm
          classname='w-full mb-8'
          initialQuery={query}
          baseUrl='/anime'
        />
        <div className='grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4'>
          {Array.from({ length: 15 }).map((_, index) => (
            <CardA key={index} loading={loading} />
          ))}
        </div>
      </main>
    );
  }
  return (
    <main className='p-6'>
      <h1 className='text-2xl font-bold mb-4'>Search Results</h1>
      <SearchForm
        classname='w-full mb-8'
        initialQuery={query}
        baseUrl='/anime'
      />
      {searchResults.data.length > 0 ? (
        <div className='grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4'>
          {searchResults.data.map((anime) => (
            <CardA
              key={anime.slug}
              title={anime.title}
              description={`${anime.status || 'Unknown status'} • ⭐${anime.rating || 'N/A'}`}
              imageUrl={anime.poster} // Changed imageUrl to image
              linkUrl={`/anime/detail/${anime.slug}`} // Changed linkUrl to link
            />
          ))}
        </div>
      ) : (
        <div className='p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4'>
          <Info className='w-8 h-8 text-blue-600 dark:text-blue-400' />
          <h2 className='text-xl font-medium text-blue-800 dark:text-blue-200'>
            No results found for "{query}"
          </h2>
        </div>
      )}
    </main>
  );
};
export default SearchPage;
