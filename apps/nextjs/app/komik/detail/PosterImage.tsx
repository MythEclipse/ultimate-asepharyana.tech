'use client';

import React, { useState } from 'react';
import Image from 'next/image';
import { PRODUCTION } from '../../../lib/url';

interface PosterImageProps {
  poster: string;
  title: string;
}

export default function PosterImage({ poster, title }: PosterImageProps) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const fallback = '/default.png';

  const imageSources = [
    poster?.trim() ? poster : null,
    poster?.trim()
      ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(poster)}`
      : null,
    fallback,
  ].filter(Boolean) as string[];

  const handleError = () => {
    if (currentIndex < imageSources.length - 1)
      setCurrentIndex(currentIndex + 1);
  };

  return (
    <Image
      src={imageSources[currentIndex]}
      alt={'Poster image of ' + (title || 'manga')}
      width={400}
      height={600}
      className="object-cover w-full aspect-[2/3]"
      priority
      unoptimized
      onError={handleError}
    />
  );
}
