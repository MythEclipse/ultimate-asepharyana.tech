import React from 'react';
import CardA from './MediaCard';

interface Komik {
  title: string;
  poster: string;
  chapter: string;
  score: string;
  date: string;
  type: string;
  komik_id: string;
  slug: string;
}

interface KomikGridProps {
  komiks?: Komik[];
  loading?: boolean;
}

const KomikGrid: React.FC<KomikGridProps> = ({ komiks, loading = false }) => {
  if (loading) {
    return (
      <div className='flex flex-col items-center p-4'>
        <div className='grid grid-cols-5 gap-4 w-full'>
          {Array.from({ length: 40 }).map((_, index) => (
            <CardA key={index} loading={loading} />
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className='flex flex-col items-center p-4'>
      <div className='grid grid-cols-5 gap-4 w-full'>
        {komiks?.map((komik) => (
          <CardA
            key={komik.slug}
            title={komik.title}
            description={''}
            imageUrl={komik.poster || ''}
            linkUrl={`/komik/detail/${komik.slug}`}
          />
        ))}
      </div>
    </div>
  );
};

export default KomikGrid;
