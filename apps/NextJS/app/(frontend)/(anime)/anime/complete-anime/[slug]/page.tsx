import AnimeGrid from '@/components/card/AnimeGrid';
import Link from 'next/link';
import { BaseUrl } from '@/lib/url';
import ButtonA from '@/components/button/ScrollButton';
export const dynamic = 'force-dynamic';
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

async function getAnimeData(slug: string): Promise<CompleteAnimeData | null> {
  try {
    const res = await fetch(`${BaseUrl}/api/anime/complete-anime/${slug}`, {
      cache: 'no-store',
    });
    if (!res.ok) throw new Error('Failed to fetch');
    return res.json();
  } catch (error) {
    console.error('Error fetching anime data:', error);
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
        Currently Finished Anime
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
            href={`/anime/complete-anime/${pagination.previous_page}`}
            className='text-blue-600 hover:underline'
          >
            <ButtonA>Previous</ButtonA>
          </Link>
        </div>
      )}
      {pagination.has_next_page && pagination.next_page !== null && (
        <div className='text-2xl font-bold mt-8 mb-4'>
          <Link
            href={`/anime/complete-anime/${pagination.next_page}`}
            className='text-blue-600 hover:underline'
          >
            <ButtonA>Next</ButtonA>
          </Link>
        </div>
      )}
    </div>
  );
};
