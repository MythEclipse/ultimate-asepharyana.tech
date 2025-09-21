'use client';

import React, { useCallback } from 'react';
import Image from 'next/image';
import { useEnhancedImageFallback } from '../../utils/hooks/useEnhancedImageFallback';

interface ImageWithFallbackProps {
  imageUrl: string;
  index: number;
  fallbackUrl?: string;
  useProxy?: boolean;
  useCdn?: boolean;
  width?: number;
  height?: number;
  className?: string;
  alt?: string;
}

export const ImageWithFallback = ({
  imageUrl,
  index,
  fallbackUrl = 'default.png',
  useProxy = true,
  useCdn = true,
  width = 725,
  height = 1024,
  className = 'w-full h-auto rounded-xl shadow-lg border border-zinc-200 dark:border-zinc-700',
  alt,
}: ImageWithFallbackProps) => {
  const { src: currentSrc, onError, onLoad } = useEnhancedImageFallback({
    imageUrl,
    fallbackUrl,
    useProxy,
    useCdn,
    resetOnUrlChange: true,
    maxRetries: 3,
  });

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
        src={currentSrc}
        alt={alt || `Chapter Page ${index + 1}`}
        className={className}
        width={width}
        height={height}
        unoptimized
        onError={onError}
        onLoad={onLoad}
      />
    </div>
  );
};
