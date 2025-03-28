'use client';

import AnimeGrid from '@/components/card/AnimeGrid';
import { Link } from 'next-view-transitions';
import {
  AlertTriangle,
  Info,
  CheckCircle,
  ChevronLeft,
  ChevronRight,
  ArrowRight,
} from 'lucide-react';
import useSWR from 'swr';
import { useParams } from 'next/navigation';

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

interface CompleteAnimeData {
  status: string;
  data: Anime[];
  pagination: Pagination;
}

export default function AnimePage() {
  const params = useParams();
  const slug = params.slug as string;

  const { data, error, isLoading } = useSWR<CompleteAnimeData | null>(
    `/api/anime2/complete-anime/${slug}`,
    async (url: string | URL | Request) => {
      const res = await fetch(url);
      if (!res.ok) throw new Error('Gagal memuat data');
      return res.json();
    }
  );

  if (error) {
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

  if (isLoading) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto space-y-8">
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

          <AnimeGrid anime2 loading animes={[]} />

          {parseInt(Array.isArray(slug) ? slug[0] : slug ?? '1', 10) > 1 && (
            <PaginationComponent
              pagination={{
                current_page: parseInt(Array.isArray(slug) ? slug[0] : slug ?? '1', 10),
                last_visible_page: undefined,
                has_next_page: true,
                has_previous_page: true,
                previous_page: parseInt(Array.isArray(slug) ? slug[0] : slug ?? '1', 10) - 1,
                next_page: parseInt(Array.isArray(slug) ? slug[0] : slug ?? '1', 10) + 1,
              }}
            />
          )}
        </div>
      </main>
    );
  }

  if (!data) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
            <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
            <h1 className="text-2xl font-bold text-blue-800 dark:text-blue-200">
              Tidak Ada Anime Tersedia
            </h1>
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
              Format Data Tidak Valid
            </h1>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
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

        <AnimeGrid anime2 animes={data.data} />

        <PaginationComponent pagination={data.pagination} />
      </div>
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  return (
    <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <Link href={`/anime2/complete-anime/${pagination.previous_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            <ChevronLeft className="w-5 h-5" />
            Sebelumnya
          </button>
        </Link>
      )}

      <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4">
        Halaman {pagination.current_page} dari {pagination.last_visible_page}
      </span>

      {pagination.has_next_page && pagination.next_page !== null && (
        <Link href={`/anime2/complete-anime/${pagination.next_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            Selanjutnya
            <ChevronRight className="w-5 h-5" />
          </button>
        </Link>
      )}
    </div>
  );
};