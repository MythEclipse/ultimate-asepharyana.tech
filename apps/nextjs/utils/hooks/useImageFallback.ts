// apps/nextjs/utils/hooks/useImageFallback.ts
import { useState, useCallback, useMemo } from 'react';
import { PRODUCTION } from '../../utils/constants'; // Centralized PRODUCTION constant

interface UseImageFallbackProps {
  imageUrl?: string;
  fallbackUrl?: string;
}

export function useImageFallback({ imageUrl, fallbackUrl = '/default.png' }: UseImageFallbackProps) {
  const [currentIndex, setCurrentIndex] = useState(0);

  const imageSources = useMemo(() => {
    return [
      imageUrl?.trim() ? imageUrl : null,
      imageUrl?.trim()
        ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(imageUrl)}`
        : null,
      fallbackUrl,
    ].filter(Boolean) as string[];
  }, [imageUrl, fallbackUrl]);

  const handleError = useCallback(() => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
    }
  }, [currentIndex, imageSources.length]);

  return {
    src: imageSources[currentIndex],
    onError: handleError,
  };
}
