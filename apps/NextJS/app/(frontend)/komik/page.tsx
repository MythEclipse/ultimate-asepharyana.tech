import React from 'react';
import Link from 'next/link';
import { BaseUrl } from '@/lib/url';
import ButtonA from '@/components/button/ScrollButton';
import { ComicCard } from '@/components/card/ComicCard';
export const dynamic = 'force-dynamic';
export interface Comic {
  komik_id: string;
  title: string;
  image: string;
  chapter: string;
  score: string;
  type: string;
  date: string;
}

const fetchComics = async (type: string): Promise<Comic[]> => {
  const res = await fetch(`${BaseUrl}/api/komik/${type}?page=1&order=update`, {
    next: {
      revalidate: 60,
    },
  });
  if (!res.ok) {
    throw new Error(`Failed to fetch ${type}`);
  }
  const data = await res.json();
  return data.data || [];
};

export default async function HomePage() {
  let manga: Comic[] = [];
  let manhua: Comic[] = [];
  let manhwa: Comic[] = [];
  let error = false;

  try {
    const [mangaData, manhuaData, manhwaData] = await Promise.all([
      fetchComics('manga'),
      fetchComics('manhua'),
      fetchComics('manhwa'),
    ]);
    manga = mangaData;
    manhua = manhuaData;
    manhwa = manhwaData;
  } catch (err) {
    error = true;
    console.error('Error fetching comics data:', err);
  }

  return (
    <div className='p-3'>
      <h1 className='text-3xl font-bold mb-6 dark:text-white'>
        Komik Manga, Manhua, dan Manhwa
      </h1>
      <div className='space-y-8'>
        {['Manga', 'Manhua', 'Manhwa'].map((type) => (
          <section key={type} className='mb-8'>
            {!error && (
              <Link scroll href={`/komik/${type.toLowerCase()}/page/1`}>
                <ButtonA className='w-full max-w-[800rem] text-center py-4 px-8'>
                  {type}
                </ButtonA>
              </Link>
            )}
            <div className='flex flex-col items-center p-4'>
              <div className='grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-4'>
                {error ? (
                  <p className='text-gray-600 dark:text-white'>
                    Error fetching data
                  </p>
                ) : type === 'Manga' && manga.length > 0 ? (
                  manga.map((comic) => (
                    <ComicCard key={comic.komik_id} comic={comic} />
                  ))
                ) : type === 'Manhua' && manhua.length > 0 ? (
                  manhua.map((comic) => (
                    <ComicCard key={comic.komik_id} comic={comic} />
                  ))
                ) : type === 'Manhwa' && manhwa.length > 0 ? (
                  manhwa.map((comic) => (
                    <ComicCard key={comic.komik_id} comic={comic} />
                  ))
                ) : (
                  <p className='text-gray-600 dark:text-white'>
                    No {type.toLowerCase()} available
                  </p>
                )}
              </div>
            </div>
          </section>
        ))}
      </div>
    </div>
  );
}
