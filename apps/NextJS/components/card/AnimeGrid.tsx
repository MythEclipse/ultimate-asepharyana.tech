// components/AnimeGrid.tsx
import React, { memo, useMemo } from 'react';
import dynamic from 'next/dynamic';
import { FixedSizeGrid as Grid } from 'react-window';

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
  anime2?: boolean;
}

type GridCellProps = {
  columnIndex: number;
  rowIndex: number;
  style: React.CSSProperties;
  isScrolling?: boolean;
  isVisible?: boolean;
  key: string;
};

const COLUMN_COUNT = 5;
const CARD_HEIGHT = 320;
const CARD_WIDTH = 200;

// components/AnimeGrid.tsx
const AnimeGrid: React.FC<AnimeGridProps> = memo(({
  animes,
  loading = false,
  anime2,
}) => {
  const link = anime2 ? '/anime2/detail' : '/anime/detail';

  const loadingCards = useMemo(
    () =>
      Array.from({ length: 25 }).map((_, index) => (
        <MediaCard key={index} loading={loading} />
      )),
    [loading]
  );

  const animeList = animes || [];
  const rowCount = Math.ceil(animeList.length / COLUMN_COUNT);

  const Cell = ({ columnIndex, rowIndex, style }: GridCellProps) => {
    const index = rowIndex * COLUMN_COUNT + columnIndex;
    if (index >= animeList.length) return null;
    const anime = animeList[index];
    return (
      <div style={style}>
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
      </div>
    );
  };

  if (loading) {
    return (
      <div className='flex flex-col items-center p-4'>
        <div className='grid  grid-cols-3 lg:grid-cols-5 gap-4 w-full'>
          {loadingCards}
        </div>
      </div>
    );
  }

  return (
    <div className='flex flex-col items-center p-4'>
      <Grid
        columnCount={COLUMN_COUNT}
        columnWidth={CARD_WIDTH}
        height={CARD_HEIGHT * Math.min(rowCount, 3)}
        rowCount={rowCount}
        rowHeight={CARD_HEIGHT}
        width={CARD_WIDTH * COLUMN_COUNT}
      >
        {Cell}
      </Grid>
    </div>
  );
});

AnimeGrid.displayName = 'AnimeGrid';

export default AnimeGrid;
