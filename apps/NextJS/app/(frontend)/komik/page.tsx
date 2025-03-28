import React from 'react';
import { Link } from 'next-view-transitions';
import { BaseUrl } from '@/lib/url';
import { ComicCard } from '@/components/card/ComicCard';
import { BookOpen, AlertTriangle, Info, ArrowRight } from 'lucide-react';
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
    <main className='min-h-screen p-6 bg-background dark:bg-dark'>
      <div className='max-w-7xl mx-auto space-y-12'>
        {/* Header Utama */}
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-purple-100 dark:bg-purple-900/50 rounded-xl'>
            <BookOpen className='w-8 h-8 text-purple-600 dark:text-purple-400' />
          </div>
          <h1 className='text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent'>
            Komik Catalog
          </h1>
        </div>

        {error ? (
          <div className='p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex items-center gap-4'>
            <AlertTriangle className='w-8 h-8 text-red-600 dark:text-red-400' />
            <div>
              <h2 className='text-xl font-medium text-red-800 dark:text-red-200 mb-2'>
                Error Loading Data
              </h2>
              <p className='text-red-700 dark:text-red-300'>
                Failed to fetch comic data. Please try again later.
              </p>
            </div>
          </div>
        ) : (
          <div className='space-y-12'>
            {['Manga', 'Manhua', 'Manhwa'].map((type) => {
              const comics = {
                Manga: manga,
                Manhua: manhua,
                Manhwa: manhwa,
              }[type];

              return (
                <section key={type} className='mb-12 space-y-6'>
                  <div className='flex items-center justify-between mb-6'>
                    <div className='flex items-center gap-3'>
                      <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
                        <BookOpen className='w-6 h-6 text-blue-600 dark:text-blue-400' />
                      </div>
                      <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                        {type}
                      </h2>
                    </div>
                    <Link
                      href={`/komik/${type.toLowerCase()}/page/1`}
                      className='flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors'
                    >
                      View All
                      <ArrowRight className='w-4 h-4' />
                    </Link>
                  </div>

                  {(comics ?? []).length > 0 ? (
                    <div className='grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4'>
                      {(comics ?? []).map((comic) => (
                        <ComicCard key={comic.komik_id} comic={comic} />
                      ))}
                    </div>
                  ) : (
                    <div className='p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4'>
                      <Info className='w-8 h-8 text-blue-600 dark:text-blue-400' />
                      <h3 className='text-lg font-medium text-blue-800 dark:text-blue-200'>
                        No {type} available at the moment
                      </h3>
                    </div>
                  )}
                </section>
              );
            })}
          </div>
        )}
      </div>
    </main>
  );
}
