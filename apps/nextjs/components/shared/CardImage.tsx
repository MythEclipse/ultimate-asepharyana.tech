'use client';

import React, { useCallback } from 'react';
import Image from 'next/image';
import { StaticImageData } from 'next/image';
import { cn } from '../../utils/utils';
import { useEnhancedImageFallback } from '../../utils/hooks/useEnhancedImageFallback';

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

export const CardImage = React.memo(
  ({
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
    const {
      src: currentSrc,
      onError,
      onLoad: handleFallbackLoad,
    } = useEnhancedImageFallback({
      imageUrl: typeof src === 'string' ? src : undefined,
      fallbackUrl,
      useProxy,
      useCdn,
      resetOnUrlChange: true,
      maxRetries: 3,
    });

    const handleImageLoad = useCallback(() => {
      handleFallbackLoad();
      onLoad?.();
    }, [handleFallbackLoad, onLoad]);

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
            onError={onError}
            onLoad={handleImageLoad}
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
        onError={onError}
        onLoad={handleImageLoad}
        placeholder={typeof currentSrc === 'object' ? placeholder : 'empty'}
      />
    );
  },
);

CardImage.displayName = 'CardImage';
