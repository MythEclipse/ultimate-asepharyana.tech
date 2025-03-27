'use client';

import React, { useState, useEffect, use } from 'react';
import useSWR from 'swr';
import { BaseUrl } from '@/lib/url';
import Image from 'next/image';
import Link from 'next/link';
import { BackgroundGradient } from '@/components/background/background-gradient';
import CardA from '@/components/card/MediaCard';
import ButtonA from '@/components/button/ScrollButton';
import Loading from './loading';

export const dynamic = 'force-dynamic';

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
    recommendations: Recommendation[];
  };
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailAnimePage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = use(params);

  const { data: anime, error } = useSWR<AnimeData>(
    slug ? `${BaseUrl}/api/anime/detail/${slug}` : null,
    fetcher,
    {
      revalidateIfStale: false,
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
      dedupingInterval: 0,
      compare: (a, b) => JSON.stringify(a) === JSON.stringify(b),
    }
  );

  const [currentIndex, setCurrentIndex] = useState(0);
  const [bookmarked, setBookmarked] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const bookmarks = JSON.parse(localStorage.getItem('bookmarks-anime') || '[]');
      setBookmarked(bookmarks.some((item: { slug: string }) => item.slug === slug));
    }
  }, [slug]);

  const handleBookmark = () => {
    let bookmarks = JSON.parse(localStorage.getItem('bookmarks-anime') || '[]');
    if (bookmarked) {
      bookmarks = bookmarks.filter((item: { slug: string }) => item.slug !== slug);
    } else {
      bookmarks.push({ slug, title: anime?.data.title, poster: anime?.data.poster });
    }
    localStorage.setItem('bookmarks-anime', JSON.stringify(bookmarks));
    setBookmarked(!bookmarked);
  };

  if (error) return <p className='text-red-500 text-center'>Failed to load anime data</p>;
  if (!anime) return <Loading />;

  const episodes = anime.data.episode_lists || [];

  console.log("Episodes Data:", episodes);

  const fallback = 'default.png';
  const imageSources = [
    anime.data.poster?.trim() ? anime.data.poster : null,
    `https://imagecdn.app/v1/images/${encodeURIComponent(anime.data.poster || fallback)}`,
    `${BaseUrl}/api/imageproxy?url=${encodeURIComponent(anime.data.poster || fallback)}`,
  ].filter(Boolean) as string[];

  const handleError = () => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex(currentIndex + 1);
    }
  };

  return (
    <main className='p-6 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-4xl mx-auto bg-white dark:bg-dark rounded-lg shadow-lg'>
        <BackgroundGradient className='rounded-[22px] p-7 bg-white dark:bg-zinc-900'>
          <div className='flex flex-col md:flex-row items-center md:items-start'>
            <div className='w-full md:w-1/3 mb-6 md:mb-0 flex justify-center md:justify-start'>
              <Image
                src={imageSources[currentIndex]}
                alt={anime.data.title}
                width={330}
                height={450}
                className='object-cover rounded-lg shadow-md'
                priority
                onError={handleError}
              />
            </div>
            <div className='w-full md:w-2/3 md:pl-6'>
              <h1 className='text-3xl font-bold mb-4 text-primary-dark dark:text-primary'>
                {anime.data.title}
              </h1>
              <button onClick={handleBookmark} className='px-4 py-2 mb-2 bg-primary text-white rounded-lg'>
                {bookmarked ? 'Remove Bookmark' : 'Add to Bookmark'}
              </button>
              <div className='text-gray-800 dark:text-gray-200 mb-4'>
                {[{ label: 'Type', value: anime.data.type },
                { label: 'Status', value: anime.data.status },
                { label: 'Release Date', value: anime.data.release_date },
                { label: 'Studio', value: anime.data.studio },
                ].map((detail) => (
                  <p key={detail.label} className='mb-2'>
                    <strong>{detail.label}:</strong> {detail.value || 'N/A'}
                  </p>
                ))}
                <p className='mb-4'>
                  <strong>Genres:</strong> {anime.data.genres?.map((genre) => genre.name).join(', ') || 'N/A'}
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
                  {episodes.length > 0 ? (
                    episodes.map((episode) => {
                      const episodeNumber = episode.episode.match(/Episode\s+(\d+)(?:\s*\(End\))?/i)?.[1] || 
                      episode.episode.replace(/Subtitle Indonesia/i, '').trim();
                      return (
                        <Link key={episode.slug} href={`/anime/full/${episode.slug}`}>
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
                    {anime.data.recommendations?.length > 0 ? (
                      anime.data.recommendations.map((recommendation) => (
                        <div key={recommendation.slug} className='flex-shrink-0 w-64'>
                          <CardA title={recommendation.title} imageUrl={recommendation.poster} linkUrl={`/anime/detail/${recommendation.slug}`} />
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
