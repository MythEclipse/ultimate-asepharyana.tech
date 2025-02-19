'use client';

import React from 'react';
import useSWR from 'swr';
import Link from 'next/link';
import Loading from '@/components/misc/loading';
import ButtonA from '@/components/button/ScrollButton';
import AnimeGrid from '@/components/card/AnimeGrid2';
import { BaseUrl } from '@/lib/url';

interface HomeData {
  status: string;
  data: {
    anime_list: Anime[];
  };
}

interface Anime {
  title: string;
  slug: string;
  poster: string;
  episode: string;
  anime_url: string;
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function AnimePage() {
  const { data: episodeData, error } = useSWR<HomeData>(
    `${BaseUrl}/api/anime2/`,
    fetcher,
    {
      revalidateOnFocus: false,
      dedupingInterval: 600000, // 10 minutes
    }
  );

  if (error) {
    console.error('Failed to fetch episodes:', error);
    return <div>Failed to load</div>;
  }

  if (!episodeData) {
    return <Loading />;
  }

  return (
    <main className='p-3'>
      <h1 className='text-3xl font-bold mb-6 dark:text-white'>Anime</h1>
      <>
        {/* Anime List Section */}
        <Link href={'/anime2/anime-list/1'}>
          <ButtonA className='w-full max-w-[800rem] text-center py-4 px-8'>
            Latest Anime List
          </ButtonA>
        </Link>
        {episodeData?.data?.anime_list && (
          <AnimeGrid
            animes={episodeData.data.anime_list.map((anime) => ({
              ...anime,
              rating: '',
              release_day: '',
              newest_release_date: '',
            }))}
          />
        )}
      </>
    </main>
  );
}
