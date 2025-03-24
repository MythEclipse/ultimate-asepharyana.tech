'use client';

import React, { useEffect, useState } from 'react';
import useSWR from 'swr';
import AnimeGrid from '@/components/card/AnimeGrid';
import Link from 'next/link';
import { BaseUrl } from '@/lib/url';
import ButtonA from '@/components/button/ScrollButton';
import Loading from '@/components/misc/loading';

interface CompleteAnimeData {
  status: string;
  data: Anime[];
  pagination: Pagination;
}

interface Anime {
  title: string;
  slug: string;
  poster: string;
  episode: string;
  anime_url: string;
  rating: string;
  last_release_date: string;
  current_episode: string;
  release_day: string;
  newest_release_date: string;
}

interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}

interface DetailAnimePageProps {
  params: Promise<{ slug: string }>;
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function AnimePage({ params }: DetailAnimePageProps) {
  const [resolvedParams, setResolvedParams] = useState<{ slug: string } | null>(
    null
  );
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    params.then(setResolvedParams);
    setMounted(true);
  }, [params]);

  const { data, error, isLoading } = useSWR<CompleteAnimeData>(
    mounted && resolvedParams
      ? `${BaseUrl}/api/anime/complete-anime/${resolvedParams.slug}`
      : null,
    fetcher,
    {
      revalidateIfStale: false,
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
      dedupingInterval: 0,
      compare: (a, b) => JSON.stringify(a) === JSON.stringify(b), // Hindari infinite loop
    }
  );

  if (!mounted || !resolvedParams) {
    return (
      <div className='min-h-screen flex items-center justify-center'>
        <Loading />
      </div>
    );
  }

  if (error) {
    return (
      <main className='p-6'>
        <h1 className='text-2xl font-bold mt-8 mb-4'>Error Loading Data</h1>
        <p>Could not fetch data from the API. Please try again later.</p>
      </main>
    );
  }

  if (isLoading || !data) {
    return (
      <div className='min-h-screen flex items-center justify-center overflow-hidden'>
        <Loading />
      </div>
    );
  }

  if (!Array.isArray(data.data)) {
    return (
      <main className='p-6'>
        <h1 className='text-2xl font-bold mt-8 mb-4'>No Data Available</h1>
      </main>
    );
  }

  return (
    <main className='p-6'>
      <h1 className='dark:text-lighta text-2xl font-bold mt-8 mb-4'>
        Currently Finished Anime
      </h1>
      <AnimeGrid animes={data.data} />
      <PaginationComponent pagination={data.pagination} />
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  return (
    <div className='flex justify-between mt-8'>
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <div className='text-2xl font-bold mt-8 mb-4'>
          <Link
            scroll
            href={`/anime/complete-anime/${pagination.previous_page}`}
            className='text-blue-600 hover:underline'
          >
            <ButtonA>Previous</ButtonA>
          </Link>
        </div>
      )}
      {pagination.has_next_page && pagination.next_page !== null && (
        <div className='text-2xl font-bold mt-8 mb-4'>
          <Link
            scroll
            href={`/anime/complete-anime/${pagination.next_page}`}
            className='text-blue-600 hover:underline'
          >
            <ButtonA>Next</ButtonA>
          </Link>
        </div>
      )}
    </div>
  );
};
