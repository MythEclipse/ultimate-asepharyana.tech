'use client';

import React, { useState, useEffect } from 'react';
import useSWR from 'swr';
import { BaseUrl } from '@/lib/url';
import Image from 'next/image';
import { BackgroundGradient } from '@/components/background/background-gradient';
import CardA from '@/components/card/MediaCard';
import ButtonA from '@/components/button/ScrollButton';
import Loading from './loading';

interface Genre {
  name: string;
}

interface Chapter {
  chapter: string;
}

interface Recommendation {
  slug: string;
  title: string;
  image: string;
}

interface MangaData {
  title: string;
  alternativeTitle: string;
  score: string;
  image: string;
  type: string;
  status: string;
  releaseDate: string;
  author: string;
  description: string;
  genres: Genre[];
  chapters: Chapter[];
  recommendations: Recommendation[];
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailMangaPage({
  params,
}: {
  params: Promise<{ komikId: string }>;
}) {
  const [resolvedParams, setResolvedParams] = useState<{ komikId: string } | null>(null);

  useEffect(() => {
    params.then(setResolvedParams);
  }, [params]);

  const { data: manga, error } = useSWR<MangaData>(
    resolvedParams ? `${BaseUrl}/api/komik/detail?komik_id=${resolvedParams.komikId}` : null,
    fetcher
  );

  const [bookmarked, setBookmarked] = useState(false);

  useEffect(() => {
    if (resolvedParams && typeof window !== 'undefined') {
      const bookmarks = JSON.parse(localStorage.getItem('bookmarks-komik') || '[]');
      setBookmarked(bookmarks.some((item: { slug: string }) => item.slug === resolvedParams.komikId));
    }
  }, [resolvedParams]);

  const handleBookmark = () => {
    if (!resolvedParams) return;

    let bookmarks = JSON.parse(localStorage.getItem('bookmarks-komik') || '[]');

    if (bookmarked) {
      bookmarks = bookmarks.filter((item: { slug: string }) => item.slug !== resolvedParams.komikId);
    } else {
      bookmarks.push({
        slug: resolvedParams.komikId,
        title: manga?.title,
        poster: manga?.image,
      });
    }

    localStorage.setItem('bookmarks-komik', JSON.stringify(bookmarks));
    setBookmarked(!bookmarked);
  };

  if (error) return <p className='text-red-500 text-center'>Failed to load manga data</p>;
  if (!manga || !resolvedParams) return <Loading />;

  return (
    <main className='p-6 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-4xl mx-auto bg-white dark:bg-dark rounded-lg shadow-lg'>
        <BackgroundGradient className='rounded-[22px] p-7 bg-white dark:bg-zinc-900'>
          <div className='flex flex-col md:flex-row items-center md:items-start'>
            <div className='w-full md:w-1/3 mb-6 md:mb-0 flex justify-center md:justify-start'>
              <Image
                src={manga.image}
                alt={manga.title}
                width={330}
                height={450}
                className='object-cover rounded-lg shadow-md'
                priority
              />
            </div>
            <div className='w-full md:w-2/3 md:pl-6'>
              <h1 className='text-3xl font-bold mb-4 text-primary-dark dark:text-primary'>{manga.title}</h1>
              <button
                onClick={handleBookmark}
                className={`px-4 py-2 rounded text-white ${bookmarked ? 'bg-red-500' : 'bg-blue-500'}`}
              >
                {bookmarked ? 'Unbookmark' : 'Bookmark'}
              </button>
              <div className='text-gray-800 dark:text-gray-200 mb-4 mt-4'>
                {[
                  { label: 'Score', value: manga.score },
                  { label: 'Type', value: manga.type },
                  { label: 'Status', value: manga.status },
                  { label: 'Release Date', value: manga.releaseDate },
                  { label: 'Author', value: manga.author },
                ].map((detail) => (
                  <p className='mb-2' key={detail.label}>
                    <strong>{detail.label}:</strong> {detail.value || 'N/A'}
                  </p>
                ))}

                <p className='mb-4'>
                  <strong>Genres:</strong> {manga.genres?.length ? manga.genres.join(', ') : 'N/A'}
                </p>
                <p className='mb-4'>
                  <strong>Description:</strong> {manga.description || 'N/A'}
                </p>
              </div>

              <div className='mt-6'>
                <h2 className='text-2xl font-semibold mb-2 text-primary-dark dark:text-primary'>Chapters</h2>
                <div className='grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-3 gap-4'>
                  {manga.chapters?.length > 0 ? (
                    manga.chapters.map((chapter, index) => (
                      <ButtonA key={index} className='w-full'>
                        <span className='text-lg font-bold mb-1 text-center truncate text-primary-dark dark:text-primary'>
                          {chapter.chapter}
                        </span>
                      </ButtonA>
                    ))
                  ) : (
                    <p className='col-span-full text-center text-primary-dark dark:text-primary'>
                      No chapters available
                    </p>
                  )}
                </div>
              </div>

              <div className='mt-6'>
                <h2 className='text-2xl font-semibold mb-2 text-primary-dark dark:text-primary'>Recommendations</h2>
                <div className='overflow-x-auto'>
                  <div className='flex space-x-4'>
                    {manga.recommendations && manga.recommendations.length > 0 ? (
                      manga.recommendations.map((recommendation) => (
                        <div key={recommendation.slug} className='flex-shrink-0 w-64'>
                          <CardA
                            title={recommendation.title}
                            imageUrl={recommendation.image}
                            linkUrl={`/komik/detail/${recommendation.slug}`}
                          />
                        </div>
                      ))
                    ) : (
                      <p className='col-span-full text-center text-primary-dark dark:text-primary'>
                        No recommendations available
                      </p>
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </BackgroundGradient>
      </div>
    </main>
  );
}
