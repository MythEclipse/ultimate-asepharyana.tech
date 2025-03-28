'use client';

import { useParams } from 'next/navigation';
import useSWR from 'swr';
import { Link } from 'next-view-transitions';
import { BackgroundGradient } from '@/components/background/background-gradient';
import ButtonA from '@/components/button/ScrollButton';
import ClientPlayer from '@/components/misc/ClientPlayer';
import { ArrowLeft, ArrowRight, Download } from 'lucide-react';

interface AnimeResponse {
  status: string;
  data: AnimeData;
}

interface AnimeData {
  episode: string;
  episode_number: string;
  anime: AnimeInfo;
  has_next_episode: boolean;
  next_episode: EpisodeInfo | null;
  has_previous_episode: boolean;
  previous_episode: EpisodeInfo | null;
  stream_url: string;
  download_urls: Record<string, { server: string; url: string }[]>;
  image_url: string;
}

interface AnimeInfo {
  slug: string;
}

interface EpisodeInfo {
  slug: string;
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailAnimePage() {
  const params = useParams();
  const slug = params.slug as string;
  const { data, error, isLoading } = useSWR<AnimeResponse | null>(
    `/api/anime/full/${slug}`,
    fetcher,
    {
      revalidateOnFocus: false,
      revalidateOnReconnect: true,
    }
  );

  if (isLoading) {
    return (
      <BackgroundGradient className='rounded-3xl p-6 bg-white dark:bg-zinc-900 shadow-xl'>
        <div className='space-y-8'>
          {/* Skeleton Header */}
          <div className='text-center space-y-4 animate-pulse'>
            <div className='h-12 bg-zinc-200 dark:bg-zinc-800 rounded-full w-3/4 mx-auto' />
            <div className='h-1 bg-zinc-200 dark:bg-zinc-800 opacity-50' />
          </div>

          {/* Skeleton Video Player */}
          <div className='aspect-video rounded-xl overflow-hidden shadow-2xl bg-zinc-200 dark:bg-zinc-800 animate-pulse' />

          {/* Skeleton Navigation */}
          <div className='flex justify-between gap-4'>
            <div className='flex-1 h-12 bg-zinc-200 dark:bg-zinc-800 rounded-lg' />
            <div className='flex-1 h-12 bg-zinc-200 dark:bg-zinc-800 rounded-lg' />
          </div>

          {/* Skeleton Downloads */}
          <div className='space-y-6'>
            <div className='h-8 bg-zinc-200 dark:bg-zinc-800 rounded-full w-1/3 mx-auto' />
            <div className='grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5'>
              {[...Array(3)].map((_, i) => (
                <div
                  key={i}
                  className='relative rounded-2xl bg-zinc-100 dark:bg-zinc-800 p-6'
                >
                  <div className='h-6 bg-zinc-200 dark:bg-zinc-700 w-1/3 mb-4 rounded' />
                  <div className='space-y-3'>
                    {[...Array(2)].map((_, j) => (
                      <div
                        key={j}
                        className='h-16 bg-zinc-200 dark:bg-zinc-700 rounded-lg'
                      />
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </BackgroundGradient>
    );
  }

  if (error || !data || data.status !== 'Ok') {
    return (
      <div className='p-4 max-w-screen-md mx-auto'>
        <p className='text-red-500'>Error loading anime details</p>
      </div>
    );
  }

  return (
    <BackgroundGradient className='rounded-3xl p-6 bg-white dark:bg-zinc-900 shadow-xl'>
      <div className='space-y-8'>
        {/* Episode Header */}
        <div className='text-center space-y-4'>
          <h1 className='text-4xl md:text-5xl font-extrabold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent'>
            {data.data.episode}
          </h1>
          <div className='h-1 bg-gradient-to-r from-transparent via-blue-400 to-transparent opacity-50' />
        </div>

        {/* Video Player */}
        {data.data.stream_url && (
          <div className='aspect-video rounded-xl overflow-hidden shadow-2xl'>
            <ClientPlayer url={data.data.stream_url} />
          </div>
        )}

        {/* Episode Navigation */}
        <div className='flex justify-between gap-4'>
          {data.data.previous_episode && (
            <Link
              scroll
              href={`/anime/full/${data.data.previous_episode.slug}`}
              className='flex-1'
            >
              <ButtonA className='w-full gap-2 hover:-translate-x-1 transition-transform'>
                <ArrowLeft size={20} />
                Previous Episode
              </ButtonA>
            </Link>
          )}

          {data.data.next_episode && (
            <Link
              scroll
              href={`/anime/full/${data.data.next_episode.slug}`}
              className='flex-1'
            >
              <ButtonA className='w-full gap-2 hover:translate-x-1 transition-transform'>
                Next Episode
                <ArrowRight size={20} />
              </ButtonA>
            </Link>
          )}
        </div>

        {/* Downloads Section */}
        <div className='space-y-6'>
          <h2 className='text-3xl font-bold text-center bg-gradient-to-r from-green-400 to-cyan-400 bg-clip-text text-transparent'>
            Download Links
          </h2>

          <div className='grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5'>
            {Object.entries(data.data.download_urls).map(
              ([resolution, links]) => (
                <div
                  key={resolution}
                  className='relative bg-gradient-to-br from-blue-50/50 to-purple-50/50 dark:from-zinc-800 dark:to-zinc-700/50 p-6 shadow-lg'
                >
                  <div className='absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-blue-400 to-purple-500' />
                  <h3 className='text-xl font-bold mb-4 text-blue-600 dark:text-blue-400'>
                    {resolution}
                  </h3>

                  <div className='space-y-3'>
                    {links.map((link, index) => (
                      <div
                        key={index}
                        className='flex items-center justify-between bg-white/50 dark:bg-zinc-900/50 p-3 rounded-lg transition-all hover:bg-white dark:hover:bg-zinc-800'
                      >
                        <span className='font-medium text-gray-700 dark:text-gray-300'>
                          {link.server}
                        </span>
                        <ButtonA
                          href={link.url}
                          className='gap-2 px-4 py-2 rounded-lg hover:scale-105 transition-transform'
                        >
                          <Download size={18} />
                          <span className='font-semibold'>Download</span>
                        </ButtonA>
                      </div>
                    ))}
                  </div>
                </div>
              )
            )}
          </div>
        </div>
      </div>
    </BackgroundGradient>
  );
}
