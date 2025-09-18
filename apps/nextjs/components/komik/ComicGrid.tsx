'use client';

import React, { memo, useMemo } from 'react';
import dynamic from 'next/dynamic';

const CardA = dynamic(() => import('../anime/MediaCard'), { ssr: false });

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

const COLUMN_CLASSES = 'grid grid-cols-3 lg:grid-cols-5 gap-6 w-full';

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
      <div className={COLUMN_CLASSES} style={{ padding: '1rem' }}>
        {loadingCards}
      </div>
    );
  }

  return (
    <div className={COLUMN_CLASSES} style={{ padding: '1rem' }}>
      {komikList.map((komik) => (
        <CardA
          key={komik.slug}
          title={komik.title}
          description={''}
          imageUrl={komik.poster || ''}
          linkUrl={`/komik/detail/${komik.slug}`}
        />
      ))}
    </div>
  );
});

KomikGrid.displayName = 'ComicGrid';

export default KomikGrid;
