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
    <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300 transform group-hover:scale-[1.02]">
      <div className="relative h-48 overflow-hidden">
        <Image
          src={imageSrc}
          alt={title}
          fill
          className="object-cover transition-transform duration-300 group-hover:scale-110"
          priority
          unoptimized
        />
      </div>
      <div className="p-6">
        <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
          {title}
        </h3>
        <p className="text-gray-600 dark:text-gray-300 text-sm">
          {description}
        </p>
      </div>
    </div>
  </Link>
));

ProjectCard.displayName = 'ProjectCard';

const CardSkeleton = React.memo(() => (
  <div className="bg-gray-200 dark:bg-gray-700 rounded-xl shadow-lg overflow-hidden animate-pulse">
    <div className="h-48 bg-gray-300 dark:bg-gray-600"></div>
    <div className="p-6 space-y-3">
      <div className="h-6 bg-gray-300 dark:bg-gray-600 rounded w-3/4"></div>
      <div className="h-4 bg-gray-300 dark:bg-gray-600 rounded w-full"></div>
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
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-6">
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
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-6">
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
