'use client';

import React, { useState, useEffect, use } from 'react';
import useSWR from 'swr';
import Image from 'next/image';
import { useRouter } from 'next/navigation';
import { PRODUCTION } from '@/lib/url';

import { BackgroundGradient } from '@/components/background/background-gradient';
import MediaCard from '@features/anime/MediaCard';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import {
  Bookmark,
  Type,
  CircleDot,
  Calendar,
  Video,
  ArrowRight,
  Film,
  Popcorn,
  Clapperboard,
} from 'lucide-react';

export const dynamic = 'force-dynamic';

interface Genre {
  name: string;
  slug: string;
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

const DetailPageSkeleton = () => (
  <main className='p-4 md:p-8 min-h-screen'>
    <div className='max-w-6xl mx-auto'>
      <div className='rounded-[24px] p-6 md:p-10 bg-card'>
        <div className='flex flex-col md:flex-row items-start gap-8'>
          <div className='w-full md:w-1/3 flex flex-col gap-4'>
            <Skeleton className='aspect-[2/3] w-full rounded-xl' />
            <Skeleton className='h-12 w-full rounded-full' />
          </div>
          <div className='w-full md:w-2/3 space-y-6'>
            <Skeleton className='h-10 w-3/4 rounded-lg' />
            <Card>
              <CardContent className='p-4 grid grid-cols-2 md:grid-cols-4 gap-4'>
                {[...Array(4)].map((_, i) => (
                  <div key={i} className='flex items-center gap-3'>
                    <Skeleton className='w-10 h-10 rounded-lg' />
                    <div className='space-y-2'>
                      <Skeleton className='h-4 w-16' />
                      <Skeleton className='h-4 w-24' />
                    </div>
                  </div>
                ))}
              </CardContent>
            </Card>
            <div className='flex flex-wrap gap-2'>
              {[...Array(3)].map((_, i) => (
                <Skeleton key={i} className='h-8 w-24 rounded-full' />
              ))}
            </div>
            <div className='space-y-3'>
              <Skeleton className='h-6 w-32' />
              <Skeleton className='h-4 w-full' />
              <Skeleton className='h-4 w-full' />
              <Skeleton className='h-4 w-5/6' />
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
);

export default function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = use(params);
  const router = useRouter();

  const { data: anime, error } = useSWR<AnimeData>(
    slug ? `/api/anime/detail/${slug}` : null,
    fetcher,
    {
      revalidateOnFocus: false,
      dedupingInterval: 60000,
    }
  );

  const [currentImageIndex, setCurrentImageIndex] = useState(0);
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
    const isBookmarked = bookmarks.some(
      (item: { slug: string }) => item.slug === slug
    );

