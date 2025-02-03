// components/AnimeGrid.tsx
import React from 'react';
import CardA from './MediaCard';

interface Komik {
  title: string;
  image: string;
  chapter: string;
  score: string;
  date: string;
  type: string;
  komik_id: string;
}

interface KomikGridProps {
  komiks: Komik[];
}

const KomikGrid: React.FC<KomikGridProps> = ({ komiks }) => {
  return (
    <div className='flex flex-col items-center p-4'>
      <div className='grid grid-cols-3 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-4'>
        {komiks.map((komik) => (
          <CardA
            key={komik.komik_id}
            title={komik.title}
            description={`Chapter: ${komik.chapter} | Date: ${komik.date}`}
            imageUrl={komik.image}
            linkUrl={`/komik/detail/${komik.komik_id}`}
          />
        ))}
      </div>
    </div>
  );
};

export default KomikGrid;
