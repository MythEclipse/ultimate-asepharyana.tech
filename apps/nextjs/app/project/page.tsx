'use client';

import { TildCard } from '../../components/ui/CardSystem';
import React, { useState, useEffect } from 'react';
import { useTheme } from 'next-themes';

// Import local images
import webAnimeL from '../../public/webAnimeL.png';
import webAnime from '../../public/webAnime.png';
import webKomikL from '../../public/webKomikL.png';
import webKomik from '../../public/webKomik.png';
import webSosmedL from '../../public/websosmedL.png';
import webSosmed from '../../public/websosmed.png';
import webChatL from '../../public/webChatL.png';
import webChat from '../../public/webChat.png';
import webCompressorL from '../../public/WebCompressorL.png';
import webCompressor from '../../public/WebCompressor.png';

// Simple Skeleton component for TildCard
const TildCardSkeleton = () => (
  <div className="bg-gray-200 dark:bg-gray-700 rounded-lg shadow-lg p-4 animate-pulse">
    <div className="h-40 bg-gray-300 dark:bg-gray-600 rounded mb-4"></div>
    <div className="h-6 bg-gray-300 dark:bg-gray-600 rounded w-3/4 mb-2"></div>
    <div className="h-4 bg-gray-300 dark:bg-gray-600 rounded w-1/2"></div>
  </div>
);

export default function Page() {
  const [mounted, setMounted] = useState(false);
  const { resolvedTheme } = useTheme();

  // useEffect only runs on client-side after component mount
  useEffect(() => {
    setMounted(true);
  }, []);

  // Header can be rendered directly because it doesn't depend on theme-changing images
  const headerContent = (
    <div className="w-full">
      <div className="mx-auto mb-16 max-w-xl text-center">
        <h2 className="mb-4 text-3xl font-bold text-dark dark:text-white">
          Project terbaru
        </h2>
        <p className="text-md font-medium text-secondary dark:text-white">
          Berikut adalah kumpulan Project yang saya buat
        </p>
      </div>
    </div>
  );

  // If not mounted, show skeleton or null to avoid flash
  if (!mounted) {
    return (
      <div className="container mx-auto p-4">
        {headerContent}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-3">
          {/* Show skeleton for each card */}
          {Array(6)
            .fill(0)
            .map((_, index) => (
              <div key={`skeleton-${index}`}>
                {' '}
                {/* Ensure unique key */}
                <TildCardSkeleton />
              </div>
            ))}
        </div>
      </div>
    );
  }

  // After mounted, resolvedTheme is accurate
  const isLightTheme = resolvedTheme === 'light';

  return (
    <div className="container mx-auto p-4">
      {headerContent}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-3">
        <div>
          <TildCard
            title="Anime"
            description="Anime scrapping dari otakudesu.cloud"
            image={{
              src: isLightTheme ? webAnimeL : webAnime,
              alt: 'Anime',
              priority: true,
              unoptimized: true
            }}
            linkUrl="/anime"
          />
        </div>
        <div>
          <TildCard
            title="Anime2"
            description="Anime scrapping dari alqanime.net"
            image={{
              src: isLightTheme ? webAnimeL : webAnime,
              alt: 'Anime2',
              priority: true,
              unoptimized: true
            }}
            linkUrl="/anime2"
          />
        </div>
        <div>
          <TildCard
            title="Komik"
            description="Komik scraping dari komikindo1.com"
            image={{
              src: isLightTheme ? webKomikL : webKomik,
              alt: 'Komik',
              priority: true,
              unoptimized: true
            }}
            linkUrl="/komik"
          />
        </div>
        <div>
          <TildCard
            title="Sosmed"
            description="Autentikasi & crud dasar"
            image={{
              src: isLightTheme ? webSosmedL : webSosmed,
              alt: 'Sosmed',
              priority: true,
              unoptimized: true
            }}
            linkUrl="/sosmed"
          />
        </div>
        <div>
          <TildCard
            title="Chat"
            description="Chat dengan websocket"
            image={{
              src: isLightTheme ? webChatL : webChat,
              alt: 'Chat',
              priority: true,
              unoptimized: true
            }}
            linkUrl="/chat"
          />
        </div>
        <div>
          <TildCard
            title="Compressor"
            description="Compressor image dan video"
            image={{
              src: isLightTheme ? webCompressorL : webCompressor,
              alt: 'Compressor',
              priority: true,
              unoptimized: true
            }}
            linkUrl="/compressor"
          />
        </div>
      </div>
    </div>
  );
}
