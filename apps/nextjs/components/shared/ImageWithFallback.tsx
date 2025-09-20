'use client';

import React, { useState, useEffect } from 'react';
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
  const [hasError, setHasError] = useState(false);
  const BaseUrl = process.env.NEXT_PUBLIC_BASE_URL || '';

  const normalizeImageUrl = (url: string) => {
    if (!url || url.startsWith('http://') || url.startsWith('https://')) {
      return url;
    }
    // Handle relative paths like /asset/img/komikuplus2.jpg
    if (url.startsWith('/')) {
      return `${window.location.origin}${url}`;
    }
    return `${window.location.origin}/${url}`;
  };

  const normalizedImageUrl = normalizeImageUrl(imageUrl);
  const fallback = 'default.png';
  const imageSources = [
    normalizedImageUrl && normalizedImageUrl.trim() !== ''
      ? normalizedImageUrl
      : null,
    `${BaseUrl}/api/imageproxy?url=${encodeURIComponent(
      normalizedImageUrl || fallback,
    )}`,
  ].filter(Boolean) as string[];

  useEffect(() => {
    // Reset state ketika imageUrl berubah
    setCurrentIndex(0);
    setHasError(false);
  }, [imageUrl]);

  const handleError = () => {
    if (!hasError && currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
      setHasError(true);
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
        className="w-full h-auto rounded-xl shadow-lg border border-zinc-200 dark:border-zinc-700"
        width={725}
        height={1024}
        unoptimized
        onError={handleError}
        onLoad={() => setHasError(false)}
      />
    </div>
  );
};
