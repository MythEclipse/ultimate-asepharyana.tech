'use client';

import React from 'react';
import Image from 'next/image';
import { useEnhancedImageFallback } from '../../utils/hooks/useEnhancedImageFallback';

interface PosterImageProps {
  poster: string;
  title: string;
  width?: number;
  height?: number;
  className?: string;
  useProxy?: boolean;
  useCdn?: boolean;
}

export default function PosterImage({
  poster,
  title,
  width = 400,
  height = 600,
  className = 'object-cover w-full aspect-[2/3]',
  useProxy = true,
  useCdn = true,
}: PosterImageProps) {
  const { src, onError } = useEnhancedImageFallback({
    imageUrl: poster,
    useProxy,
    useCdn
  });

  return (
    <Image
      src={src}
      alt={title}
      width={width}
      height={height}
      className={className}
      priority
      unoptimized
      onError={onError}
    />
  );
}
