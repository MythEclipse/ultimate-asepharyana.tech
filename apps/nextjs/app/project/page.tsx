'use client';

import React, { useState, useEffect } from 'react';
import { useTheme } from 'next-themes';
import Image from 'next/image';
import Link from 'next/link';
import { SectionHeader } from '../../components/shared/SectionHeader';
import { FolderKanban } from 'lucide-react';

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

// Types
interface ProjectImage {
  light: import('next/image').StaticImageData;
  dark: import('next/image').StaticImageData;
}

interface Project {
  title: string;
  description: string;
  images: ProjectImage;
  linkUrl: string;
}

interface CardProps {
  title: string;
  description: string;
  imageSrc: import('next/image').StaticImageData;
  linkUrl: string;
}

// Project data configuration
const PROJECTS: Project[] = [
  {
    title: 'Anime',
    description: 'Anime scraping dari otakudesu.cloud',
    images: { light: webAnimeL, dark: webAnime },
    linkUrl: '/anime',
  },
  {
    title: 'Anime2',
    description: 'Anime scraping dari alqanime.net',
    images: { light: webAnimeL, dark: webAnime },
    linkUrl: '/anime2',
  },
  {
    title: 'Komik',
    description: 'Komik scraping dari komikindo1.com',
    images: { light: webKomikL, dark: webKomik },
    linkUrl: '/komik',
  },
  {
    title: 'Sosmed',
    description: 'Autentikasi & CRUD dasar',
    images: { light: webSosmedL, dark: webSosmed },
    linkUrl: '/sosmed',
  },
  {
    title: 'Chat',
    description: 'Chat realtime dengan WebSocket',
    images: { light: webChatL, dark: webChat },
    linkUrl: '/chat',
  },
  {
    title: 'Compressor',
    description: 'Kompressor gambar dan video',
    images: { light: webCompressorL, dark: webCompressor },
    linkUrl: '/compressor',
  },
];

// Components
const ProjectCard = React.memo(({ title, description, imageSrc, linkUrl }: CardProps) => (
  <Link href={linkUrl} className="block group">
    <article className="relative bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-2xl shadow-xl overflow-hidden border border-gray-200 dark:border-gray-700 hover:shadow-2xl hover:border-blue-400 dark:hover:border-blue-500 transition-all duration-500 transform group-hover:scale-[1.03] group-hover:-translate-y-1">
      {/* Image Container with Overlay */}
      <div className="relative h-56 overflow-hidden bg-gradient-to-br from-blue-500 to-purple-600">
        <Image
          src={imageSrc}
          alt={title}
          fill
          className="object-cover transition-all duration-700 group-hover:scale-110 group-hover:brightness-110"
          priority
          unoptimized
        />
        {/* Gradient Overlay */}
        <div className="absolute inset-0 bg-gradient-to-t from-black/60 via-black/20 to-transparent opacity-60 group-hover:opacity-40 transition-opacity duration-300"></div>
      </div>

      {/* Content */}
      <div className="p-6 space-y-3">
        <h3 className="text-2xl font-bold text-gray-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors duration-300 flex items-center gap-2">
          {title}
          <svg className="w-5 h-5 transform transition-transform duration-300 group-hover:translate-x-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
        </h3>
        <p className="text-gray-600 dark:text-gray-400 text-sm leading-relaxed">
          {description}
        </p>
      </div>

      {/* Bottom Accent Line */}
      <div className="h-1 w-0 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 group-hover:w-full transition-all duration-500 ease-out"></div>
    </article>
  </Link>
));

ProjectCard.displayName = 'ProjectCard';

const CardSkeleton = React.memo(() => (
  <div className="bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800 rounded-2xl shadow-xl overflow-hidden border border-gray-200 dark:border-gray-700 animate-pulse">
    <div className="h-56 bg-gradient-to-br from-gray-300 to-gray-400 dark:from-gray-600 dark:to-gray-700"></div>
    <div className="p-6 space-y-4">
      <div className="h-7 bg-gray-300 dark:bg-gray-600 rounded-lg w-2/3"></div>
      <div className="space-y-2">
        <div className="h-4 bg-gray-300 dark:bg-gray-600 rounded w-full"></div>
        <div className="h-4 bg-gray-300 dark:bg-gray-600 rounded w-4/5"></div>
      </div>
    </div>
  </div>
));

CardSkeleton.displayName = 'CardSkeleton';

export default function Page() {
  const [mounted, setMounted] = useState(false);
  const { resolvedTheme } = useTheme();

  useEffect(() => {
    setMounted(true);
  }, []);

  const isLightTheme = resolvedTheme === 'light';

  // Show skeleton while mounting to prevent hydration mismatch
  if (!mounted) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto space-y-8">
          <SectionHeader
            icon={FolderKanban}
            title="Project Terbaru"
            subtitle="Berikut adalah kumpulan project yang saya buat"
            color="blue"
          />
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
            {Array.from({ length: 6 }).map((_, index) => (
              <CardSkeleton key={`skeleton-${index}`} />
            ))}
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
        <SectionHeader
          icon={FolderKanban}
          title="Project Terbaru"
          subtitle="Berikut adalah kumpulan project yang saya buat"
          color="blue"
        />
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
          {PROJECTS.map((project) => (
            <ProjectCard
              key={project.linkUrl}
              title={project.title}
              description={project.description}
              imageSrc={isLightTheme ? project.images.light : project.images.dark}
              linkUrl={project.linkUrl}
            />
          ))}
        </div>
      </div>
    </main>
  );
}
