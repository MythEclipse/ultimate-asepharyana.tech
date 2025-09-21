'use client';

import AnimeGrid from '../../components/anime/AnimeGrid';
import useSWR from 'swr';
import {
  ArrowRight,
  CheckCircle,
  Clapperboard,
  TriangleAlert,
} from 'lucide-react';
import Link from 'next/link';

import { fetchData } from '../../utils/useFetch';

const fetcher = async (url: string) => {
  const response = await fetchData(url);
  return response.data as HomeData;
};

interface HomeData {
  status: string;
  data: {
    ongoing_anime: OngoingAnime[];
    complete_anime: CompleteAnime[];
  };
}

interface OngoingAnime {
  title: string;
  slug: string;
  poster: string;
  current_episode: string;
  anime_url: string;
}

interface CompleteAnime {
  title: string;
  slug: string;
  poster: string;
  episode_count: string;
  anime_url: string;
}

interface Anime2PageClientProps {
  initialData: HomeData | null;
  initialError: string | null;
}

function Anime2PageClient({
  initialData,
  initialError,
}: Anime2PageClientProps) {
  const { data, error, isLoading, mutate } = useSWR<HomeData>(
    `/api/anime2/`,
    fetcher,
    {
      revalidateOnFocus: false,
      fallbackData: initialData || undefined,
      refreshInterval: 60000,
    },
  );

  const displayError = error || initialError;
  const displayData = data || initialData;

  if (isLoading && !displayData) {
    return (
      <main className="p-4 md:p-8 bg-background dark:bg-dark min-h-screen">
        <div className="max-w-7xl mx-auto">
          <h1 className="text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400">
            Anime
          </h1>

          {/* Ongoing Anime Section */}
          <section className="mb-12 space-y-6">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-3">
                <div className="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
                  <Clapperboard className="w-6 h-6 text-blue-600 dark:text-blue-400" />
                </div>
                <h2 className="text-xl sm:text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                  Ongoing Anime
                </h2>
              </div>
              <Link
                href="/anime2/ongoing-anime/1"
                className="flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
              >
                View All
                <ArrowRight className="w-4 h-4" />
              </Link>
            </div>

            <AnimeGrid anime2 animes={[]} loading={true} />
          </section>

          {/* Complete Anime Section */}
          <section className="space-y-6">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-3">
                <div className="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
                  <CheckCircle className="w-6 h-6 text-green-600 dark:text-green-400" />
                </div>
                <h2 className="text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent">
                  Complete Anime
                </h2>
              </div>
              <Link
                href="/anime2/complete-anime/1"
                className="flex items-center gap-2 text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 transition-colors"
              >
                View All
                <ArrowRight className="w-4 h-4" />
              </Link>
            </div>

            <AnimeGrid anime2 animes={[]} loading={true} />
          </section>
        </div>
      </main>
    );
  }

  if (
    displayError ||
    !displayData ||
    displayData.status !== 'Ok' ||
    !displayData.data
  ) {
    return (
      <div className="p-4 md:p-8 bg-background dark:bg-dark min-h-screen flex flex-col items-center justify-center gap-4">
        <TriangleAlert className="w-16 h-16 text-red-500" />
        <h2 className="text-2xl font-bold text-center">
          {displayError ? 'Gagal memuat data anime' : 'Data tidak ditemukan'}
        </h2>
        <button
          onClick={() => mutate()}
          className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors"
        >
          Coba Lagi
        </button>
      </div>
    );
  }

  return (
    <main className="p-4 md:p-8 bg-background dark:bg-dark min-h-screen">
      <div className="max-w-7xl mx-auto">
        <h1 className="text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400">
          Anime
        </h1>

        {/* Ongoing Anime Section */}
        <section className="mb-12 space-y-6">
          <div className="flex items-center justify-between mb-6">
            <div className="flex items-center gap-3">
              <div className="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
                <Clapperboard className="w-6 h-6 text-blue-600 dark:text-blue-400" />
              </div>
              <h2 className="text-xl sm:text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                Ongoing Anime
              </h2>
            </div>
            <Link
              href="/anime2/ongoing-anime/1"
              className="flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
            >
              View All
              <ArrowRight className="w-4 h-4" />
            </Link>
          </div>

          <AnimeGrid
            anime2={true}
            animes={displayData.data.ongoing_anime.map((anime) => ({
              ...anime,
              rating: '',
              release_day: '',
              newest_release_date: '',
            }))}
          />
        </section>

        {/* Complete Anime Section */}
        <section className="space-y-6">
          <div className="flex items-center justify-between mb-6">
            <div className="flex items-center gap-3">
              <div className="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
                <CheckCircle className="w-6 h-6 text-green-600 dark:text-green-400" />
              </div>
              <h2 className="text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent">
                Complete Anime
              </h2>
            </div>
            <Link
              href="/anime2/complete-anime/1"
              className="flex items-center gap-2 text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 transition-colors"
            >
              View All
              <ArrowRight className="w-4 h-4" />
            </Link>
          </div>

          <AnimeGrid
            anime2={true}
            animes={displayData.data.complete_anime.map((anime) => ({
              ...anime,
              rating: '',
              release_day: '',
              newest_release_date: '',
              current_episode: anime.episode_count,
            }))}
          />
        </section>
      </div>
    </main>
  );
}

export default Anime2PageClient;
