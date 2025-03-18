'use client';

import React, { useState, useEffect } from 'react';
import useSWR from 'swr';
import { BaseUrl } from '@/lib/url';
import Image from 'next/image';
// import Link from 'next/link';
import { BackgroundGradient } from '@/components/background/background-gradient';
import CardA from '@/components/card/MediaCard';
// import ButtonA from '@/components/button/ScrollButton';
import Loading from './loading';

interface Genre {
  name: string;
  slug: string;
  anime_url: string;
}

interface Episode {
  episode: string;
  slug: string;
  href: string;
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
    downloads: DownloadResolution[];
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
      ? `${BaseUrl}/api/anime2/detail/${resolvedParams.slug}`
      : null,
    fetcher
  );

  if (error)
    return (
      <p className='text-red-500 text-center'>Failed to load anime data</p>
    );
  if (!anime || !resolvedParams) return <Loading />;

  // Group episodes by episode number
  const groupedEpisodes = anime.data.episode_lists.reduce((acc, episode) => {
    const match = episode.slug.match(/_(\d+)_/);
    const episodeNumber = match ? match[1] : 'Unknown';
    if (!acc[episodeNumber]) {
      acc[episodeNumber] = [];
    }
    acc[episodeNumber].push(episode);
    return acc;
  }, {} as Record<string, Episode[]>);

  return (
    <main className='p-6 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-4xl mx-auto bg-white dark:bg-dark rounded-lg shadow-lg'>
        <BackgroundGradient className='rounded-[22px] p-7 bg-white dark:bg-zinc-900'>
          <div className='flex flex-col md:flex-row items-center md:items-start'>
            {/* Bagian kiri (gambar) */}
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

            {/* Bagian kanan (konten) */}
            <div className='w-full md:w-2/3 md:pl-6'>
              {/* Judul dan informasi utama */}
              <h1 className='text-3xl font-bold mb-4 text-primary-dark dark:text-primary'>
                {anime.data.title}
              </h1>
              {anime.data.alternative_title && (
                <p className='text-xl text-gray-600 dark:text-gray-300 mb-4'>
                  {anime.data.alternative_title}
                </p>
              )}

              {/* Informasi detail */}
              <div className='text-gray-800 dark:text-gray-200 mb-4'>
                <div className='grid grid-cols-2 gap-4 mb-6'>
                  {[
                    { label: 'Type', value: anime.data.type },
                    { label: 'Status', value: anime.data.status },
                    { label: 'Release Date', value: anime.data.release_date },
                    { label: 'Studio', value: anime.data.studio },
                  ].map((detail) => (
                    <div key={detail.label} className='bg-gray-100 dark:bg-gray-800 p-3 rounded-lg'>
                      <strong className='block text-sm text-gray-500 dark:text-gray-400'>
                        {detail.label}
                      </strong>
                      <span className='text-primary-dark dark:text-primary'>
                        {detail.value || 'N/A'}
                      </span>
                    </div>
                  ))}
                </div>

                <div className='mb-6'>
                  <strong>Genres:</strong>{' '}
                  <div className='flex flex-wrap gap-2 mt-2'>
                    {anime.data.genres.map((genre) => (
                      <span
                        key={genre.slug}
                        className='px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-100 rounded-full text-sm'
                      >
                        {genre.name}
                      </span>
                    ))}
                  </div>
                </div>

                <div className='mb-6'>
                  <strong>Synopsis:</strong>
                  <p className='mt-2 text-gray-600 dark:text-gray-300'>
                    {anime.data.synopsis || 'N/A'}
                  </p>
                </div>
              </div>

              {/* Batch Downloads Section */}
              {anime.data.downloads.length > 0 && (
                <div className='mt-6'>
                  <h2 className='text-2xl font-semibold mb-4 text-primary-dark dark:text-primary'>
                    Batch Downloads
                  </h2>
                  <div className='grid grid-cols-1 gap-4'>
                    {anime.data.downloads.map((batch, index) => (
                      <div key={index} className='bg-gray-100 dark:bg-gray-800 p-4 rounded-lg'>
                        <h3 className='text-lg font-semibold mb-2'>
                          {batch.resolution}
                        </h3>
                        <div className='flex flex-wrap gap-2'>
                          {batch.links.map((link, linkIndex) => (
                            <a
                              key={linkIndex}
                              href={link.url} // Langsung menggunakan url dari data
                              target='_blank'
                              rel='noopener noreferrer'
                              className='px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors'
                            >
                              {link.name}
                            </a>
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Episode List Section */}
              {Object.keys(groupedEpisodes).length > 0 && (
                <div className='mt-6'>
                  <h2 className='text-2xl font-semibold mb-4 text-primary-dark dark:text-primary'>
                    Episode List
                  </h2>
                  <div className='space-y-4'>
                    {Object.entries(groupedEpisodes).map(([episodeNumber, resolutions]) => (
                      <div key={episodeNumber} className='bg-gray-100 dark:bg-gray-800 p-4 rounded-lg'>
                        <h3 className='text-lg font-semibold mb-2'>
                          Episode {episodeNumber}
                        </h3>
                        <div className='grid grid-cols-2 md:grid-cols-3 gap-2'>
                          {resolutions.map((resolution, resIndex) => (
                            <a
                              key={resIndex}
                              href={resolution.href} // Langsung menggunakan href dari data
                              className='px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors text-center'
                              target='_blank'
                              rel='noopener noreferrer'
                            >
                              {resolution.episode}
                            </a>
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Recommendations Section */}
              {anime.data.recommendations?.length > 0 && (
                <div className='mt-6'>
                  <h2 className='text-2xl font-semibold mb-4 text-primary-dark dark:text-primary'>
                    Recommendations
                  </h2>
                  <div className='flex overflow-x-auto pb-4 gap-4'>
                    {anime.data.recommendations.map((recommendation) => (
                      <a 
                        key={recommendation.slug}
                        href={`https://alqanime.net/${recommendation.slug}`} // Contoh penggunaan URL eksternal
                        className='flex-shrink-0 w-64'
                        target='_blank'
                        rel='noopener noreferrer'
                      >
                        <CardA
                          title={recommendation.title}
                          imageUrl={recommendation.poster}
                          linkUrl={`/anime2/detail/${recommendation.slug}`}
                          badge={recommendation.type}
                        />
                      </a>
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
