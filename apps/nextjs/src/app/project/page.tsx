'use client';

import TildCard from '../../components/ui/TildCard';
import React, { useState, useEffect } from 'react'; // Import useState dan useEffect
import { useTheme } from 'next-themes';

// Import gambar lokal
import webAnimeL from '../../../public/webAnimeL.png';
import webAnime from '../../../public/webAnime.png';
import webKomikL from '../../../public/webKomikL.png';
import webKomik from '../../../public/webKomik.png';
import webSosmedL from '../../../public/websosmedL.png';
import webSosmed from '../../../public/websosmed.png';
import webChatL from '../../../public/webChatL.png';
import webChat from '../../../public/webChat.png';
import webCompressorL from '../../../public/WebCompressorL.png';
import webCompressor from '../../../public/WebCompressor.png';

// Komponen Skeleton sederhana untuk TildCard
const TildCardSkeleton = () => (
  <div className='bg-gray-200 dark:bg-gray-700 rounded-lg shadow-lg p-4 animate-pulse'>
    <div className='h-40 bg-gray-300 dark:bg-gray-600 rounded mb-4'></div>
    <div className='h-6 bg-gray-300 dark:bg-gray-600 rounded w-3/4 mb-2'></div>
    <div className='h-4 bg-gray-300 dark:bg-gray-600 rounded w-1/2'></div>
  </div>
);

export default function Page() {
  const [mounted, setMounted] = useState(false); // State untuk melacak apakah komponen sudah mounted
  const { resolvedTheme } = useTheme();

  // useEffect hanya berjalan di client-side setelah component mount
  useEffect(() => {
    setMounted(true);
  }, []);

  // Header bisa dirender langsung karena tidak bergantung pada gambar yang berubah tema
  const headerContent = (
    <div className='w-full'>
      <div className='mx-auto mb-16 max-w-xl text-center'>
        <h2 className='mb-4 text-3xl font-bold text-dark dark:text-white'>
          Project terbaru
        </h2>
        <p className='text-md font-medium text-secondary dark:text-white'>
          Berikut adalah kumpulan Project yang saya buat
        </p>
      </div>
    </div>
  );

  // Jika belum mounted, tampilkan skeleton atau null untuk menghindari flash
  if (!mounted) {
    return (
      <div className='container mx-auto p-4'>
        {headerContent}
        <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-3'>
          {/* Tampilkan skeleton untuk setiap kartu */}
          {Array(6)
            .fill(0)
            .map((_, index) => (
              <div key={`skeleton-${index}`}>
                {' '}
                {/* Pastikan key unik */}
                <TildCardSkeleton />
              </div>
            ))}
        </div>
      </div>
    );
  }

  // Setelah mounted, resolvedTheme sudah akurat
  const isLightTheme = resolvedTheme === 'light'; // Cukup gunakan resolvedTheme setelah mounted

  return (
    <div className='container mx-auto p-4'>
      {headerContent}
      <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-3'>
        <div>
          <TildCard
            title='Anime'
            description='Anime scrapping dari otakudesu.cloud'
            imageUrl={isLightTheme ? webAnimeL : webAnime}
            linkUrl='/anime'
          />
        </div>
        <div>
          <TildCard
            title='Anime2'
            description='Anime scrapping dari alqanime.net'
            imageUrl={isLightTheme ? webAnimeL : webAnime} // Menggunakan gambar yang sama untuk contoh
            linkUrl='/anime2'
          />
        </div>
        <div>
          <TildCard
            title='Komik'
            description='Komik scraping dari komikindo1.com'
            imageUrl={isLightTheme ? webKomikL : webKomik}
            linkUrl='/komik'
          />
        </div>
        <div>
          <TildCard
            title='Sosmed'
            description='Autentikasi & crud dasar'
            imageUrl={isLightTheme ? webSosmedL : webSosmed}
            linkUrl='/sosmed'
          />
        </div>
        <div>
          <TildCard
            title='Chat'
            description='Chat dengan websocket'
            imageUrl={isLightTheme ? webChatL : webChat}
            linkUrl='/chat'
          />
        </div>
        <div>
          <TildCard
            title='Compressor'
            description='Compressor image dan video'
            imageUrl={isLightTheme ? webCompressorL : webCompressor}
            linkUrl='/compressor'
          />
        </div>
      </div>
    </div>
  );
}
