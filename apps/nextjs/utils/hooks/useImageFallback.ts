// apps/nextjs/utils/hooks/useImageFallback.ts
import { useState, useCallback, useMemo } from 'react';
import { generateImageSources } from '../image-proxy';
import { ImageFallbackOptions } from '../../types/image';

interface UseImageFallbackProps {
  imageUrl?: string;
  fallbackUrl?: string;
  useProxy?: boolean;
  useCdn?: boolean;
}

export function useImageFallback({
  imageUrl,
  fallbackUrl = '/default.png',
  useProxy = true,
  useCdn = true
}: UseImageFallbackProps) {
  const [currentIndex, setCurrentIndex] = useState(0);

  const imageSources = useMemo(() => {
    const options: ImageFallbackOptions = {
      fallbackUrl,
      useProxy,
      useCdn,
    };

    return generateImageSources(imageUrl || '', options);
  }, [imageUrl, fallbackUrl, useProxy, useCdn]);

  const handleError = useCallback(() => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
    }
  }, [currentIndex, imageSources.length]);

  return {
    src: imageSources[currentIndex],
    onError: handleError,
    imageSources, // Expose for debugging/testing
    currentIndex, // Expose for debugging/testing
  };
}
