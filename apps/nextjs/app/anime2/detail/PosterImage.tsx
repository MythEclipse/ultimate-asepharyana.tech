'use client';

import React from 'react';
import Image from 'next/image';
import { useImageFallback } from '../../../utils/hooks/useImageFallback';

interface PosterImageProps {
  poster: string;
  title: string;
}

export default function PosterImage({ poster, title }: PosterImageProps) {
  const { src, onError } = useImageFallback({ imageUrl: poster });

  return (
    <Image
      src={src}
      alt={title}
      width={400}
      height={600}
      className="object-cover w-full aspect-[2/3]"
      priority
      unoptimized
      onError={onError}
    />
  );
}
