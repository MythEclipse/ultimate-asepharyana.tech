'use client';

import { memo } from 'react';
import Link from 'next/link';
import UnifiedGrid from '../../components/shared/UnifiedGrid';
import { ErrorState } from '../../components/error/ErrorState';
import { BookOpen, AlertTriangle, Info, ArrowRight } from 'lucide-react';
import {
  useMangaList,
  useManhuaList,
  useManhwaList,
  type Komik,
  type KomikData,
} from '../../utils/hooks/useKomik';

interface KomikPageClientProps {
  manga: KomikData | null;
  manhua: KomikData | null;
  manhwa: KomikData | null;
  error: string | null;
}

function KomikPageClient({
  manga,
  manhua,
  manhwa,
  error,
}: KomikPageClientProps) {
  const { data: mangaData } = useMangaList(1, 'update', manga || undefined);
  const { data: manhuaData } = useManhuaList(1, 'update', manhua || undefined);
  const { data: manhwaData } = useManhwaList(1, 'update', manhwa || undefined);

  // Menentukan status loading untuk setiap kategori
  const isLoading = {
    Manga: !mangaData && !error,
    Manhua: !manhuaData && !error,
    Manhwa: !manhwaData && !error,
  };

  const komiksData = {
    Manga: mangaData?.data || manga?.data,
    Manhua: manhuaData?.data || manhua?.data,
    Manhwa: manhwaData?.data || manhwa?.data,
  };

  return (
    <main className="min-h-screen p-4 md:p-8 lg:p-12 bg-background text-foreground">
      <div className="max-w-7xl mx-auto space-y-12">
        <div className="flex items-center gap-4">
          <div className="p-3 bg-purple-100 dark:bg-purple-900/50 rounded-xl">
            <BookOpen className="w-8 h-8 text-purple-600 dark:text-purple-400" />
          </div>
          <h1 className="text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
            Komik Catalog
          </h1>
        </div>

        {error ? (
          <ErrorState
            icon={AlertTriangle}
            title="Error Loading Data"
            message="Failed to fetch comic data. Please try again later."
            type="error"
            fullScreen={false}
          />
        ) : (
          <div className="space-y-12">
            {['Manga', 'Manhua', 'Manhwa'].map((type) => {
              const komiks = komiksData[type as keyof typeof komiksData];

              return (
                <section key={type} className="mb-12 space-y-6">
                  <div className="flex items-center justify-between mb-6">
                    <div className="flex items-center gap-3">
                      <div className="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
                        <BookOpen className="w-6 h-6 text-blue-600 dark:text-blue-400" />
                      </div>
                      <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        {type}
                      </h2>
                    </div>
                    <Link
                      href={`/komik/${type.toLowerCase()}/page/1`}
                      className="flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
                    >
                      View All
                      <ArrowRight className="w-4 h-4" />
                    </Link>
                  </div>

                  {isLoading[type as keyof typeof isLoading] ? (
                    <UnifiedGrid loading={true} items={[]} itemType="komik" />
                  ) : komiks ? (
                    komiks.length > 0 ? (
                      <UnifiedGrid
                        items={komiks.map((comic: Komik) => ({
                          slug: comic.slug,
                          title: comic.title,
                          poster: comic.poster,
                          chapter: comic.chapter,
                          chapter_count: comic.chapter, // Assuming chapter here refers to latest chapter text, and we can use it for count display as well
                        }))}
                        itemType="komik"
                      />
                    ) : (
                      <div className="p-4 sm:p-6 bg-blue-100 dark:bg-blue-900/30 rounded-xl sm:rounded-2xl flex items-center gap-3 sm:gap-4">
                        <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
                        <h3 className="text-base sm:text-lg font-medium text-blue-800 dark:text-blue-200">
                          No {type} available at the moment
                        </h3>
                      </div>
                    )
                  ) : null}
                </section>
              );
            })}
          </div>
        )}
      </div>
    </main>
  );
}

export default memo(KomikPageClient);
