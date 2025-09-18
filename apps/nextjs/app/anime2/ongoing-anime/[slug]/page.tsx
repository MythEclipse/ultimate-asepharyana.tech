'use client';

import AnimeGrid from '../../../../components/anime/AnimeGrid';
import {
  AlertTriangle,
  Info,
  ChevronLeft,
  ChevronRight,
  ArrowRight,
  Clapperboard,
} from 'lucide-react';
import useSWR from 'swr';
import { useParams } from 'next/navigation';

import { useRouter } from 'next/navigation';

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
  current_page?: number;
  last_visible_page?: number;
  has_next_page?: boolean;
  next_page?: number | null;
  has_previous_page?: boolean;
  previous_page?: number | null;
}

interface OngoingAnimeData {
  status: string;
  data: Anime[];
  pagination: Pagination;
}

import { fetchData } from '../../../../utils/useFetch';

const fetcher = async (url: string) => {
  const response = await fetchData(url);
  return response.data;
};

export default function AnimePage() {
  const params = useParams();
  const slug = params?.slug as string;

  const { data, error, isLoading } = useSWR<OngoingAnimeData | null>(
    slug ? `/api/anime2/ongoing-anime/${slug}` : null,
    fetcher,
    {
      refreshInterval: 60000,
    }
  );

  if (!slug) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-yellow-100 dark:bg-yellow-900/30 rounded-2xl flex items-center gap-4">
            <AlertTriangle className="w-8 h-8 text-yellow-600 dark:text-yellow-400" />
            <div>
              <h1 className="text-2xl font-bold text-yellow-800 dark:text-yellow-200 mb-2">
                Loading or Invalid URL
              </h1>
              <p className="text-yellow-700 dark:text-yellow-300">
                Please ensure you are accessing this page with a valid slug.
              </p>
            </div>
          </div>
        </div>
      </main>
    );
  }

  if (isLoading) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto">
          {/* <h1 className='text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
                    Anime
                </h1> */}

          {/* Ongoing Anime Section */}
          <section className="mb-12 space-y-6">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-3">
                <div className="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
                  <Clapperboard className="w-6 h-6 text-blue-600 dark:text-blue-400" />
                </div>
                <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                  Ongoing Anime
                </h2>
              </div>
              <div className="flex items-center gap-2 text-blue-600 dark:text-blue-400">
                <span className="skeleton w-16 h-4 rounded"></span>
                <ArrowRight className="w-4 h-4" />
              </div>
            </div>

            <AnimeGrid anime2 loading animes={[]} />

            {parseInt(Array.isArray(slug) ? slug[0] : (slug ?? '1'), 10) >
              1 && (
              <PaginationComponent
                pagination={{
                  current_page: parseInt(
                    Array.isArray(slug) ? slug[0] : (slug ?? '1'),
                    10,
                  ),
                  last_visible_page: undefined,
                  has_next_page: true,
                  has_previous_page: true,
                  previous_page:
                    parseInt(
                      Array.isArray(slug) ? slug[0] : (slug ?? '1'),
                      10,
                    ) - 1,
                  next_page:
                    parseInt(
                      Array.isArray(slug) ? slug[0] : (slug ?? '1'),
                      10,
                    ) + 1,
                }}
              />
            )}
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
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex items-center gap-4">
            <AlertTriangle className="w-8 h-8 text-red-600 dark:text-red-400" />
            <div>
              <h1 className="text-2xl font-bold text-red-800 dark:text-red-200 mb-2">
                Error Memuat Data
              </h1>
              <p className="text-red-700 dark:text-red-300">
                Gagal mengambil data dari API. Silakan coba lagi nanti.
              </p>
            </div>
          </div>
        </div>
      </main>
    );
  }

  if (!Array.isArray(data.data)) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
            <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
            <h1 className="text-2xl font-bold text-blue-800 dark:text-blue-200">
              No Anime Available
            </h1>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
              <Clapperboard className="w-6 h-6 text-blue-600 dark:text-blue-400" />
            </div>
            <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              Ongoing Anime
            </h2>
          </div>
          <div className="flex items-center gap-2 text-blue-600 dark:text-blue-400">
            <span className="skeleton w-16 h-4 rounded"></span>
            <ArrowRight className="w-4 h-4" />
          </div>
        </div>

        <AnimeGrid anime2 animes={data.data} />

        <PaginationComponent pagination={data.pagination} />
      </div>
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  const router = useRouter();
  return (
    <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <button
          onClick={() =>
            router.push(`/anime2/ongoing-anime/${pagination.previous_page}`)
          }
          className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2"
        >
          <ChevronLeft className="w-5 h-5" />
          Previous
        </button>
      )}

      <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4">
        Page {pagination.current_page} of {pagination.last_visible_page}
      </span>

      {pagination.has_next_page && pagination.next_page !== null && (
        <button
          onClick={() =>
            router.push(`/anime2/ongoing-anime/${pagination.next_page}`)
          }
          className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2"
        >
          Next
          <ChevronRight className="w-5 h-5" />
        </button>
      )}
    </div>
  );
};
