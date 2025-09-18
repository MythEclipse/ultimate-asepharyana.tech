import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import AnimeGrid from '../../../../components/anime/AnimeGrid';
import { APIURLSERVER } from '../../../../lib/url';
import {
  AlertTriangle,
  Info,
  ChevronLeft,
  ChevronRight,
  Clapperboard,
} from 'lucide-react';

export const revalidate = 60;

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

async function AnimePage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;

  if (!slug) {
    notFound();
  }

  let data: OngoingAnimeData | null = null;
  let error: string | null = null;

  try {
    const url = `/api/anime2/ongoing-anime/${slug}`;
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    data = await response.json();
  } catch (err) {
    console.error('Failed to fetch ongoing anime2 data on server:', err);
    error = 'Failed to load ongoing anime data';
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
        <Link href={`/anime2/ongoing-anime/${pagination.previous_page}`}>
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
        <Link href={`/anime2/ongoing-anime/${pagination.next_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            Selanjutnya
            <ChevronRight className="w-5 h-5" />
          </button>
        </Link>
      )}
    </div>
  );
};

export default AnimePage;
