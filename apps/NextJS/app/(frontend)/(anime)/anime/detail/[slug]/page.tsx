'use client';

import React, { useState, useEffect } from 'react';
import useSWR from 'swr';
import { BaseUrl } from '@/lib/url';
import Image from 'next/image';
import Link from 'next/link';
import { BackgroundGradient } from '@/components/background/background-gradient';
import CardA from '@/components/card/MediaCard';
import ButtonA from '@/components/button/ScrollButton';
import Loading from './loading';

interface Genre {
  name: string;
  slug: string;
  anime_url: string;
}

interface Episode {
  episode: string;
  slug: string;
}

interface Recommendation {
  slug: string;
  title: string;
  poster: string;
}

interface AnimeData {
  status: string;
  data: {
    title: string;
    alternative_title: string;
    poster: string;
    type: string;
    status: string;
    release_date: string;
    studio: string;
    synopsis: string;
    genres: Genre[];
    producers: string[];
    episode_lists: Episode[];
    batch: Episode[];
    recommendations: Recommendation[];
  };
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const [resolvedParams, setResolvedParams] = useState<{ slug: string } | null>(
    null
  );

  useEffect(() => {
    params.then(setResolvedParams);
  }, [params]);

  const { data: anime, error } = useSWR<AnimeData>(
    resolvedParams
      ? `${BaseUrl}/api/anime/detail/${resolvedParams.slug}`
      : null,
    fetcher
  );

  const [bookmarked, setBookmarked] = useState(false);

  useEffect(() => {
    if (resolvedParams && typeof window !== 'undefined') {
      const bookmarks = JSON.parse(
        localStorage.getItem('bookmarks-anime') || '[]'
      );
      setBookmarked(
        bookmarks.some(
          (item: { slug: string }) => item.slug === resolvedParams.slug
        )
      );
    }
  }, [resolvedParams]);

  const handleBookmark = () => {
    if (!resolvedParams) return;

    let bookmarks = JSON.parse(localStorage.getItem('bookmarks-anime') || '[]');

    if (bookmarked) {
      bookmarks = bookmarks.filter(
        (item: { slug: string }) => item.slug !== resolvedParams.slug
      );
    } else {
      bookmarks.push({
        slug: resolvedParams.slug,
        title: anime?.data.title,
        poster: anime?.data.poster,
      });
    }

    localStorage.setItem('bookmarks-anime', JSON.stringify(bookmarks));
    setBookmarked(!bookmarked);
  };

  if (error)
    return (
      <p className='text-red-500 text-center'>Failed to load anime data</p>
    );
  if (!anime || !resolvedParams) return <Loading />;

  return (
    <main className='p-6 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-4xl mx-auto bg-white dark:bg-dark rounded-lg shadow-lg'>
        <BackgroundGradient className='rounded-[22px] p-7 bg-white dark:bg-zinc-900'>
          <div className='flex flex-col md:flex-row items-center md:items-start'>
            <div className='w-full md:w-1/3 mb-6 md:mb-0 flex justify-center md:justify-start'>
              <Image
                src={anime.data.poster}
                alt={anime.data.title}
                width={330}
                height={450}
                className='object-cover rounded-lg shadow-md'
                priority
              />
            </div>
            <div className='w-full md:w-2/3 md:pl-6'>
              <h1 className='text-3xl font-bold mb-4 text-primary-dark dark:text-primary'>
                {anime.data.title}
              </h1>
              <button
                onClick={handleBookmark}
                className={`px-4 py-2 rounded text-white ${
                  bookmarked ? 'bg-red-500' : 'bg-blue-500'
                }`}
              >
                {bookmarked ? 'Unbookmark' : 'Bookmark'}
              </button>
              <div className='text-gray-800 dark:text-gray-200 mb-4 mt-4'>
                {[
                  { label: 'Type', value: anime.data.type },
                  { label: 'Status', value: anime.data.status },
                  { label: 'Release Date', value: anime.data.release_date },
                  { label: 'Studio', value: anime.data.studio },
                ].map((detail) => (
                  <p className='mb-2' key={detail.label}>
                    <strong>{detail.label}:</strong> {detail.value || 'N/A'}
                  </p>
                ))}

                <p className='mb-4'>
                  <strong>Genres:</strong>{' '}
                  {anime.data.genres
                    ? anime.data.genres.map((genre) => genre.name).join(', ')
                    : 'N/A'}
                </p>
                <p className='mb-4'>
                  <strong>Synopsis:</strong> {anime.data.synopsis || 'N/A'}
                </p>
              </div>

              <div className='mt-6'>
                <h2 className='text-2xl font-semibold mb-2 text-primary-dark dark:text-primary'>
                  Episodes
                </h2>
                <div className='grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-3 gap-4'>
                  {anime.data.episode_lists?.length > 0 ? (
                    anime.data.episode_lists.map((episode) => {
                      const episodeNumber =
                        episode.episode.match(/Episode (\d+)/)?.[1] ||
                        episode.episode;
                      return (
                        <Link
                          scroll
                          key={episode.slug}
                          href={`/anime/full/${episode.slug}`}
                          className=''
                        >
                          <ButtonA className='w-full'>
                            <span className='text-lg font-bold mb-1 text-center truncate text-primary-dark dark:text-primary'>
                              Episode {episodeNumber}
                            </span>
                          </ButtonA>
                        </Link>
                      );
                    })
                  ) : (
                    <p className='col-span-full text-center text-primary-dark dark:text-primary'>
                      No episodes available
                    </p>
                  )}
                </div>
              </div>

              <div className='mt-6'>
                <h2 className='text-2xl font-semibold mb-2 text-primary-dark dark:text-primary'>
                  Recommendations
                </h2>
                <div className='overflow-x-auto'>
                  <div className='flex space-x-4'>
                    {anime.data.recommendations &&
                    anime.data.recommendations.length > 0 ? (
                      anime.data.recommendations.map((recommendation) => (
                        <div
                          key={recommendation.slug}
                          className='flex-shrink-0 w-64'
                        >
                          <CardA
                            title={recommendation.title}
                            imageUrl={recommendation.poster}
                            linkUrl={`/anime/detail/${recommendation.slug}`}
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
