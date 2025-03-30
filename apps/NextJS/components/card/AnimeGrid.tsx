// components/AnimeGrid.tsx
import React from 'react';
import MediaCard from './MediaCard';

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
  animes?: Anime[];
  loading?: boolean;
  anime2?: boolean;
}

// components/AnimeGrid.tsx
// components/AnimeGrid.tsx
const AnimeGrid: React.FC<AnimeGridProps> = ({
  animes,
  loading = false,
  anime2,
}) => {
  const link = anime2 ? '/anime2/detail' : '/anime/detail';
  if (loading) {
    return (
      <div className='flex flex-col items-center p-4'>
        <div className='grid  grid-cols-3 lg:grid-cols-5 gap-4 w-full'>
          {Array.from({ length: 25 }).map((_, index) => (
            <MediaCard key={index} loading={loading} />
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className='flex flex-col items-center p-4'>
      <div className='grid grid-cols-3 lg:grid-cols-5 gap-4 w-full'>
        {animes?.map((anime) => (
          <MediaCard
            key={anime.slug}
            title={anime.title}
            description={
              anime.current_episode ||
              anime.episode ||
              anime.episode_count ||
              ''
            }
            imageUrl={anime.poster || ''}
            linkUrl={`${link}/${anime.slug}`}
          />
        ))}
      </div>
    </div>
  );
};

export default AnimeGrid;
