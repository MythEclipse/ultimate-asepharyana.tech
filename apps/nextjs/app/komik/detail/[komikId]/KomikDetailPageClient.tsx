'use client';

import { memo } from 'react';
import { getErrorMessage } from '../../../../utils/client-utils';
import PosterImage from '../../../../components/shared/PosterImage';
import BookmarkButton from '../BookmarkButton';
import { BackgroundGradient } from '../../../../components/background/background-gradient';
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
  CircleDot,
  FileText,
  Type,
  User,
  ArrowRight,
  AlertTriangle,
} from 'lucide-react';
import { useRouter } from 'next/navigation';
import {
  useKomikDetail,
  type KomikDetail,
} from '../../../../utils/hooks/useKomik';

interface KomikDetailPageClientProps {
  komikId: string;
  initialData: KomikDetail | null;
  initialError: string | null;
}

function KomikDetailPageClient({
  komikId,
  initialData,
  initialError,
}: KomikDetailPageClientProps) {
  const router = useRouter();
  const { data: swrData, error: swrError } = useKomikDetail(
    komikId,
    initialData || undefined,
  );

  const mangaData = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  if (displayError || !mangaData) {
    return (
      <div className="min-h-screen p-6 flex items-center justify-center">
        <Card className="max-w-md w-full p-8 text-center">
          <AlertTriangle className="w-16 h-16 text-destructive mx-auto mb-4" />
          <CardHeader>
            <CardTitle className="text-2xl text-destructive">
              Gagal Memuat Data
            </CardTitle>
            <CardDescription>
              {displayError || 'Terjadi kesalahan saat mengambil data komik.'}
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  const metadata = [
    {
      label: 'Type',
      value: mangaData.type,
      icon: <Type className="w-5 h-5 text-blue-500" />,
    },
    {
      label: 'Status',
      value: mangaData.status,
      icon: <CircleDot className="w-5 h-5 text-green-500" />,
    },
    {
      label: 'Author',
      value: mangaData.author,
      icon: <User className="w-5 h-5 text-purple-500" />,
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
                      poster={mangaData.poster}
                      title={mangaData.title}
                    />
                  </Card>
                  <BookmarkButton
                    komikId={komikId}
                    title={mangaData.title}
                    poster={mangaData.poster}
                  />
                </div>

                <div className="w-full md:w-2/3 space-y-6">
                  <h1 className="text-4xl font-bold tracking-tight">
                    {mangaData.title}
                  </h1>

                  {mangaData.alternative_title && (
                    <p className="text-lg text-muted-foreground">
                      {mangaData.alternative_title}
                    </p>
                  )}

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
                    {mangaData.genres?.map((genre) => (
                      <Badge variant="secondary" key={genre}>
                        {genre}
                      </Badge>
                    ))}
                  </div>

                  <Card>
                    <CardHeader>
                      <CardTitle>Synopsis</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className="text-muted-foreground leading-relaxed">
                        {mangaData.synopsis || 'No synopsis available.'}
                      </p>
                    </CardContent>
                  </Card>

                  <Card>
                    <CardHeader>
                      <CardTitle>Chapters</CardTitle>
                      <CardDescription>
                        Total {mangaData.chapters?.length ?? 0} chapters
                        available.
                      </CardDescription>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                        {mangaData.chapters && mangaData.chapters.length > 0 ? (
                          mangaData.chapters.map((chapter, index) => (
                            <Tooltip key={`${chapter.slug}-${index}`}>
                              <TooltipTrigger asChild>
                                <Button
                                  variant="ghost"
                                  onClick={() =>
                                    router.push(
                                      `/komik/chapter/${chapter.slug}`,
                                    )
                                  }
                                  className="justify-between w-full h-full p-3 whitespace-normal"
                                >
                                  <div className="flex items-start gap-2 min-w-0">
                                    <FileText className="w-4 h-4 mt-1 flex-shrink-0" />
                                    <p className="line-clamp-3 text-left">
                                      {chapter.title}
                                    </p>
                                  </div>
                                  <ArrowRight className="w-4 h-4 self-center flex-shrink-0" />
                                </Button>
                              </TooltipTrigger>
                              <TooltipContent>
                                <p>{chapter.title}</p>
                                {chapter.date && (
                                  <p className="text-xs">{chapter.date}</p>
                                )}
                              </TooltipContent>
                            </Tooltip>
                          ))
                        ) : (
                          <div className="col-span-full py-6 text-center text-muted-foreground">
                            <FileText className="mx-auto h-12 w-12 mb-3" />
                            No chapters available yet.
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </div>
            </div>
          </BackgroundGradient>
        </div>
      </main>
    </TooltipProvider>
  );
}

export default memo(KomikDetailPageClient);