    if (isBookmarked) {
      bookmarks = bookmarks.filter(
        (item: { slug: string }) => item.slug !== slug
      );
    } else if (anime?.data) {
      bookmarks.push({
        slug,
        title: anime.data.title,
        poster: anime.data.poster,
      });
    }
    localStorage.setItem('bookmarks-anime', JSON.stringify(bookmarks));
    setBookmarked(!isBookmarked);
  };

  const imageSources = [
    anime?.data.poster,
    anime?.data.poster
      ? `https://imagecdn.app/v1/images/${encodeURIComponent(anime.data.poster)}`
      : null,
    anime?.data.poster
      ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(anime.data.poster)}`
      : null,
    '/default.png',
  ].filter(Boolean) as string[];

  const handleImageError = () => {
    if (currentImageIndex < imageSources.length - 1) {
      setCurrentImageIndex(currentImageIndex + 1);
    }
  };

  if (error)
    return (
      <p className='text-destructive text-center p-8'>
        Failed to load anime data.
      </p>
    );
  if (!anime) return <DetailPageSkeleton />;

  const { data } = anime;
  const metadata = [
    {
      label: 'Type',
      value: data.type,
      icon: <Type className='w-5 h-5 text-blue-500' />,
    },
    {
      label: 'Status',
      value: data.status,
      icon: <CircleDot className='w-5 h-5 text-green-500' />,
    },
    {
      label: 'Released',
      value: data.release_date,
      icon: <Calendar className='w-5 h-5 text-red-500' />,
    },
    {
      label: 'Studio',
      value: data.studio,
      icon: <Video className='w-5 h-5 text-purple-500' />,
    },
  ];

  return (
    <TooltipProvider delayDuration={100}>
      <main className='p-4 md:p-8 bg-background min-h-screen'>
        <div className='max-w-6xl mx-auto'>
          <BackgroundGradient className='rounded-[24px] p-0.5'>
            <div className='bg-card text-card-foreground rounded-[22px] p-6 md:p-10'>
              <div className='flex flex-col md:flex-row items-start gap-8'>
                <div className='w-full md:w-1/3 flex flex-col gap-4 md:sticky top-8'>
                  <Card className='overflow-hidden'>
                    <Image
                      src={imageSources[currentImageIndex]}
                      alt={data.title}
                      width={400}
                      height={600}
                      className='object-cover w-full aspect-[2/3]'
                      priority
                      onError={handleImageError}
                      unoptimized
                    />
                  </Card>
                  <Button
                    onClick={handleBookmark}
                    variant={bookmarked ? 'destructive' : 'default'}
                    size='lg'
                    className='w-full'
                  >
                    <Bookmark className='w-5 h-5 mr-2' />
                    {bookmarked ? 'Remove from Bookmarks' : 'Add to Bookmarks'}
                  </Button>
                </div>

                <div className='w-full md:w-2/3 space-y-6'>
                  <h1 className='text-4xl font-bold tracking-tight'>
                    {data.title}
                  </h1>

                  <Card>
                    <CardContent className='p-4 grid grid-cols-2 md:grid-cols-4 gap-4'>
                      {metadata.map((item) => (
                        <div
                          key={item.label}
                          className='flex items-center gap-3'
                        >
                          <span className='p-2 bg-muted rounded-lg'>
                            {item.icon}
                          </span>
                          <div>
                            <p className='text-sm text-muted-foreground'>
                              {item.label}
                            </p>
                            <p className='font-semibold'>
                              {item.value || 'N/A'}
                            </p>
                          </div>
                        </div>
                      ))}
                    </CardContent>
                  </Card>

                  <div className='flex flex-wrap gap-2'>
                    {data.genres?.map((genre) => (
                      <Badge variant='secondary' key={genre.slug}>
                        {genre.name}
                      </Badge>
                    ))}
                  </div>

                  <Card>
                    <CardHeader>
                      <CardTitle>Synopsis</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className='text-muted-foreground leading-relaxed'>
                        {data.synopsis || 'No synopsis available.'}
                      </p>
                    </CardContent>
                  </Card>

                  <Card>
                    <CardHeader>
                      <CardTitle>Episodes</CardTitle>
                      <CardDescription>
                        Total {data.episode_lists?.length || 0} episodes
                        available.
                      </CardDescription>
                    </CardHeader>
                    <CardContent>
                      <div className='grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2'>
                        {data.episode_lists?.length > 0 ? (
                          data.episode_lists.map((episode) => (
                            <Tooltip key={episode.slug}>
                              <TooltipTrigger asChild>
                                <Button
                                  variant='ghost'
                                  onClick={() =>
                                    router.push(`/anime/full/${episode.slug}`)
                                  }
                                  className='justify-between w-full h-full p-3 whitespace-normal'
                                >
                                  <div className='flex items-start gap-2 min-w-0'>
                                    <Clapperboard className='w-4 h-4 mt-1 flex-shrink-0' />
                                    <p className='line-clamp-3 text-left'>
                                      {episode.episode}
                                    </p>
                                  </div>
                                  <ArrowRight className='w-4 h-4 self-center flex-shrink-0' />
                                </Button>
                              </TooltipTrigger>
                              <TooltipContent>
                                <p>{episode.episode}</p>
                              </TooltipContent>
                            </Tooltip>
                          ))
                        ) : (
                          <div className='col-span-full py-6 text-center text-muted-foreground'>
                            <Film className='mx-auto h-12 w-12 mb-3' />
                            No episodes available yet.
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>

                  <div>
                    <h2 className='text-2xl font-bold tracking-tight mb-4'>
                      Recommendations
                    </h2>
                    <div className='flex overflow-x-auto pb-4 -mx-1 gap-4'>
                      {data.recommendations?.length > 0 ? (
                        data.recommendations.map((rec) => (
                          <div
                            key={rec.slug}
                            className='flex-shrink-0 w-40 md:w-48'
                          >
                            <MediaCard
                              title={rec.title}
                              imageUrl={rec.poster}
                              linkUrl={`/anime/detail/${rec.slug}`}
                            />
                          </div>
                        ))
                      ) : (
                        <div className='w-full py-6 text-center text-muted-foreground'>
                          <Popcorn className='mx-auto h-12 w-12 mb-3' />
                          No recommendations available.
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </BackgroundGradient>
        </div>
      </main>
    </TooltipProvider>
  );
}
