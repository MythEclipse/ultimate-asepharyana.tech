import AnimeGrid from '@/components/card/AnimeGrid3';
import { BaseUrl } from '@/lib/url';
import { notFound } from 'next/navigation';
// import ButtonA from '@/components/button/ScrollButton';
import { ArrowRight, CheckCircle, Clapperboard } from 'lucide-react';
import Link from 'next/link';
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
    <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-7xl mx-auto'>
        <h1 className='text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
          Anime
        </h1>

        {/* Ongoing Anime Section */}
        <section className='mb-12 space-y-6'>
          <div className='flex items-center justify-between mb-6'>
            <div className='flex items-center gap-3'>
              <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
                <Clapperboard className='w-6 h-6 text-blue-600 dark:text-blue-400' />
              </div>
              <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                Ongoing Anime
              </h2>
            </div>
            <Link
              href='/anime2/ongoing-anime/1'
              className='flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors'
            >
              View All
              <ArrowRight className='w-4 h-4' />
            </Link>
          </div>

          <AnimeGrid
            animes={data.data.ongoing_anime.map((anime) => ({
              ...anime,
              rating: '',
              release_day: '',
              newest_release_date: '',
            }))}
          />
        </section>

        {/* Complete Anime Section */}
        <section className='space-y-6'>
          <div className='flex items-center justify-between mb-6'>
            <div className='flex items-center gap-3'>
              <div className='p-3 bg-green-100 dark:bg-green-900/50 rounded-xl'>
                <CheckCircle className='w-6 h-6 text-green-600 dark:text-green-400' />
              </div>
              <h2 className='text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent'>
                Complete Anime
              </h2>
            </div>
            <Link
              href='/anime2/complete-anime/1'
              className='flex items-center gap-2 text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 transition-colors'
            >
              View All
              <ArrowRight className='w-4 h-4' />
            </Link>
          </div>

          <AnimeGrid
            animes={data.data.complete_anime.map((anime) => ({
              ...anime,
              rating: '',
              release_day: '',
              newest_release_date: '',
              current_episode: '',
            }))}
          />
        </section>
      </div>
    </main>
  );
}
