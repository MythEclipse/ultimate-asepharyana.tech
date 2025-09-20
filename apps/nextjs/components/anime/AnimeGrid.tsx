// components/AnimeGrid.tsx
'use client';

import React, { memo, useMemo } from 'react';
import dynamic from 'next/dynamic';

const MediaCard = dynamic(() => import('./MediaCard'), { ssr: false });

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
  anime?: Anime;
  loading2?: boolean;
  anime2?: boolean;
}

const COLUMN_CLASSES = 'grid grid-cols-3 lg:grid-cols-5 gap-6 w-full';

const AnimeGrid: React.FC<AnimeGridProps> = memo(
  ({ animes, loading = false, anime, loading2 = false, anime2 }) => {
    const link = anime2 ? '/anime2/detail' : '/anime/detail';

    const loadingCards = useMemo(
      () =>
        Array.from({ length: 40 }).map((_, index) => (
          <MediaCard key={index} loading={loading} />
        )),
      [loading],
    );

    const animeList = animes || [];

    if (anime) {
      return (
        <MediaCard
          key={anime.slug}
          title={anime.title}
          description={
            anime.current_episode || anime.episode || anime.episode_count || ''
          }
          imageUrl={anime.poster || ''}
          linkUrl={`${link}/${anime.slug}`}
        />
      );
    }
    if (loading2) {
      return <MediaCard loading={true} />;
    }
    if (loading) {
      return (
        <div className={COLUMN_CLASSES} style={{ padding: '1rem' }}>
          {loadingCards}
        </div>
      );
    }

    return (
      <div className={COLUMN_CLASSES} style={{ padding: '1rem' }}>
        {animeList.map((anime) => (
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
    );
  },
);

AnimeGrid.displayName = 'AnimeGrid';

export default AnimeGrid;
