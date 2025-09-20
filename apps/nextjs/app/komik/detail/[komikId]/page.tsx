import PosterImage from '../PosterImage';
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
  Calendar,
  CircleDot,
  FileText,
  Type,
  User,
  ArrowRight,
  AlertTriangle,
} from 'lucide-react';
import { fetchKomikData } from '../../../../lib/komikFetcher';

// --- INTERFACES ---
interface Chapter {
  chapter: string;
  chapter_id: string;
  date: string;
}

interface MangaData {
  title: string;
  poster: string;
  type: string;
  status: string;
  release_date: string;
  author: string;
  description: string;
  total_chapter: string;
  updated_on: string;
  genres: string[];
  chapters: Chapter[];
}

interface ApiResponse {
  status: boolean;
  data: MangaData;
}

export const revalidate = 60;

export default async function DetailMangaPage({
  params,
}: {
  params: Promise<{ komikId: string }>;
}) {
  const { komikId } = await params;

  let mangaData: ApiResponse | null = null;

  try {
    mangaData = await fetchKomikData(`/api/komik2/detail?komik_id=${komikId}`, revalidate, 10000);
  } catch (_e) {
    return (
      <div className="min-h-screen p-6 flex items-center justify-center">
        <Card className="max-w-md w-full p-8 text-center">
          <AlertTriangle className="w-16 h-16 text-destructive mx-auto mb-4" />
          <CardHeader>
            <CardTitle className="text-2xl text-destructive">
              Gagal Memuat Data
            </CardTitle>
            <CardDescription>
              Terjadi kesalahan saat mengambil data manga.
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  if (!mangaData?.data) {
    return (
      <div className="min-h-screen p-6 flex items-center justify-center">
        <Card className="max-w-md w-full p-8 text-center">
          <AlertTriangle className="w-16 h-16 text-destructive mx-auto mb-4" />
          <CardHeader>
            <CardTitle className="text-2xl text-destructive">
              Gagal Memuat Data
            </CardTitle>
            <CardDescription>
              Terjadi kesalahan saat mengambil data manga.
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  const manga = mangaData.data;

  const metadata = [
    {
      label: 'Tipe',
      value: manga.type,
      icon: <Type className="w-5 h-5 text-blue-500" />,
    },
    {
      label: 'Status',
      value: manga.status,
      icon: <CircleDot className="w-5 h-5 text-green-500" />,
    },
    {
      label: 'Rilis',
      value: manga.release_date,
      icon: <Calendar className="w-5 h-5 text-red-500" />,
    },
    {
      label: 'Author',
      value: manga.author,
      icon: <User className="w-5 h-5 text-purple-500" />,
    },
    {
      label: 'Total Chapter',
      value: manga.total_chapter,
      icon: <FileText className="w-5 h-5 text-gray-500" />,
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
                    <PosterImage poster={manga.poster} title={manga.title} />
                  </Card>
                  <BookmarkButton
                    komikId={komikId}
                    title={manga.title}
                    poster={manga.poster}
                  />
                </div>

                <div className="w-full md:w-2/3 space-y-6">
                  <div className="space-y-2">
                    <h1 className="text-4xl font-bold tracking-tight">
                      {manga.title}
                    </h1>
                  </div>

                  <Card>
                    <CardContent className="p-4 grid grid-cols-2 md:grid-cols-3 gap-4">
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
                    {manga.genres?.map((genre: string, index: number) => (
                      <Badge variant="secondary" key={index}>
                        {genre}
                      </Badge>
                    ))}
                  </div>

                  <Card>
                    <CardHeader>
                      <CardTitle>Sinopsis</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className="text-muted-foreground leading-relaxed">
                        {manga.description || 'Tidak ada sinopsis.'}
                      </p>
                    </CardContent>
                  </Card>

                  <Card>
                    <CardHeader>
                      <CardTitle>Daftar Chapter</CardTitle>
                      {manga.updated_on && (
                        <CardDescription>
                          Terakhir update: {manga.updated_on}
                        </CardDescription>
                      )}
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                        {manga.chapters?.length > 0 ? (
                          manga.chapters.map((chapter: Chapter) => (
                            <Tooltip key={chapter.chapter_id}>
                              <TooltipTrigger asChild>
                                <Button
                                  variant="ghost"
                                  className="justify-between w-full h-full p-3 whitespace-normal"
                                >
                                  <a
                                    href={`/komik/chapter/${chapter.chapter_id}`}
                                    className="line-clamp-2 text-left flex-1"
                                  >
                                    {chapter.chapter}
                                  </a>
                                  <ArrowRight className="w-4 h-4 ml-2 flex-shrink-0" />
                                </Button>
                              </TooltipTrigger>
                              <TooltipContent>
                                <p>Rilis: {chapter.date}</p>
                              </TooltipContent>
                            </Tooltip>
                          ))
                        ) : (
                          <div className="col-span-full py-6 text-center text-muted-foreground">
                            <FileText className="mx-auto h-12 w-12 mb-3" />
                            Belum ada chapter.
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
