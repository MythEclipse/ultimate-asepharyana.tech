import Link from 'next/link';
import AnimeGrid from '@/components/card/AnimeGrid';
import { BaseUrl } from '@/lib/url';
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

async function getAnimeData(): Promise<HomeData | null> {
  try {
    const res = await fetch(`${BaseUrl}/api/anime/`, {
      cache: 'no-store',
    });

    if (!res.ok) return null;

    return res.json();
  } catch {
    return null;
  }
}

export default async function AnimePage() {
  const data = await getAnimeData();

  if (!data) {
    return <div>Error loading data</div>;
  }

  return (
    <main className='p-3'>
      <h1 className='text-3xl font-bold mb-6 dark:text-white'>Anime</h1>

      <Link href={'/anime/ongoing-anime/1'}>
        <ButtonA className='w-full max-w-[800rem] text-center py-4 px-8'>
          Latest Ongoing Anime
        </ButtonA>
      </Link>
      <AnimeGrid
        animes={data.data.ongoing_anime.map((anime) => ({
          ...anime,
          rating: '',
          release_day: '',
          newest_release_date: '',
        }))}
      />

      <Link scroll href={'/anime/complete-anime/1'}>
        <ButtonA className='w-full max-w-[800rem] text-center py-4 px-8'>
          Latest Complete Anime
        </ButtonA>
      </Link>
      <AnimeGrid
        animes={data.data.complete_anime.map((anime) => ({
          ...anime,
          rating: '',
          release_day: '',
          newest_release_date: '',
          current_episode: '',
        }))}
      />
    </main>
  );
}
