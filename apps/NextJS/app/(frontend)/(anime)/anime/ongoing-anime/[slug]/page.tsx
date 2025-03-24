import Link from 'next/link';
import AnimeGrid from '@/components/card/AnimeGrid';
import { BaseUrl } from '@/lib/url';
import ButtonA from '@/components/button/ScrollButton';

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

async function getOngoingAnime(slug: string): Promise<OngoingAnimeData | null> {
  try {
    const res = await fetch(`${BaseUrl}/api/anime/ongoing-anime/${slug}`, {
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
  const data = await getOngoingAnime(slug);

  if (!data) {
    return (
      <main className='p-6'>
        <h1 className='text-2xl font-bold mt-8 mb-4'>Error Loading Data</h1>
        <p>Could not fetch data from the API. Please try again later.</p>
      </main>
    );
  }

  if (!Array.isArray(data.data)) {
    return (
      <main className='p-6'>
        <h1 className='text-2xl font-bold mt-8 mb-4'>No Data Available</h1>
      </main>
    );
  }

  return (
    <main className='p-6'>
      <h1 className='dark:text-lighta text-2xl font-bold mt-8 mb-4'>
        Ongoing Anime
      </h1>
      <AnimeGrid animes={data.data} />
      <PaginationComponent pagination={data.pagination} />
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  return (
    <div className='flex justify-between mt-8'>
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <div className='text-2xl font-bold mt-8 mb-4'>
          <Link
            scroll
            href={`/anime/ongoing-anime/${pagination.previous_page}`}
            className='text-blue-600 hover:underline'
          >
            <ButtonA>Previous</ButtonA>
          </Link>
        </div>
      )}
      {pagination.has_next_page && pagination.next_page !== null && (
        <div className='text-2xl font-bold mt-8 mb-4'>
          <Link
            scroll
            href={`/anime/ongoing-anime/${pagination.next_page}`}
            className='text-blue-600 hover:underline'
          >
            <ButtonA>Next</ButtonA>
          </Link>
        </div>
      )}
    </div>
  );
};
