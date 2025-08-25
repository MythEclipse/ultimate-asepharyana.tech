'use client';

import React, { useState, useEffect } from 'react';
import useSWR from 'swr';
import Image from 'next/image';
import { useParams, useRouter } from 'next/navigation';
import { PRODUCTION } from '../../../../lib/url';

import { BackgroundGradient } from '../../../../components/background/background-gradient';
import MediaCard from '../../../../components/anime/MediaCard';
import { Button } from '../../../../components/ui/button';
import { Badge } from '../../../../components/ui/badge';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from '../../../../components/ui/card';
import { Skeleton } from '../../../../components/ui/skeleton';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '../../../../components/ui/tooltip';
import {
  Bookmark,
  Calendar,
  CircleDot,
  FileText,
  Star,
  Type,
  User,
  ArrowRight,
  AlertTriangle,
} from 'lucide-react';

// --- INTERFACES ---
interface Chapter {
  chapter: string;
  chapter_id: string;
  date: string;
}

interface Recommendation {
  slug: string;
  title: string;
  poster: string;
}

interface MangaData {
  title: string;
  alternativeTitle: string;
  score: string;
  poster: string;
  type: string;
  status: string;
  releaseDate: string;
  author: string;
  description: string;
  totalChapter: string;
  updatedOn: string;
  genres: string[];
  chapters: Chapter[];
  recommendations: Recommendation[];
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

// --- SKELETON COMPONENT ---
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
              <CardContent className='p-4'>
                <Skeleton className='h-20 w-full' />
              </CardContent>
            </Card>
            <div className='flex flex-wrap gap-2'>
              {[...Array(3)].map((_, i) => (
                <Skeleton key={i} className='h-8 w-24 rounded-full' />
              ))}
            </div>
            <Skeleton className='h-24 w-full' />
            <Skeleton className='h-48 w-full' />
          </div>
        </div>
      </div>
    </div>
  </main>
);

