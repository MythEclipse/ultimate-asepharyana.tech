import React, { memo, useMemo } from 'react';
import dynamic from 'next/dynamic';
import { FixedSizeGrid as Grid } from 'react-window';

const CardA = dynamic(() => import('./MediaCard'), { ssr: false });

interface Komik {
  title?: string;
  poster?: string;
  chapter?: string;
  score?: string;
  date?: string;
  type?: string;
  komik_id?: string;
  slug: string;
}

interface KomikGridProps {
  komiks?: Komik[];
  loading?: boolean;
  komik?: Komik;
  loading2?: boolean;
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

const KomikGrid: React.FC<KomikGridProps> = memo(({
  komiks,
  komik,
  loading,
  loading2 = false,
}) => {
  const loadingCards = useMemo(
    () =>
      Array.from({ length: 40 }).map((_, index) => (
        <CardA key={index} loading={loading} />
      )),
    [loading]
  );

  const komikList = komiks || [];

  const rowCount = Math.ceil(komikList.length / COLUMN_COUNT);

  const Cell = ({ columnIndex, rowIndex, style }: GridCellProps) => {
    const index = rowIndex * COLUMN_COUNT + columnIndex;
    if (index >= komikList.length) return null;
    const komik = komikList[index];
    return (
      <div style={style}>
        <CardA
          key={komik.slug}
          title={komik.title}
          description={''}
          imageUrl={komik.poster || ''}
          linkUrl={`/komik/detail/${komik.slug}`}
        />
      </div>
    );
  };

  if (komik) {
    return (
      <CardA
        key={komik.slug}
        title={komik.title}
        description={''}
        imageUrl={komik.poster || ''}
        linkUrl={`/komik/detail/${komik.slug}`}
      />
    );
  }
  if (loading2) {
    return <CardA loading={true} />;
  }
  if (loading) {
    return (
      <div className='flex flex-col items-center p-4'>
        <div className='grid grid-cols-3 lg:grid-cols-5 gap-4 w-full'>
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

KomikGrid.displayName = 'ComicGrid';

export default KomikGrid;
