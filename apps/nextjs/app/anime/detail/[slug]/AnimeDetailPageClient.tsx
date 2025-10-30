'use client';

import React, { useEffect, memo } from 'react';
import { useRouter } from 'next/navigation';
import { getErrorMessage } from '../../../../utils/client-utils';

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
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '../../../../components/ui/tooltip';
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
import PosterImage from '../../../../components/shared/PosterImage';
import ErrorLoadingDisplay from '../../../../components/shared/ErrorLoadingDisplay';
import { AnimeData } from '../../../../types/anime';
import { useBookmark } from '../../../../utils/hooks/useBookmark';
import type { AnimeBookmark } from '../../../../lib/bookmarks';
import { useAnimeDetail } from '../../../../utils/hooks/useAnime';

interface AnimeDetailPageClientProps {
  slug: string;
  initialData: AnimeData | null;
  initialError: string | null;
}

function AnimeDetailPageClient({
  slug,
  initialData,
  initialError,
}: AnimeDetailPageClientProps) {
  const router = useRouter();
  const { data: swrData, error: swrError } = useAnimeDetail(
    slug,
    initialData || undefined,
  );

  // Use the SWR data if available, otherwise fallback to initialData
  const animeData = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  // Use bookmark hook with anime data
  const bookmarkData: AnimeBookmark | undefined = animeData
    ? {
        slug,
        title: animeData.title,
        poster: animeData.poster,
      }
    : undefined;

  const { isBookmarked: bookmarked, toggle: handleBookmark } =
    useBookmark<AnimeBookmark>('anime', slug, bookmarkData);

  useEffect(() => {
    if (animeData?.episode_lists) {
      animeData.episode_lists.forEach((episode) => {
        router.prefetch(`/anime/full/${episode.slug}`);
      });
    }
  }, [animeData, router]);

  if (displayError) {
    return <ErrorLoadingDisplay type="error" message={displayError} />;
  }

  if (!animeData) {
    return <ErrorLoadingDisplay type="loading" skeletonType="detail" />;
  }
  const metadata = [
    {
      label: 'Type',
      value: animeData.type,
      icon: <Type className="w-5 h-5 text-blue-500" />,
    },
    {
      label: 'Status',
      value: animeData.status,
      icon: <CircleDot className="w-5 h-5 text-green-500" />,
    },
    {
      label: 'Released',
      value: animeData.release_date,
      icon: <Calendar className="w-5 h-5 text-red-500" />,
    },
    {
      label: 'Studio',
      value: animeData.studio,
      icon: <Video className="w-5 h-5 text-purple-500" />,
    },
  ];

  return (
    <TooltipProvider delayDuration={100}>
      <main className="p-4 md:p-8 bg-background min-h-screen">
        <div className="max-w-6xl mx-auto">
          <BackgroundGradient className="rounded-[24px] p-0.5">
            <div className="bg-card text-card-foreground rounded-[22px] p-6 md:p-10">
              <div className="flex flex-col md:flex-row items-start gap-8">
                <div className="w-full md:w-1/3 flex flex-col gap-4 md:sticky top-8">
                  <Card className="overflow-hidden">
                    <PosterImage
                      poster={animeData.poster}
                      title={animeData.title}
                    />
                  </Card>
                  <Button
                    onClick={() => handleBookmark()}
                    variant={bookmarked ? 'destructive' : 'default'}
                    size="lg"
                    className="w-full"
                  >
                    <Bookmark className="w-5 h-5 mr-2" />
                    {bookmarked ? 'Remove from Bookmarks' : 'Add to Bookmarks'}
                  </Button>
                </div>

                <div className="w-full md:w-2/3 space-y-6">
                  <h1 className="text-4xl font-bold tracking-tight">
                    {animeData.title}
                  </h1>

                  <Card>
                    <CardContent className="p-4 grid grid-cols-2 md:grid-cols-4 gap-4">
                      {metadata.map((item) => (
                        <div
                          key={item.label}
                          className="flex items-center gap-3"
                        >
                          <span className="p-2 bg-muted rounded-lg">
                            {item.icon}
                          </span>
                          <div>
                            <p className="text-sm text-muted-foreground">
                              {item.label}
                            </p>
                            <p className="font-semibold">
                              {item.value || 'N/A'}
                            </p>
                          </div>
                        </div>
                      ))}
                    </CardContent>
                  </Card>

                  <div className="flex flex-wrap gap-2">
                    {animeData.genres?.map((genre) => (
                      <Badge variant="secondary" key={genre.slug}>
                        {genre.name}
                      </Badge>
                    ))}
                  </div>

                  <Card>
                    <CardHeader>
                      <CardTitle>Synopsis</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className="text-muted-foreground leading-relaxed">
                        {animeData.synopsis || 'No synopsis available.'}
                      </p>
                    </CardContent>
                  </Card>

                  <Card>
                    <CardHeader>
                      <CardTitle>Episodes</CardTitle>
                      <CardDescription>
                        Total {animeData.episode_lists?.length ?? 0} episodes
                        available.
                      </CardDescription>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                        {animeData.episode_lists &&
                        animeData.episode_lists.length > 0 ? (
                          animeData.episode_lists.map((episode, index) => (
                            <Tooltip key={`${episode.slug}-${index}`}>
                              <TooltipTrigger asChild>
                                <Button
                                  variant="ghost"
                                  onClick={() =>
                                    router.push(`/anime/full/${episode.slug}`)
                                  }
                                  className="justify-between w-full h-full p-3 whitespace-normal"
                                >
                                  <div className="flex items-start gap-2 min-w-0">
                                    <Clapperboard className="w-4 h-4 mt-1 flex-shrink-0" />
                                    <p className="line-clamp-3 text-left">
                                      {episode.episode}
                                    </p>
                                  </div>
                                  <ArrowRight className="w-4 h-4 self-center flex-shrink-0" />
                                </Button>
                              </TooltipTrigger>
                              <TooltipContent>
                                <p>{episode.episode}</p>
                              </TooltipContent>
                            </Tooltip>
                          ))
                        ) : (
                          <div className="col-span-full py-6 text-center text-muted-foreground">
                            <Film className="mx-auto h-12 w-12 mb-3" />
                            No episodes available yet.
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>

                  <div>
                    <h2 className="text-2xl font-bold tracking-tight mb-4">
                      Recommendations
                    </h2>
                    <div className="flex overflow-x-auto pb-4 -mx-1 gap-4">
                      {animeData.recommendations &&
                      animeData.recommendations.length > 0 ? (
                        animeData.recommendations.map((rec, index) => (
                          <div
                            key={`${rec.slug}-${index}`}
                            className="flex-shrink-0 w-40 md:w-48"
                          >
                            <MediaCard
                              title={rec.title}
                              imageUrl={rec.poster}
                              linkUrl={`/anime/detail/${rec.slug}`}
                            />
                          </div>
                        ))
                      ) : (
                        <div className="w-full py-6 text-center text-muted-foreground">
                          <Popcorn className="mx-auto h-12 w-12 mb-3" />
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

export default memo(AnimeDetailPageClient);
