import AnimeGrid from '@/components/card/AnimeGrid3';
import { BaseUrl } from '@/lib/url';
import { notFound } from 'next/navigation';
import ButtonA from '@/components/button/ScrollButton';
export const dynamic = 'force-dynamic';
interface HomeData {
  status: string;
  data: {
    ongoing_anime: OngoingAnime[];
    complete_anime: CompleteAnime[];
  };
}

interface OngoingAnime {
  title: string;
  slug: string;
  poster: string;
  current_episode: string;
  anime_url: string;
}

interface CompleteAnime {
  title: string;
  slug: string;
  poster: string;
  episode_count: string;
  anime_url: string;
  current_episode: string;
}

export default async function AnimePage() {
  const res = await fetch(`${BaseUrl}/api/anime2/`, {
    cache: 'no-store',
  });

  if (!res.ok) {
    return notFound();
  }

  const data: HomeData = await res.json();

  if (!data?.data) {
    return notFound();
  }

  return (
    <main className='p-3'>
      <h1 className='text-3xl font-bold mb-6 dark:text-white'>Anime</h1>
      <>
        <a href={'/anime2/ongoing-anime/1'}>
          <ButtonA className='w-full max-w-[800rem] text-center py-4 px-8'>
            Latest Ongoing Anime
          </ButtonA>
        </a>
        <AnimeGrid animes={data.data.ongoing_anime} />

        <a href={'/anime2/complete-anime/1'}>
          <ButtonA className='w-full max-w-[800rem] text-center py-4 px-8'>
            Latest Complete Anime
          </ButtonA>
        </a>
        <AnimeGrid animes={data.data.complete_anime} />
      </>
    </main>
  );
}
