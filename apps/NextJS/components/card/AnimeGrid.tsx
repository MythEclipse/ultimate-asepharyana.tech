// components/AnimeGrid.tsx
import React from 'react';
import MediaCard from './MediaCard';

interface Anime {
  title: string;
  slug: string;
  rating?: string;
  poster?: string;
  current_episode?: string;
  release_day?: string;
  newest_release_date?: string;
  anime_url?: string;
}

interface AnimeGridProps {
  animes: Anime[];
}

const AnimeGrid: React.FC<AnimeGridProps> = ({ animes }) => {
  return (
    <div className="flex flex-col items-center p-4">
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-4">
        {animes.map((anime) => (
          <MediaCard
            key={anime.slug}
            title={anime.title}
            description={anime.rating || 'N/A'}
            imageUrl={anime.poster || ''}
            linkUrl={`/anime/detail/${anime.slug}`}
          />
        ))}
      </div>
    </div>
  );
};

export default AnimeGrid;
