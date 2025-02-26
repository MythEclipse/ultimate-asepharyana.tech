// components/AnimeGrid.tsx
import React from 'react';
import MediaCard from './MediaCard';
import { BaseUrl } from '@/lib/url';

interface Anime {
  title: string;
  slug: string;
  rating?: string;
  poster?: string;
  release_day?: string;
  newest_release_date?: string;
  anime_url?: string;
  current_episode?: string;
  episode?: string;
  episode_count?: string;
}

interface AnimeGridProps {
  animes: Anime[];
}

const AnimeGrid: React.FC<AnimeGridProps> = ({ animes }) => {
  return (
    <div className='flex flex-col items-center p-4'>
      <div className='grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-4'>
        {animes.map((anime) => (
          <MediaCard
            key={anime.slug}
            title={anime.title}
            description={
              anime.current_episode ||
              anime.episode ||
              anime.episode_count ||
              ''
            }
            imageUrl={
              anime.poster
                ? `${BaseUrl}/api/imageproxy?url=${encodeURIComponent(anime.poster)}`
                : ''
            }
            linkUrl={`/anime2/full/${anime.slug}`}
          />
        ))}
      </div>
    </div>
  );
};

export default AnimeGrid;
