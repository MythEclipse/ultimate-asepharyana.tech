'use client';

import React, { useState, useEffect, useRef } from 'react';
import Image from 'next/image';

interface ImageWithFallbackProps {
  imageUrl: string;
  index: number;
}

export const ImageWithFallback = ({
  imageUrl,
  index,
}: ImageWithFallbackProps) => {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isLoading, setIsLoading] = useState(true);
  const BaseUrl = process.env.NEXT_PUBLIC_BASE_URL;
  const fallback = '/default.png';

  const imageSources = [
    imageUrl && imageUrl.trim() !== '' ? imageUrl : null,
    `https://imagecdn.app/v1/images/${encodeURIComponent(imageUrl || fallback)}`,
    `${BaseUrl}/api/img-compress2?url=${encodeURIComponent(imageUrl || fallback)}`,
    `${BaseUrl}/api/img-compress3?url=${encodeURIComponent(imageUrl || fallback)}`,
    `${BaseUrl}/api/imageproxy?url=${encodeURIComponent(imageUrl || fallback)}`,
    fallback,
  ].filter(Boolean) as string[];

  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    setCurrentIndex(0);
    setIsLoading(true);
  }, [imageUrl]);

  useEffect(() => {
    if (isLoading) {
      timeoutRef.current = setTimeout(() => {
        if (currentIndex < imageSources.length - 1) {
          setCurrentIndex((prev) => prev + 1);
        } else {
          setIsLoading(false);
        }
      }, 4000);
    }

    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, [currentIndex, isLoading, imageSources.length]);

  const handleError = () => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
    } else {
      setIsLoading(false);
    }
  };

  return (
    <div
      style={{
        position: 'relative',
        width: '100%',
        minHeight: '300px',
        backgroundColor: '#f0f0f0',
      }}
    >
      <Image
        src={imageSources[currentIndex]}
        alt={`Chapter Page ${index + 1}`}
        className='w-full h-auto rounded-xl shadow-lg border border-zinc-200 dark:border-zinc-700 object-contain'
        width={725}
        height={1024}
        unoptimized
        onError={handleError}
        onLoad={() => {
          setIsLoading(false);
          if (timeoutRef.current) clearTimeout(timeoutRef.current);
        }}
      />
    </div>
  );
};
