import AnimeGrid from '@/components/card/AnimeGrid2a';
import { Link } from 'next-view-transitions';
import { BaseUrl } from '@/lib/url';
import {
  AlertTriangle,
  Info,
  CheckCircle,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';
export const dynamic = 'force-dynamic';
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

interface OngoingAnimeData {
  status: string;
  data: Anime[];
  pagination: Pagination;
}

async function getAnimeData(slug: string): Promise<OngoingAnimeData | null> {
  try {
    const res = await fetch(`${BaseUrl}/api/anime2/ongoing-anime/${slug}`, {
      cache: 'no-store',
    });

    if (!res.ok) return null;

    return res.json();
  } catch {
    return null;
  }
}

export default async function AnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const data = await getAnimeData(slug);

  if (!data) {
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
      <main className='min-h-screen p-6 bg-background dark:bg-dark'>
        <div className='max-w-7xl mx-auto mt-12'>
          <div className='p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4'>
            <Info className='w-8 h-8 text-blue-600 dark:text-blue-400' />
            <h1 className='text-2xl font-bold text-blue-800 dark:text-blue-200'>
              No Anime Available
            </h1>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className='min-h-screen p-6 bg-background dark:bg-dark'>
      <div className='max-w-7xl mx-auto space-y-8'>
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
            <CheckCircle className='w-8 h-8 text-blue-600 dark:text-blue-400' />
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
