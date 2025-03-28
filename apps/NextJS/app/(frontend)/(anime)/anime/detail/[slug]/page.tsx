'use client';

import React, { useState, useEffect, use } from 'react';
import useSWR from 'swr';
import { PRODUCTION } from '@/lib/url';
import Image from 'next/image';
import { BackgroundGradient } from '@/components/background/background-gradient';
import CardA from '@/components/card/MediaCard';
import ButtonA from '@/components/button/ScrollButton';
import {
  Bookmark,
  Type,
  CircleDot,
  Calendar,
  Video,
  ArrowRight,
  Film,
  Popcorn,
} from 'lucide-react';
import { useTransitionRouter } from 'next-view-transitions';
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

export default function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = use(params);
  const router = useTransitionRouter()
  const { data: anime, error } = useSWR<AnimeData>(
    slug ? `/api/anime/detail/${slug}` : null,
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
    if (anime?.data.episode_lists) {
      anime.data.episode_lists.forEach((episode) => {
        router.prefetch(`/anime/full/${episode.slug}`);
      });
    }
  }, [anime, router]);
  useEffect(() => {
    if (typeof window !== 'undefined') {
      const bookmarks = JSON.parse(
        localStorage.getItem('bookmarks-anime') || '[]'
      );
      setBookmarked(
        bookmarks.some((item: { slug: string }) => item.slug === slug)
      );
    }
  }, [slug]);

  const handleBookmark = () => {
    let bookmarks = JSON.parse(localStorage.getItem('bookmarks-anime') || '[]');
    if (bookmarked) {
      bookmarks = bookmarks.filter(
        (item: { slug: string }) => item.slug !== slug
      );
    } else {
      bookmarks.push({
        slug,
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
    if (!anime) {
      return (
        <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
          <div className='max-w-6xl mx-auto bg-white dark:bg-dark-foreground rounded-3xl shadow-2xl dark:shadow-none'>
            <div className='rounded-[24px] p-6 md:p-10 bg-white dark:bg-zinc-900'>
              <div className='flex flex-col md:flex-row items-center md:items-start gap-8'>
                {/* Skeleton - Cover Image Section */}
                <div className='w-full md:w-1/3 flex flex-col gap-4'>
                  <div className='bg-zinc-200 dark:bg-zinc-700 rounded-2xl w-full aspect-[2/3] animate-pulse' />
                  <div className='h-12 bg-zinc-200 dark:bg-zinc-700 rounded-full animate-pulse' />
                </div>
    
                {/* Skeleton - Content Section */}
                <div className='w-full md:w-2/3 space-y-6'>
                  {/* Title Skeleton */}
                  <div className='h-10 bg-zinc-200 dark:bg-zinc-700 rounded-full w-3/4 animate-pulse' />
    
                  {/* Metadata Grid Skeleton */}
                  <div className='grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-zinc-50 dark:bg-zinc-800 rounded-xl'>
                    {[...Array(4)].map((_, i) => (
                      <div key={i} className='flex items-center gap-3'>
                        <div className='w-10 h-10 bg-zinc-300 dark:bg-zinc-600 rounded-lg animate-pulse' />
                        <div className='space-y-2'>
                          <div className='h-4 bg-zinc-300 dark:bg-zinc-600 rounded w-16 animate-pulse' />
                          <div className='h-4 bg-zinc-300 dark:bg-zinc-600 rounded w-24 animate-pulse' />
                        </div>
                      </div>
                    ))}
                  </div>
    
                  {/* Genres Skeleton */}
                  <div className='flex flex-wrap gap-2'>
                    {[...Array(3)].map((_, i) => (
                      <div
                        key={i}
                        className='h-8 bg-zinc-200 dark:bg-zinc-700 rounded-full w-24 animate-pulse'
                      />
                    ))}
                  </div>
    
                  {/* Synopsis Skeleton */}
                  <div className='space-y-3'>
                    <div className='h-6 bg-zinc-200 dark:bg-zinc-700 rounded w-32 animate-pulse' />
                    {[...Array(4)].map((_, i) => (
                      <div
                        key={i}
                        className='h-4 bg-zinc-200 dark:bg-zinc-700 rounded w-full animate-pulse'
                      />
                    ))}
                  </div>
    
                  {/* Episodes Skeleton */}
                  <div className='space-y-4'>
                    <div className='h-8 bg-zinc-200 dark:bg-zinc-700 rounded w-48 animate-pulse' />
                    <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-1'>
                      {[...Array(6)].map((_, i) => (
                        <div
                          key={i}
                          className='h-16 bg-zinc-100 dark:bg-zinc-800 rounded-lg animate-pulse'
                        />
                      ))}
                    </div>
                  </div>
    
                  {/* Recommendations Skeleton */}
                  <div className='space-y-4'>
                    <div className='h-8 bg-zinc-200 dark:bg-zinc-700 rounded w-48 animate-pulse' />
                    <div className='flex overflow-x-auto pb-4 gap-4'>
                      {[...Array(4)].map((_, i) => (
                        <div
                          key={i}
                          className='flex-shrink-0 w-48 md:w-56 space-y-2'
                        >
                          <div className='bg-zinc-200 dark:bg-zinc-700 aspect-[2/3] rounded-xl animate-pulse' />
                          <div className='h-4 bg-zinc-200 dark:bg-zinc-700 rounded w-3/4 animate-pulse' />
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </main>
      );
    }

  const episodes = anime.data.episode_lists || [];

  console.log('Episodes Data:', episodes);

  const fallback = 'default.png';
  const imageSources = [
    anime.data.poster && anime.data.poster.trim()
      ? anime.data.poster
      : fallback,
    anime.data.poster && anime.data.poster.trim()
      ? `https://imagecdn.app/v1/images/${encodeURIComponent(anime.data.poster)}`
      : null,
    anime.data.poster && anime.data.poster.trim()
      ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(anime.data.poster)}`
      : null,
  ].filter((src) => src && src.trim()) as string[];

  const handleError = () => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex(currentIndex + 1);
    }
  };

  return (
    <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-6xl mx-auto bg-white dark:bg-dark-foreground rounded-3xl shadow-2xl dark:shadow-none'>
        <BackgroundGradient className='rounded-[24px] p-6 md:p-10 bg-white dark:bg-zinc-900'>
          <div className='flex flex-col md:flex-row items-center md:items-start gap-8'>
            {/* Cover Image Section */}
            <div className='w-full md:w-1/3 flex flex-col gap-4'>
              <div className='relative group overflow-hidden rounded-2xl shadow-xl hover:shadow-2xl transition-shadow'>
                <Image
                  src={imageSources[currentIndex]}
                  alt={anime.data.title}
                  width={400}
                  height={600}
                  className='object-cover w-full aspect-[2/3] transform transition-transform hover:scale-105'
                  priority
                  onError={handleError}
                />
                <div className='absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity' />
              </div>

              <button
                onClick={handleBookmark}
                className={`flex items-center justify-center gap-2 px-6 py-3 rounded-full font-semibold transition-all ${
                  bookmarked
                    ? 'bg-red-500/90 hover:bg-red-600 text-white shadow-md'
                    : 'bg-green-500/90 hover:bg-green-600 text-white shadow-md'
                }`}
              >
                <Bookmark className='w-5 h-5' />
                {bookmarked ? 'Bookmarked' : 'Bookmark Now'}
              </button>
            </div>

            {/* Content Section */}
            <div className='w-full md:w-2/3 space-y-6'>
              <h1 className='text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
                {anime.data.title}
              </h1>

              {/* Metadata Grid */}
              <div className='grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-zinc-50 dark:bg-zinc-800 rounded-xl'>
                {[
                  {
                    label: 'Type',
                    value: anime.data.type,
                    icon: <Type className='w-5 h-5 text-blue-500' />,
                  },
                  {
                    label: 'Status',
                    value: anime.data.status,
                    icon: <CircleDot className='w-5 h-5 text-green-500' />,
                  },
                  {
                    label: 'Released',
                    value: anime.data.release_date,
                    icon: <Calendar className='w-5 h-5 text-red-500' />,
                  },
                  {
                    label: 'Studio',
                    value: anime.data.studio,
                    icon: <Video className='w-5 h-5 text-purple-500' />,
                  },
                ].map((detail) => (
                  <div key={detail.label} className='flex items-center gap-3'>
                    <span className='p-2 bg-white dark:bg-zinc-700 rounded-lg'>
                      {detail.icon}
                    </span>
                    <div>
                      <p className='text-sm text-zinc-500'>{detail.label}</p>
                      <p className='font-medium dark:text-zinc-200'>
                        {detail.value || 'N/A'}
                      </p>
                    </div>
                  </div>
                ))}
              </div>

              {/* Genres */}
              <div className='flex flex-wrap gap-2'>
                {anime.data.genres?.map((genre) => (
                  <span
                    key={genre.name}
                    className='px-3 py-1 text-sm bg-blue-100 dark:bg-blue-900/50 text-blue-800 dark:text-blue-200 rounded-full'
                  >
                    {genre.name}
                  </span>
                ))}
              </div>

              {/* Synopsis */}
              <div className='prose dark:prose-invert max-w-none'>
                <h3 className='text-xl font-semibold mb-2 text-zinc-800 dark:text-zinc-100'>
                  Synopsis
                </h3>
                <p className='text-zinc-600 dark:text-zinc-300 leading-relaxed'>
                  {anime.data.synopsis || 'No synopsis available.'}
                </p>
              </div>

              {/* Episodes Section */}
              <div className='space-y-4'>
                <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                  Episodes
                </h2>
                <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-1'>
                  {episodes.length > 0 ? (
                    episodes.map((episode) => {
                      const episodeNumber =
                        episode.episode.match(/Episode (\d+)/i)?.[1] ||
                        episode.episode;
                      return (
                       
                          <ButtonA onClick={()=> router.push(`/anime/full/${episode.slug}`)} key={episode.slug} className='group flex items-center justify-between p-6 bg-white dark:bg-zinc-800 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-700 transition-colors shadow-sm w-full'>
                            <span className='font-medium text-zinc-700 dark:text-zinc-200 truncate text-lg'>
                              Episode {episodeNumber}
                            </span>
                            <ArrowRight className='w-6 h-6 text-zinc-400 group-hover:text-blue-500 transition-colors' />
                          </ButtonA>
                      );
                    })
                  ) : (
                    <div className='col-span-full py-6 text-center text-zinc-500 dark:text-zinc-400'>
                      <Film className='mx-auto h-12 w-12 mb-3' />
                      No episodes available
                    </div>
                  )}
                </div>
              </div>

              {/* Recommendations Section */}
              <div className='space-y-4'>
                <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                  Recommendations
                </h2>
                <div className='flex overflow-x-auto pb-4 gap-4 scrollbar-thin scrollbar-thumb-zinc-300 scrollbar-track-transparent dark:scrollbar-thumb-zinc-600'>
                  {anime.data.recommendations?.length > 0 ? (
                    anime.data.recommendations.map((recommendation) => (
                      <div
                        key={recommendation.slug}
                        className='flex-shrink-0 w-48 md:w-56'
                      >
                        <CardA
                          title={recommendation.title}
                          imageUrl={recommendation.poster}
                          linkUrl={`/anime/detail/${recommendation.slug}`}
                        />
                      </div>
                    ))
                  ) : (
                    <div className='w-full py-6 text-center text-zinc-500 dark:text-zinc-400'>
                      <Popcorn className='mx-auto h-12 w-12 mb-3' />
                      No recommendations available
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        </BackgroundGradient>
      </div>
    </main>
  );
}
