'use client';

import React, { memo } from 'react';
import { getErrorMessage } from '../../../../utils/client-utils';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '../../../../components/ui/accordion';
import { Badge } from '../../../../components/ui/badge';
import { Button } from '../../../../components/ui/button';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '../../../../components/ui/card';
import PosterImage from '../../../../components/shared/PosterImage';
import MediaCard from '../../../../components/anime/MediaCard';
import { BackgroundGradient } from '../../../../components/background/background-gradient';
import {
  CircleDot,
  Calendar,
  Video,
  Download,
  Film,
  BookOpen,
  Server,
  Package,
} from 'lucide-react';
import ErrorLoadingDisplay from '../../../../components/shared/ErrorLoadingDisplay';
import { useAnime2Detail, type Anime2Data, type DownloadResolution2 } from '../../../../utils/hooks/useAnime2';

interface Anime2DetailPageClientProps {
  slug: string;
  initialData: Anime2Data | null;
  initialError: string | null;
}

const processDownloads = (downloads: DownloadResolution2[] = []) => {
  const episodes: Record<string, DownloadResolution2[]> = {};
  downloads.forEach((download) => {
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

function Anime2DetailPageClient({
  slug,
  initialData,
  initialError,
}: Anime2DetailPageClientProps) {
  const { data: swrData, error: swrError } = useAnime2Detail(slug, initialData || undefined);

  const anime = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  if (displayError) {
    return <ErrorLoadingDisplay type="error" message={displayError} />;
  }

  if (!anime) {
    return <ErrorLoadingDisplay type="loading" skeletonType="detail" />;
  }
  const groupedDownloads = processDownloads(anime.downloads || []);

  const metadata = [
    {
      label: 'Rilis',
      value: anime.release_date,
      icon: <Calendar className="w-5 h-5 text-red-500" />,
    },
    {
      label: 'Studio',
      value: anime.studio,
      icon: <Video className="w-5 h-5 text-purple-500" />,
    },
  ];

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="max-w-6xl mx-auto">
        <BackgroundGradient className="rounded-[24px] p-0.5">
          <div className="bg-card text-card-foreground rounded-[22px] p-6 md:p-10">
            <div className="flex flex-col md:flex-row items-start gap-8">
              <div className="w-full md:w-1/3 flex flex-col gap-4 md:sticky top-8">
                <Card className="overflow-hidden">
                  <PosterImage poster={anime.poster} title={anime.title} />
                </Card>
                <Card>
                  <CardHeader className="p-4 pb-2">
                    <CardTitle className="text-base">Informasi</CardTitle>
                  </CardHeader>
                  <CardContent className="p-4 pt-0 space-y-3 text-sm">
                    <div className="flex items-center gap-3">
                      <BookOpen className="w-5 h-5 text-primary" />
                      <span className="font-semibold">Tipe:</span>
                      <span className="text-muted-foreground">
                        {anime.type}
                      </span>
                    </div>
                    <div className="flex items-center gap-3">
                      <CircleDot className="w-5 h-5 text-green-500" />
                      <span className="font-semibold">Status:</span>
                      <span className="text-muted-foreground">
                        {anime.status}
                      </span>
                    </div>
                  </CardContent>
                </Card>
              </div>

              <div className="w-full md:w-2/3 space-y-6">
                <div className="space-y-2">
                  <h1 className="text-4xl font-bold tracking-tight">
                    {anime.title}
                  </h1>
                  {anime.alternative_title && (
                    <p className="text-xl text-muted-foreground">
                      {anime.alternative_title}
                    </p>
                  )}
                </div>

                <Card>
                  <CardContent className="p-4 grid grid-cols-2 gap-4">
                    {metadata.map((item) => (
                      <div key={item.label} className="flex items-center gap-3">
                        <span className="p-2 bg-muted rounded-lg">
                          {item.icon}
                        </span>
                        <div>
                          <p className="text-sm text-muted-foreground">
                            {item.label}
                          </p>
                          <p className="font-semibold">{item.value || 'N/A'}</p>
                        </div>
                      </div>
                    ))}
                  </CardContent>
                </Card>

                <div className="flex flex-wrap gap-2">
                  {anime.genres.map((genre) => (
                    <Badge variant="secondary" key={genre.slug}>
                      {genre.name}
                    </Badge>
                  ))}
                </div>

                <Card>
                  <CardHeader>
                    <CardTitle>Sinopsis</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <p className="text-muted-foreground leading-relaxed">
                      {anime.synopsis || 'Tidak ada sinopsis.'}
                    </p>
                  </CardContent>
                </Card>

                {/* === BAGIAN UNDUHAN DENGAN LOGIKA ASLI ANDA === */}
                {anime.batch && anime.batch.length > 0 && (
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center">
                        <Package className="mr-2" /> Unduhan Batch
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-4">
                      {anime.batch.map((batchRes, index) => (
                        <div key={index}>
                          <h4 className="font-semibold mb-2">
                            {batchRes.resolution}
                          </h4>
                          <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
                            {batchRes.links.map((link, linkIndex) => (
                              <Button
                                asChild
                                variant="secondary"
                                key={linkIndex}
                              >
                                <a
                                  href={link.url}
                                  target="_blank"
                                  rel="noopener noreferrer"
                                >
                                  <Server className="w-4 h-4 mr-2" />{' '}
                                  {link.name}
                                </a>
                              </Button>
                            ))}
                          </div>
                        </div>
                      ))}
                    </CardContent>
                  </Card>
                )}

                {Object.keys(groupedDownloads).length > 0 && (
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center">
                        <Film className="mr-2" /> Daftar Episode
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <Accordion type="single" collapsible className="w-full">
                        {Object.entries(groupedDownloads)
                          .sort(([epA], [epB]) => parseInt(epA) - parseInt(epB))
                          .map(([episodeNumber, resolutions]) => (
                            <AccordionItem
                              value={`ep-${episodeNumber}`}
                              key={episodeNumber}
                            >
                              <AccordionTrigger>
                                Episode {episodeNumber}
                              </AccordionTrigger>
                              <AccordionContent className="space-y-4">
                                {resolutions.map((res, resIndex) => (
                                  <div key={resIndex}>
                                    <h4 className="font-semibold text-sm text-muted-foreground mb-2">
                                      {res.resolution}
                                    </h4>
                                    <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
                                      {res.links.map((link, linkIndex) => (
                                        <Button
                                          asChild
                                          variant="outline"
                                          size="sm"
                                          key={linkIndex}
                                        >
                                          <a
                                            href={link.url}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                          >
                                            <Download className="w-4 h-4 mr-2" />{' '}
                                            {link.name}
                                          </a>
                                        </Button>
                                      ))}
                                    </div>
                                  </div>
                                ))}
                              </AccordionContent>
                            </AccordionItem>
                          ))}
                      </Accordion>
                    </CardContent>
                  </Card>
                )}
                {/* ============================================== */}

                {anime.recommendations && anime.recommendations.length > 0 && (
                  <div>
                    <h2 className="text-2xl font-bold tracking-tight mb-4">
                      Rekomendasi
                    </h2>
                    <div className="flex overflow-x-auto pb-4 -mx-1 gap-4">
                      {anime.recommendations.map((rec) => (
                        <div
                          key={rec.slug}
                          className="flex-shrink-0 w-40 md:w-48"
                        >
                          <MediaCard
                            title={rec.title}
                            imageUrl={rec.poster}
                            linkUrl={`/anime2/detail/${rec.slug}`}
                            badge={rec.type}
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
  );
}

export default memo(Anime2DetailPageClient);