export default function DetailMangaPage() {
  const router = useRouter();
  const params = useParams();
  const komikId = params.komikId as string;

  const {
    data: mangaData,
    error,
    isLoading,
  } = useSWR<MangaData>(
    komikId ? `/api/komik/detail?komik_id=${komikId}` : null,
    fetcher
  );

  const [bookmarked, setBookmarked] = useState(false);
  const [currentIndex, setCurrentIndex] = useState(0);

  useEffect(() => {
    if (typeof window !== 'undefined' && komikId) {
      const bookmarks = JSON.parse(
        localStorage.getItem('bookmarks-komik') || '[]'
      );
      setBookmarked(
        bookmarks.some((item: { slug: string }) => item.slug === komikId)
      );
    }
  }, [komikId]);

  const handleBookmark = () => {
    if (!mangaData) return;
    let bookmarks = JSON.parse(localStorage.getItem('bookmarks-komik') || '[]');
    const isBookmarked = bookmarks.some(
      (item: { slug: string }) => item.slug === komikId
    );

    if (isBookmarked) {
      bookmarks = bookmarks.filter(
        (item: { slug: string }) => item.slug !== komikId
      );
    } else {
      bookmarks.push({
        slug: komikId,
        title: mangaData.title,
        poster: mangaData.poster,
      });
    }
    localStorage.setItem('bookmarks-komik', JSON.stringify(bookmarks));
    setBookmarked(!isBookmarked);
  };

  if (isLoading) return <DetailPageSkeleton />;
  if (error || !mangaData)
    return (
      <div className='min-h-screen p-6 flex items-center justify-center'>
        <Card className='max-w-md w-full p-8 text-center'>
          <AlertTriangle className='w-16 h-16 text-destructive mx-auto mb-4' />
          <CardHeader>
            <CardTitle className='text-2xl text-destructive'>
              Gagal Memuat Data
            </CardTitle>
            <CardDescription>
              Terjadi kesalahan saat mengambil data manga.
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );

  const manga = mangaData;
  const fallback = '/default.png';
  const imageSources = [
    manga.poster?.trim() ? manga.poster : null,
    manga.poster?.trim()
      ? `https://imagecdn.app/v1/images/${encodeURIComponent(manga.poster)}`
      : null,
    manga.poster?.trim()
      ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(manga.poster)}`
      : null,
    fallback,
  ].filter(Boolean) as string[];

  const handleError = () => {
    if (currentIndex < imageSources.length - 1)
      setCurrentIndex(currentIndex + 1);
  };

  const metadata = [
    {
      label: 'Skor',
      value: manga.score,
      icon: <Star className='w-5 h-5 text-amber-500' />,
    },
    {
      label: 'Tipe',
      value: manga.type,
      icon: <Type className='w-5 h-5 text-blue-500' />,
    },
    {
      label: 'Status',
      value: manga.status,
      icon: <CircleDot className='w-5 h-5 text-green-500' />,
    },
    {
      label: 'Rilis',
      value: manga.releaseDate,
      icon: <Calendar className='w-5 h-5 text-red-500' />,
    },
    {
      label: 'Author',
      value: manga.author,
      icon: <User className='w-5 h-5 text-purple-500' />,
    },
    {
      label: 'Total Chapter',
      value: manga.totalChapter,
      icon: <FileText className='w-5 h-5 text-gray-500' />,
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
                      src={imageSources[currentIndex]}
                      alt={manga.title}
                      width={400}
                      height={600}
                      className='object-cover w-full aspect-[2/3]'
                      priority
                      unoptimized
                      onError={handleError}
                    />
                  </Card>
                  <Button
                    onClick={handleBookmark}
                    variant={bookmarked ? 'destructive' : 'default'}
                    size='lg'
                    className='w-full'
                  >
                    <Bookmark className='w-5 h-5 mr-2' />
                    {bookmarked ? 'Hapus Bookmark' : 'Bookmark'}
                  </Button>
                </div>

                <div className='w-full md:w-2/3 space-y-6'>
                  <div className='space-y-2'>
                    <h1 className='text-4xl font-bold tracking-tight'>
                      {manga.title}
                    </h1>
                    {manga.alternativeTitle && (
                      <p className='text-xl text-muted-foreground'>
                        {manga.alternativeTitle}
                      </p>
                    )}
                  </div>

                  <Card>
                    <CardContent className='p-4 grid grid-cols-2 md:grid-cols-3 gap-4'>
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
                    {manga.genres?.map((genre, index) => (
                      <Badge variant='secondary' key={index}>
                        {genre}
                      </Badge>
                    ))}
                  </div>

                  <Card>
                    <CardHeader>
                      <CardTitle>Sinopsis</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className='text-muted-foreground leading-relaxed'>
                        {manga.description || 'Tidak ada sinopsis.'}
                      </p>
                    </CardContent>
                  </Card>

                  <Card>
                    <CardHeader>
                      <CardTitle>Daftar Chapter</CardTitle>
                      {manga.updatedOn && (
                        <CardDescription>
                          Terakhir update: {manga.updatedOn}
                        </CardDescription>
                      )}
                    </CardHeader>
                    <CardContent>
                      <div className='grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2'>
                        {manga.chapters?.length > 0 ? (
                          manga.chapters.map((chapter) => (
                            <Tooltip key={chapter.chapter_id}>
                              <TooltipTrigger asChild>
                                <Button
                                  variant='ghost'
                                  onClick={() =>
                                    router.push(
                                      `/komik/chapter/${chapter.chapter_id}`
                                    )
                                  }
                                  className='justify-between w-full h-full p-3 whitespace-normal'
                                >
                                  <p className='line-clamp-2 text-left'>
                                    {chapter.chapter}
                                  </p>
                                  <ArrowRight className='w-4 h-4 ml-2 flex-shrink-0' />
                                </Button>
                              </TooltipTrigger>
                              <TooltipContent>
                                <p>Rilis: {chapter.date}</p>
                              </TooltipContent>
                            </Tooltip>
                          ))
                        ) : (
                          <div className='col-span-full py-6 text-center text-muted-foreground'>
                            <FileText className='mx-auto h-12 w-12 mb-3' />
                            Belum ada chapter.
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>

                  {manga.recommendations?.length > 0 && (
                    <div>
                      <h2 className='text-2xl font-bold tracking-tight mb-4'>
                        Rekomendasi
                      </h2>
                      <div className='flex overflow-x-auto pb-4 -mx-1 gap-4'>
                        {manga.recommendations.map((rec) => (
                          <div
                            key={rec.slug}
                            className='flex-shrink-0 w-40 md:w-48'
                          >
                            <MediaCard
                              title={rec.title}
                              imageUrl={rec.poster}
                              linkUrl={`/komik/detail/${rec.slug}`}
                            />
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </BackgroundGradient>
        </div>
      </main>
    </TooltipProvider>
  );
}
