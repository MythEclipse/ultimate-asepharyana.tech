// UnifiedGrid.tsx

import React from 'react';
import MediaCard from '../anime/MediaCard';

export type Anime = {
  slug: string;
  title: string;
  poster?: string;
  current_episode?: string;
  episode?: string;
  episode_count?: string;
};

export type Komik = {
  slug: string;
  title: string;
  poster?: string;
  chapter?: string;
  chapter_count?: string;
};

type ItemType = 'anime' | 'komik';

type UnifiedGridProps<T extends Anime | Komik = Anime | Komik> = {
  items?: T[];
  loading?: boolean;
  singleItem?: T;
  itemType: ItemType;
  gridClassName?: string;
  // Removed itemClassName and isAnime2
};

const DEFAULT_GRID_CLASS =
  'grid grid-cols-3 lg:grid-cols-5 gap-6 w-full';

function UnifiedGrid<T extends Anime | Komik>({
  items,
  loading = false,
  singleItem,
  itemType,
  gridClassName,
}: UnifiedGridProps<T>) {
  // Skeleton loading
  if (loading) {
    return (
      <div className={gridClassName || DEFAULT_GRID_CLASS}>
        {Array.from({ length: 10 }).map((_, idx) => (
          <MediaCard
            key={idx}
            loading={true}
          />
        ))}
      </div>
    );
  }

  // Single item mode
  if (singleItem) {
    if (itemType === 'anime') {
      const anime = singleItem as Anime;
      return (
        <div className={gridClassName || DEFAULT_GRID_CLASS}>
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
            linkUrl={`/anime/detail/${anime.slug}`}
          />
        </div>
      );
    } else {
      const komik = singleItem as Komik;
      return (
        <div className={gridClassName || DEFAULT_GRID_CLASS}>
          <MediaCard
            key={komik.slug}
            title={komik.title}
            description={
              komik.chapter ||
              komik.chapter_count ||
              ''
            }
            imageUrl={komik.poster || ''}
            linkUrl={`/komik/detail/${komik.slug}`}
          />
        </div>
      );
    }
  }

  // Items array mode
  if (items && items.length > 0) {
    return (
      <div className={gridClassName || DEFAULT_GRID_CLASS}>
        {items.map((item) => {
          if (itemType === 'anime') {
            const anime = item as Anime;
            return (
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
                linkUrl={`/anime/detail/${anime.slug}`}
              />
            );
          } else {
            const komik = item as Komik;
            return (
              <MediaCard
                key={komik.slug}
                title={komik.title}
                description={
                  komik.chapter ||
                  komik.chapter_count ||
                  ''
                }
                imageUrl={komik.poster || ''}
                linkUrl={`/komik/detail/${komik.slug}`}
              />
            );
          }
        })}
      </div>
    );
  }

  // Empty state
  return (
    <div className={gridClassName || DEFAULT_GRID_CLASS}>
      <div className="col-span-full text-center text-gray-500">
        No items found.
      </div>
    </div>
  );
}

export default UnifiedGrid;
