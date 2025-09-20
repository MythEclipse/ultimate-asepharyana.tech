import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import UnifiedGrid from '../../../../components/shared/UnifiedGrid';
import { APIURLSERVER } from '../../../../lib/url';
import {
  CheckCircle,
  AlertTriangle,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';

export const revalidate = 60;

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

async function AnimePage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;

  if (!slug) {
    notFound();
  }

  let data: CompleteAnimeData | null = null;
  let error: string | null = null;

  try {
    const url = `/api/anime/complete-anime/${slug}`;
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    data = await response.json();
  } catch (err) {
    console.error('Failed to fetch complete anime data on server:', err);
    error = 'Failed to load complete anime data';
  }

  if (error || !data) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex items-center gap-4">
            <AlertTriangle className="w-8 h-8 text-red-600 dark:text-red-400" />
            <div>
              <h1 className="text-2xl font-bold text-red-800 dark:text-red-200 mb-2">
                Error Loading Data
              </h1>
              <p className="text-red-700 dark:text-red-300">
                Could not fetch data from the API. Please try again later.
              </p>
            </div>
          </div>
        </div>
      </main>
    );
  }

  if (!Array.isArray(data.data)) {
    return (
      <main className="p-4 md:p-8 bg-background dark:bg-dark min-h-screen">
        <div className="max-w-7xl mx-auto">
          <section className="space-y-6">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-3">
                <div className="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
                  <CheckCircle className="w-6 h-6 text-green-600 dark:text-green-400" />
                </div>
                <h2 className="text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent">
                  Complete Anime
                </h2>
              </div>
            </div>

            <UnifiedGrid items={[]} loading={true} itemType="anime" />
          </section>
        </div>
      </main>
    );
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
              <CheckCircle className="w-6 h-6 text-green-600 dark:text-green-400" />
            </div>
            <h2 className="text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent">
              Complete Anime
            </h2>
          </div>
        </div>

        <UnifiedGrid items={data.data} itemType="anime" />

        <PaginationComponent pagination={data.pagination} />
      </div>
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  return (
    <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <Link href={`/anime/complete-anime/${pagination.previous_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            <ChevronLeft className="w-5 h-5" />
            Previous
          </button>
        </Link>
      )}

      <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4">
        Page {pagination.current_page} of {pagination.last_visible_page}
      </span>

      {pagination.has_next_page && pagination.next_page !== null && (
        <Link href={`/anime/complete-anime/${pagination.next_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            Next
            <ChevronRight className="w-5 h-5" />
          </button>
        </Link>
      )}
    </div>
  );
};

export default AnimePage;
