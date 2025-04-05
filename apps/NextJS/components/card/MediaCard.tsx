'use client';
import React, { useState } from 'react';
import { Card as ShadcnCard } from '@/components/card/ComponentCard';
import Image from 'next/image';
import { PRODUCTION } from '@/lib/url';
import { useTransitionRouter } from 'next-view-transitions';

interface CardProps {
  title?: string;
  description?: string;
  imageUrl?: string;
  linkUrl?: string;
  type?: string;
  badge?: string;
  loading?: boolean;
}

const TypeLabel = ({ type, badge }: { type?: string; badge?: string }) => {
  const label = badge || type;
  if (!label) return null;
  const typeColors = {
    Manga: 'bg-red-500',
    Manhua: 'bg-green-500',
    Manhwa: 'bg-blue-500',
    BD: 'bg-purple-500',
    TV: 'bg-yellow-500',
    OVA: 'bg-pink-500',
    ONA: 'bg-indigo-500',
  };
  return (
    <span
      className={`absolute top-2 right-2 px-2 py-1 rounded-md text-white text-sm font-bold ${typeColors[label as keyof typeof typeColors] || 'bg-gray-500'}`}
    >
      {label}
    </span>
  );
};

export default function CardA({
  title,
  description,
  imageUrl,
  linkUrl,
  type,
  badge,
  loading,
}: CardProps) {
  const [isLoading, setIsLoading] = useState(true);
  const [currentIndex, setCurrentIndex] = useState(0);
  const router = useTransitionRouter();
  const fallback = 'https://asepharyana.cloud/default.png';
  const imageSources = [
    imageUrl && imageUrl.trim() ? imageUrl : fallback,
    imageUrl && imageUrl.trim()
      ? `https://imagecdn.app/v1/images/${encodeURIComponent(imageUrl)}`
      : null,
    imageUrl && imageUrl.trim()
      ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(imageUrl)}`
      : null,
      
  ].filter((src) => src && src.trim()) as string[];

  const handleError = () => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex(currentIndex + 1);
    }
  };
  return (
    <button
      onClick={() => router.push(linkUrl || '/')}
      className='cursor-pointer transform transition-transform duration-300 hover:scale-105 hover:rotate-1 active:scale-95 focus:outline-none'
    >
      <ShadcnCard className='w-full max-w-sm sm:max-w-md md:max-w-lg lg:max-w-xl bg-white dark:bg-black overflow-hidden transform transition-transform duration-300 hover:shadow-2xl text-blue-500 border border-blue-500 rounded-xl shadow-lg shadow-blue-500/50 hover:bg-blue-500 hover:text-white focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50 hover:ring-4 hover:ring-gradient-to-r hover:from-blue-400 hover:via-purple-500 hover:to-pink-500'>
        <div className='relative h-48 sm:h-56 md:h-64 lg:h-72'>
          {loading ? (
            <div className='absolute inset-0 bg-gray-300 animate-pulse rounded-t-xl border border-gray-700' />
          ) : (
            <Image
              src={imageSources[currentIndex]}
              alt={title || 'Image'}
              fill
              sizes='(max-width: 640px) 100vw, (max-width: 768px) 50vw, (max-width: 1024px) 33vw, 25vw'
              className='object-cover transition-opacity duration-500 ease-in-out rounded-t-xl'
              onLoad={() => setIsLoading(false)}
              onError={handleError}
              unoptimized
            />
          )}
          <TypeLabel type={type} badge={badge} />
        </div>
        <div className='p-4'>
          <h3
            className={`text-xs sm:text-sm md:text-lg font-bold text-black dark:text-gray-200 truncate ${loading ? 'bg-gray-300 animate-pulse h-5 w-3/4 rounded' : ''}`}
          >
            {(!loading || !isLoading) && title}
          </h3>
          {description && (
            <p
              className={`text-sm sm:text-sm md:text-base text-gray-600 dark:text-gray-400 mt-2 ${loading ? 'bg-gray-300 animate-pulse h-4 w-full rounded' : ''}`}
            >
              {(!loading || !isLoading) && description}
            </p>
          )}
        </div>
      </ShadcnCard>
    </button>
  );
}
