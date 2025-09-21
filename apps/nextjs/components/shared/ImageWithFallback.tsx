'use client';

import React, { useState, useEffect } from 'react';
import Image from 'next/image';
import { generateImageSources } from '../../utils/image-proxy-client';
import { ImageFallbackOptions } from '../../types/image';

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
  const [currentIndex, setCurrentIndex] = useState(0);
  const [hasError, setHasError] = useState(false);

  const imageSources = React.useMemo(() => {
    const options: ImageFallbackOptions = {
      fallbackUrl,
      useProxy,
      useCdn,
    };

    return generateImageSources(imageUrl, options);
  }, [imageUrl, fallbackUrl, useProxy, useCdn]);

  useEffect(() => {
    // Reset state when imageUrl changes
    setCurrentIndex(0);
    setHasError(false);
  }, [imageUrl]);

  const handleError = () => {
    if (!hasError && currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
      setHasError(true);
    }
  };

  const handleLoad = () => {
    setHasError(false);
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
        alt={alt || `Chapter Page ${index + 1}`}
        className={className}
        width={width}
        height={height}
        unoptimized
        onError={handleError}
        onLoad={handleLoad}
      />
    </div>
  );
};
