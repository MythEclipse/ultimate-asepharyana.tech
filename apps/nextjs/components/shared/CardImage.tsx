'use client';

import React, { useState, useCallback, useMemo } from 'react';
import Image from 'next/image';
import { StaticImageData } from 'next/image';
import { generateImageSources } from '../../utils/image-proxy';
import { ImageFallbackOptions } from '../../types/image';
import { cn } from '../../utils/utils';
import { PRODUCTION } from '../../utils/url-utils';

export interface CardImageProps {
  src: string | StaticImageData;
  alt: string;
  width?: number;
  height?: number;
  className?: string;
  priority?: boolean;
  unoptimized?: boolean;
  useProxy?: boolean;
  useCdn?: boolean;
  fallbackUrl?: string;
  fill?: boolean;
  sizes?: string;
  placeholder?: 'blur' | 'empty';
  onLoad?: () => void;
}

export const CardImage = React.memo(({
  src,
  alt,
  width = 300,
  height = 400,
  className,
  priority = false,
  unoptimized = false,
  useProxy = true,
  useCdn = true,
  fallbackUrl = '/default.png',
  fill = false,
  sizes = '(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw',
  placeholder = 'empty',
  onLoad,
}: CardImageProps) => {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [hasError, setHasError] = useState(false);

  const imageSources = useMemo(() => {
    // If src is StaticImageData, use it directly
    if (typeof src === 'object') {
      return [src];
    }

    const options: ImageFallbackOptions = {
      fallbackUrl,
      useProxy,
      useCdn,
    };

    return generateImageSources(src, options);
  }, [src, fallbackUrl, useProxy, useCdn]);

  const handleError = useCallback(() => {
    if (!hasError && currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
      setHasError(true);
    }
  }, [hasError, currentIndex, imageSources.length]);

  const handleLoad = useCallback(() => {
    setHasError(false);
    onLoad?.();
  }, [onLoad]);

  const currentSrc = imageSources[currentIndex];

  if (fill) {
    return (
      <div className="relative w-full h-full">
        <Image
          src={currentSrc}
          alt={alt}
          fill
          sizes={sizes}
          className={cn('object-cover', className)}
          priority={priority}
          unoptimized={unoptimized}
          onError={handleError}
          onLoad={handleLoad}
          placeholder={typeof currentSrc === 'object' ? placeholder : 'empty'}
        />
      </div>
    );
  }

  return (
    <Image
      src={currentSrc}
      alt={alt}
      width={width}
      height={height}
      className={cn('object-cover', className)}
      priority={priority}
      unoptimized={unoptimized}
      onError={handleError}
      onLoad={handleLoad}
      placeholder={typeof currentSrc === 'object' ? placeholder : 'empty'}
    />
  );
});

CardImage.displayName = 'CardImage';
