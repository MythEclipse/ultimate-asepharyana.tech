// UnifiedGrid.tsx
'use client';

import React, { memo, useMemo } from 'react';
import dynamic from 'next/dynamic';

const MediaCard = dynamic(() => import('../anime/MediaCard'), { ssr: false });

export type Anime = {
  slug: string;
  title: string;
  poster?: string;
  current_episode?: string;
  episode?: string;
  episode_count?: string;
  rating?: string;
  release_day?: string;
  newest_release_date?: string;
  anime_url?: string;
};

export type Komik = {
  slug: string;
  title?: string;
  poster?: string;
  chapter?: string;
  chapter_count?: string;
  score?: string;
  date?: string;
  type?: string;
  komik_id?: string;
};

type ItemType = 'anime' | 'komik';

type UnifiedGridProps<T extends Anime | Komik = Anime | Komik> = {
  items?: T[];
  loading?: boolean;
  singleItem?: T;
  itemType: ItemType;
  gridClassName?: string;
  isAnime2?: boolean;
  loading2?: boolean;
};

const DEFAULT_GRID_CLASS = 'grid grid-cols-3 lg:grid-cols-5 gap-6 w-full';

function UnifiedGrid<T extends Anime | Komik>({
  items,
  loading = false,
  singleItem,
  itemType,
  gridClassName,
  isAnime2 = false,
  loading2 = false,
}: UnifiedGridProps<T>) {
  const loadingCards = useMemo(
    () =>
      Array.from({ length: 40 }).map((_, index) => (
        <MediaCard key={index} loading={loading} />
      )),
    [loading],
  );

  // Single item mode (loading2)
  if (loading2) {
    return <MediaCard loading={true} />;
  }

  // Single item mode (actual item)
  if (singleItem) {
    if (itemType === 'anime') {
      const anime = singleItem as Anime;
      const link = isAnime2 ? '/anime2/detail' : '/anime/detail';
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
          linkUrl={`${link}/${anime.slug}`}
        />
      );
    } else {
      const komik = singleItem as Komik;
      return (
        <MediaCard
          key={komik.slug}
          title={komik.title || ''}
          description={''} // ComicGrid always uses empty string for description
          imageUrl={komik.poster || ''}
          linkUrl={`/komik/detail/${komik.slug}`}
        />
      );
    }
  }

  // Loading state for grid
  if (loading) {
    return (
      <div className={gridClassName || DEFAULT_GRID_CLASS} style={{ padding: '1rem' }}>
        {loadingCards}
      </div>
    );
  }

  // Items array mode
  if (items && items.length > 0) {
    return (
      <div className={gridClassName || DEFAULT_GRID_CLASS} style={{ padding: '1rem' }}>
        {items.map((item) => {
          if (itemType === 'anime') {
            const anime = item as Anime;
            const link = isAnime2 ? '/anime2/detail' : '/anime/detail';
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
                linkUrl={`${link}/${anime.slug}`}
              />
            );
          } else {
            const komik = item as Komik;
            return (
              <MediaCard
                key={komik.slug}
                title={komik.title || ''}
                description={''} // ComicGrid always uses empty string for description
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

const MemoizedUnifiedGrid = memo(UnifiedGrid) as typeof UnifiedGrid;
(MemoizedUnifiedGrid as any).displayName = 'UnifiedGrid';

export default MemoizedUnifiedGrid;
