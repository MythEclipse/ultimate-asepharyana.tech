import React from 'react';
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

export default async function AnimePage() {
  let episodeData: HomeData | null = null;

  try {
    const response = await fetch(`${BaseUrl}/api/anime2/`, {
      next: { revalidate: 600 },
    });
    episodeData = await response.json();
  } catch (error) {
    console.error('Failed to fetch episodes:', error);
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
        <AnimeGrid
          animes={episodeData.data.anime_list.map((anime) => ({
            ...anime,
            rating: '',
            release_day: '',
            newest_release_date: '',
          }))}
        />
      </>
    </main>
  );
}
