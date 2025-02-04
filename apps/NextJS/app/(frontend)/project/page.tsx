'use client';

import TildCard from '@/components/card/TildCard';
import React from 'react';
import { useTheme } from 'next-themes';

// Import gambar lokal
import webAnimeL from '@/public/webAnimeL.png';
import webAnime from '@/public/webAnime.png';
import webKomikL from '@/public/webKomikL.png';
import webKomik from '@/public/webKomik.png';
import webSosmedL from '@/public/websosmedL.png';
import webSosmed from '@/public/websosmed.png';
import webChatL from '@/public/webChatL.png';
import webChat from '@/public/webChat.png';
import webCompressorL from '@/public/WebCompressorL.png';
import webCompressor from '@/public/WebCompressor.png';

export default function Page() {
  const { theme, resolvedTheme } = useTheme();
  const isLightTheme = theme === 'light' || resolvedTheme === 'light';

  return (
    <div className='container mx-auto p-4'>
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
            title='Komik'
            description='Komik scraping dari komikindo.pw'
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
