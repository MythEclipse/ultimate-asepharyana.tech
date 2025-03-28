'use client';

import React, { useState } from 'react';
import { useParams } from 'next/navigation';
import useSWR from 'swr';
import Image from 'next/image';
import { BackgroundGradient } from '@/components/background/background-gradient';
import CardA from '@/components/card/MediaCard';
import {
  Type,
  CircleDot,
  Calendar,
  Video,
  Download,
  Film,
  ArrowUpRight,
  BookOpen,
  AlertTriangle,
} from 'lucide-react';
import { PRODUCTION } from '@/lib/url';

interface Genre {
  name: string;
  slug: string;
  anime_url: string;
}

interface DownloadLink {
  name: string;
  url: string;
}

interface DownloadResolution {
  resolution: string;
  links: DownloadLink[];
}

interface Recommendation {
  slug: string;
  title: string;
  poster: string;
  status: string;
  type: string;
}

interface AnimeData {
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
  batch: DownloadResolution[];
  downloads: DownloadResolution[];
  recommendations: Recommendation[];
}

const processDownloads = (downloads: DownloadResolution[]) => {
  const episodes: Record<string, DownloadResolution[]> = {};

  downloads?.forEach((download) => {
    let episodeNumber = 'unknown';
    for (const link of download.links) {
      const episodeMatch = link.url.match(/(?:BD|EP|_)(\d+)(?:_|\.|$)/i);
      if (episodeMatch) {
        episodeNumber = episodeMatch[1];
        break;
      }
    }
    episodes[episodeNumber] = episodes[episodeNumber] || [];
    episodes[episodeNumber].push(download);
  });

  return episodes;
};

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailAnimePage() {
  const [currentIndex, setCurrentIndex] = useState(0);
  const params = useParams();
  const slug = Array.isArray(params.slug) ? params.slug[0] : params.slug;

  const { data, error, isLoading } = useSWR<{ data: AnimeData }>(
    `/api/anime2/detail/${slug}`,
    fetcher
  );

  if (isLoading)
    return (
      <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
        <div className='max-w-6xl mx-auto bg-white dark:bg-dark-foreground rounded-3xl shadow-2xl dark:shadow-none'>
          <div className='rounded-[24px] p-6 md:p-10 bg-white dark:bg-zinc-900'>
            <div className='flex flex-col md:flex-row items-center md:items-start gap-8'>
              {/* Cover Section */}
              <div className='w-full md:w-1/3 flex flex-col gap-4'>
                <div className='relative group overflow-hidden rounded-2xl shadow-xl hover:shadow-2xl transition-shadow'>
                  <div className='w-full aspect-[2/3] bg-gray-300 animate-pulse rounded-2xl' />
                </div>

                {/* Quick Info */}
                <div className='p-4 bg-zinc-50 dark:bg-zinc-800 rounded-xl space-y-2'>
                  <div className='flex items-center gap-3'>
                    <div className='w-5 h-5 bg-gray-400 rounded' />
                    <span className='font-medium'>Tipe:</span>
                    <span className='text-zinc-700 dark:text-zinc-300'>
                      Placeholder
                    </span>
                  </div>
                  <div className='flex items-center gap-3'>
                    <div className='w-5 h-5 bg-gray-400 rounded' />
                    <span className='font-medium'>Status:</span>
                    <span className='text-zinc-700 dark:text-zinc-300'>
                      Placeholder
                    </span>
                  </div>
                </div>
              </div>

              {/* Content Section */}
              <div className='w-full md:w-2/3 space-y-6'>
                <div className='h-12 bg-gray-300 rounded-lg w-2/3 animate-pulse' />

                <div className='h-6 bg-gray-300 rounded-lg w-1/2 animate-pulse' />

                {/* Metadata Grid */}
                <div className='grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-zinc-50 dark:bg-zinc-800 rounded-xl'>
                  {[...Array(4)].map((_, i) => (
                    <div key={i} className='flex items-center gap-3'>
                      <div className='w-5 h-5 bg-gray-400 rounded' />
                      <div>
                        <div className='h-4 bg-gray-400 rounded w-16 mb-2' />
                        <div className='h-5 bg-gray-400 rounded w-20' />
                      </div>
                    </div>
                  ))}
                </div>

                {/* Genres */}
                <div className='flex flex-wrap gap-2'>
                  {[...Array(3)].map((_, i) => (
                    <div
                      key={i}
                      className='h-6 bg-gray-300 rounded-full w-20'
                    />
                  ))}
                </div>

                {/* Synopsis */}
                <div className='space-y-3'>
                  <div className='h-6 bg-gray-300 rounded-lg w-32' />
                  <div className='h-24 bg-gray-200 rounded-lg animate-pulse' />
                </div>

                {/* Batch Downloads Skeleton */}
                <div className='space-y-4'>
                  <div className='h-8 bg-gray-300 rounded-lg w-64' />
                  <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'>
                    {[...Array(3)].map((_, i) => (
                      <div
                        key={i}
                        className='h-32 bg-gray-100 dark:bg-zinc-700 rounded-xl p-4'
                      >
                        <div className='h-6 bg-gray-300 rounded-lg w-3/4 mb-4' />
                        <div className='h-8 bg-gray-200 rounded-lg mb-2' />
                      </div>
                    ))}
                  </div>
                </div>

                {/* Episode List Skeleton */}
                <div className='space-y-4'>
                  <div className='h-8 bg-gray-300 rounded-lg w-64' />
                  <div className='space-y-3'>
                    {[...Array(3)].map((_, i) => (
                      <div
                        key={i}
                        className='h-32 bg-gray-100 dark:bg-zinc-700 rounded-xl p-4'
                      >
                        <div className='h-6 bg-gray-300 rounded-lg w-3/4 mb-4' />
                        <div className='grid grid-cols-2 gap-2'>
                          {[...Array(2)].map((_, j) => (
                            <div
                              key={j}
                              className='h-8 bg-gray-200 rounded-lg'
                            />
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Recommendations Skeleton */}
                <div className='space-y-4'>
                  <div className='h-8 bg-gray-300 rounded-lg w-64' />
                  <div className='flex overflow-x-auto pb-4 gap-4'>
                    {[...Array(4)].map((_, i) => (
                      <div
                        key={i}
                        className='w-48 h-64 bg-gray-200 rounded-xl'
                      />
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    );
  if (error || !data?.data)
    return (
      <div className='min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center'>
        <div className='max-w-2xl text-center'>
          <div className='p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex flex-col items-center gap-4'>
            <AlertTriangle className='w-12 h-12 text-red-600 dark:text-red-400' />
            <h2 className='text-2xl font-bold text-red-800 dark:text-red-200'>
              Gagal Memuat Data Anime
            </h2>
            <p className='text-red-700 dark:text-red-300'>
              Silakan coba kembali atau periksa koneksi internet Anda
            </p>
          </div>
        </div>
      </div>
    );

  const anime = data.data;
  const groupedDownloads = processDownloads(anime.downloads);
  const fallback = 'https://asepharyana.cloud/default.png';
  const imageSources = [
    anime.poster && anime.poster.trim() ? anime.poster : fallback,
    anime.poster && anime.poster.trim()
      ? `https://imagecdn.app/v1/images/${encodeURIComponent(anime.poster)}`
      : null,
    anime.poster && anime.poster.trim()
      ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(anime.poster)}`
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
            {/* Cover Section */}
            <div className='w-full md:w-1/3 flex flex-col gap-4'>
              <div className='relative group overflow-hidden rounded-2xl shadow-xl hover:shadow-2xl transition-shadow'>
                <Image
                  src={imageSources[currentIndex]}
                  alt={anime.title}
                  width={400}
                  height={600}
                  className='object-cover w-full aspect-[2/3] transform transition-transform hover:scale-105'
                  priority
                  onError={handleError}
                />
                <div className='absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity' />
              </div>

              {/* Quick Info */}
              <div className='p-4 bg-zinc-50 dark:bg-zinc-800 rounded-xl space-y-2'>
                <div className='flex items-center gap-3'>
                  <BookOpen className='w-5 h-5 text-purple-600' />
                  <span className='font-medium'>Tipe:</span>
                  <span className='text-zinc-700 dark:text-zinc-300'>
                    {anime.type}
                  </span>
                </div>
                <div className='flex items-center gap-3'>
                  <CircleDot className='w-5 h-5 text-green-600' />
                  <span className='font-medium'>Status:</span>
                  <span className='text-zinc-700 dark:text-zinc-300'>
                    {anime.status}
                  </span>
                </div>
              </div>
            </div>

            {/* Content Section */}
            <div className='w-full md:w-2/3 space-y-6'>
              <h1 className='text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
                {anime.title}
              </h1>

              {anime.alternative_title && (
                <p className='text-xl text-zinc-600 dark:text-zinc-300 font-medium'>
                  {anime.alternative_title}
                </p>
              )}

              {/* Metadata Grid */}
              <div className='grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-zinc-50 dark:bg-zinc-800 rounded-xl'>
                {[
                  {
                    label: 'Type',
                    value: anime.type,
                    icon: <Type className='w-5 h-5 text-blue-500' />,
                  },
                  {
                    label: 'Status',
                    value: anime.status,
                    icon: <CircleDot className='w-5 h-5 text-green-500' />,
                  },
                  {
                    label: 'Released',
                    value: anime.release_date,
                    icon: <Calendar className='w-5 h-5 text-red-500' />,
                  },
                  {
                    label: 'Studio',
                    value: anime.studio,
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
                {anime.genres.map((genre) => (
                  <span
                    key={genre.slug}
                    className='px-3 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-800 dark:text-blue-200 rounded-full text-sm'
                  >
                    {genre.name}
                  </span>
                ))}
              </div>

              {/* Synopsis */}
              <div className='prose dark:prose-invert max-w-none'>
                <h3 className='text-xl font-semibold mb-2 text-zinc-800 dark:text-zinc-100'>
                  Sinopsis
                </h3>
                <p className='text-zinc-600 dark:text-zinc-300 leading-relaxed'>
                  {anime.synopsis || 'Tidak ada sinopsis yang tersedia.'}
                </p>
              </div>

              {/* Batch Downloads */}
              {anime.batch?.length > 0 && (
                <div className='space-y-4'>
                  <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                    Batch Downloads
                  </h2>
                  <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'>
                    {anime.batch.map((batchRes, index) => (
                      <div
                        key={index}
                        className='bg-white dark:bg-zinc-800 p-4 rounded-xl shadow-sm hover:shadow-md transition-shadow'
                      >
                        <div className='flex items-center gap-2 mb-3'>
                          <Download className='w-5 h-5 text-green-500' />
                          <h3 className='text-lg font-semibold'>
                            {batchRes.resolution}
                          </h3>
                        </div>
                        <div className='space-y-2'>
                          {batchRes.links.map((link, linkIndex) => (
                            <a
                              key={linkIndex}
                              href={link.url}
                              target='_blank'
                              rel='noopener noreferrer'
                              className='flex items-center justify-between px-4 py-2 bg-zinc-50 dark:bg-zinc-700 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-600 transition-colors'
                            >
                              <span className='truncate'>{link.name}</span>
                              <ArrowUpRight className='w-4 h-4 flex-shrink-0' />
                            </a>
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Episode List */}
              {Object.entries(groupedDownloads).length > 0 && (
                <div className='space-y-4'>
                  <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                    Daftar Episode
                  </h2>
                  <div className='space-y-3'>
                    {Object.entries(groupedDownloads)
                      .sort(([epA], [epB]) => {
                        if (epA === 'unknown' && epB === 'unknown') return 0;
                        if (epA === 'unknown') return 1;
                        if (epB === 'unknown') return -1;
                        return parseInt(epA) - parseInt(epB);
                      })
                      .map(
                        ([episodeNumber, resolutions]) =>
                          episodeNumber !== 'unknown' && (
                            <div
                              key={episodeNumber}
                              className='bg-white dark:bg-zinc-800 p-4 rounded-xl shadow-sm hover:shadow-md transition-shadow'
                            >
                              <div className='flex items-center gap-2 mb-3'>
                                <Film className='w-5 h-5 text-purple-500' />
                                <h3 className='text-lg font-semibold'>
                                  Episode {episodeNumber}
                                </h3>
                              </div>
                              <div className='grid grid-cols-2 md:grid-cols-3 gap-2'>
                                {resolutions.map((resolution, resIndex) => (
                                  <div key={resIndex} className='space-y-1'>
                                    <div className='flex items-center gap-1 text-sm text-zinc-500'>
                                      <Video className='w-4 h-4' />
                                      {resolution.resolution}
                                    </div>
                                    <div className='space-y-1'>
                                      {resolution.links.map(
                                        (link, linkIndex) => (
                                          <a
                                            key={linkIndex}
                                            href={link.url}
                                            className='flex items-center justify-between px-3 py-2 bg-zinc-50 dark:bg-zinc-700 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-600 transition-colors text-sm'
                                            target='_blank'
                                            rel='noopener noreferrer'
                                          >
                                            <span className='truncate'>
                                              {link.name}
                                            </span>
                                            <ArrowUpRight className='w-4 h-4 flex-shrink-0' />
                                          </a>
                                        )
                                      )}
                                    </div>
                                  </div>
                                ))}
                              </div>
                            </div>
                          )
                      )}
                  </div>
                </div>
              )}

              {/* Recommendations */}
              {anime.recommendations?.length > 0 && (
                <div className='space-y-4'>
                  <h2 className='text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                    Rekomendasi Serupa
                  </h2>
                  <div className='flex overflow-x-auto pb-4 gap-4 scrollbar-thin scrollbar-thumb-zinc-300 scrollbar-track-transparent dark:scrollbar-thumb-zinc-600'>
                    {anime.recommendations.map((recommendation) => (
                      <CardA
                        key={recommendation.slug}
                        title={recommendation.title}
                        imageUrl={recommendation.poster}
                        linkUrl={`/anime2/detail/${recommendation.slug}`}
                        badge={recommendation.type}
                      />
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        </BackgroundGradient>
      </div>
    </main>
  );
}
