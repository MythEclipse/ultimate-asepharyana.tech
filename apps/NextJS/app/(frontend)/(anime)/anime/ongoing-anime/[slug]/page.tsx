"use client";

import useSWR from 'swr';
import { useParams } from 'next/navigation';
import { Link } from 'next-view-transitions';
import AnimeGrid from '@/components/card/AnimeGrid';
import { BaseUrl } from '@/lib/url';
import {
  AlertTriangle,
  ChevronLeft,
  ChevronRight,
  Clapperboard,
  ArrowRight,
} from 'lucide-react';

interface OngoingAnimeData {
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

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function AnimePage() {
  const params = useParams();
  const slug = params.slug as string;
  const { data, error, isLoading } = useSWR<OngoingAnimeData | null>(
    `${BaseUrl}/api/anime/ongoing-anime/${slug}`,
    fetcher,
    {
      revalidateOnFocus: false,
    }
  );

  if (isLoading) {
    return (
      <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-7xl mx-auto'>
        {/* <h1 className='text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
                    Anime
                </h1> */}

        {/* Ongoing Anime Section */}
        <section className='mb-12 space-y-6'>
          <div className='flex items-center justify-between mb-6'>
            <div className='flex items-center gap-4'>
              <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
                <Clapperboard className='w-8 h-8 text-blue-600 dark:text-blue-400' />
              </div>
              <h1 className='text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                Currently Airing Anime
              </h1>
            </div>
            <div className='flex items-center gap-2 text-blue-600 dark:text-blue-400'>
              <span className='skeleton w-16 h-4 rounded'></span>
              <ArrowRight className='w-4 h-4' />
            </div>
          </div>

          <AnimeGrid animes={[]} loading={true} />
        </section>

        {/* Complete Anime Section */}
        {/* <section className='space-y-6'>
                    <div className='flex items-center justify-between mb-6'>
                        <div className='flex items-center gap-3'>
                            <div className='p-3 bg-green-100 dark:bg-green-900/50 rounded-xl'>
                                <CheckCircle className='w-6 h-6 text-green-600 dark:text-green-400' />
                            </div>
                            <h2 className='text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent'>
                                Complete Anime
                            </h2>
                        </div>
                        <div className='flex items-center gap-2 text-green-600 dark:text-green-400'>
                            <span className='skeleton w-16 h-4 rounded'></span>
                            <ArrowRight className='w-4 h-4' />
                        </div>
                    </div>

                    <AnimeGrid
                        animes={[]}
                        loading={true}
                    />
                </section> */}
      </div>
    </main>
    );
  }

  if (error || !data) {
    return (
      <main className='min-h-screen p-6 bg-background dark:bg-dark'>
        <div className='max-w-7xl mx-auto mt-12'>
          <div className='p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex items-center gap-4'>
            <AlertTriangle className='w-8 h-8 text-red-600 dark:text-red-400' />
            <div>
              <h1 className='text-2xl font-bold text-red-800 dark:text-red-200 mb-2'>
                Error Loading Data
              </h1>
              <p className='text-red-700 dark:text-red-300'>
                {error?.message || 'Could not fetch data from the API. Please try again later.'}
              </p>
            </div>
          </div>
        </div>
      </main>
    );
  }

  if (!Array.isArray(data.data)) {
    return (
      <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-7xl mx-auto'>
        {/* <h1 className='text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
                    Anime
                </h1> */}

        {/* Ongoing Anime Section */}
        <section className='mb-12 space-y-6'>
          <div className='flex items-center justify-between mb-6'>
            <div className='flex items-center gap-4'>
              <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
                <Clapperboard className='w-8 h-8 text-blue-600 dark:text-blue-400' />
              </div>
              <h1 className='text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                Currently Airing Anime
              </h1>
            </div>
            <div className='flex items-center gap-2 text-blue-600 dark:text-blue-400'>
              <span className='skeleton w-16 h-4 rounded'></span>
              <ArrowRight className='w-4 h-4' />
            </div>
          </div>

          <AnimeGrid animes={[]} loading={true} />
        </section>

        {/* Complete Anime Section */}
        {/* <section className='space-y-6'>
                    <div className='flex items-center justify-between mb-6'>
                        <div className='flex items-center gap-3'>
                            <div className='p-3 bg-green-100 dark:bg-green-900/50 rounded-xl'>
                                <CheckCircle className='w-6 h-6 text-green-600 dark:text-green-400' />
                            </div>
                            <h2 className='text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent'>
                                Complete Anime
                            </h2>
                        </div>
                        <div className='flex items-center gap-2 text-green-600 dark:text-green-400'>
                            <span className='skeleton w-16 h-4 rounded'></span>
                            <ArrowRight className='w-4 h-4' />
                        </div>
                    </div>

                    <AnimeGrid
                        animes={[]}
                        loading={true}
                    />
                </section> */}
      </div>
    </main>
    );
  }

  return (
    <main className='min-h-screen p-6 bg-background dark:bg-dark'>
      <div className='max-w-7xl mx-auto space-y-8'>
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
            <Clapperboard className='w-8 h-8 text-blue-600 dark:text-blue-400' />
          </div>
          <h1 className='text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
            Currently Airing Anime
          </h1>
        </div>

        <AnimeGrid animes={data.data} />

        <PaginationComponent pagination={data.pagination} />
      </div>
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  return (
    <div className='flex flex-wrap gap-4 justify-between items-center mt-8'>
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <Link href={`/anime/ongoing-anime/${pagination.previous_page}`}>
          <button className='px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2'>
            <ChevronLeft className='w-5 h-5' />
            Previous
          </button>
        </Link>
      )}

      <span className='text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4'>
        Page {pagination.current_page} of {pagination.last_visible_page}
      </span>

      {pagination.has_next_page && pagination.next_page !== null && (
        <Link href={`/anime/ongoing-anime/${pagination.next_page}`}>
          <button className='px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2'>
            Next
            <ChevronRight className='w-5 h-5' />
          </button>
        </Link>
      )}
    </div>
  );
};