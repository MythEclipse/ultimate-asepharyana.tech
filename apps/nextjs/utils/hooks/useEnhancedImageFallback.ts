// Enhanced image fallback hook that consolidates all image handling logic
import { useState, useCallback, useMemo, useEffect } from 'react';
import { StaticImageData } from 'next/image';
import { generateImageSources } from '../image-proxy';
import { ImageFallbackOptions } from '../../types/image';

export interface EnhancedImageFallbackOptions extends ImageFallbackOptions {
  resetOnUrlChange?: boolean;
  maxRetries?: number;
}

export interface UseEnhancedImageFallbackReturn {
  src: string | StaticImageData;
  onError: () => void;
  onLoad: () => void;
  hasError: boolean;
  currentIndex: number;
  imageSources: (string | StaticImageData)[];
  retryCount: number;
}

export function useEnhancedImageFallback({
  imageUrl,
  fallbackUrl = '/default.png',
  useProxy = true,
  useCdn = true,
  resetOnUrlChange = true,
  maxRetries = 3,
}: {
  imageUrl?: string | StaticImageData;
  fallbackUrl?: string;
  useProxy?: boolean;
  useCdn?: boolean;
  resetOnUrlChange?: boolean;
  maxRetries?: number;
}): UseEnhancedImageFallbackReturn {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [hasError, setHasError] = useState(false);
  const [retryCount, setRetryCount] = useState(0);

  const imageSources = useMemo(() => {
    // If imageUrl is StaticImageData, use it directly
    if (typeof imageUrl === 'object') {
      return [imageUrl];
    }

    const options: ImageFallbackOptions = {
      fallbackUrl,
      useProxy,
      useCdn,
    };

    return generateImageSources(imageUrl || '', options);
  }, [imageUrl, fallbackUrl, useProxy, useCdn]);

  // Reset state when imageUrl changes
  useEffect(() => {
    if (resetOnUrlChange) {
      setCurrentIndex(0);
      setHasError(false);
      setRetryCount(0);
    }
  }, [imageUrl, resetOnUrlChange]);

  const handleError = useCallback(() => {
    if (currentIndex < imageSources.length - 1) {
      setCurrentIndex((prev) => prev + 1);
      setHasError(true);
    } else if (retryCount < maxRetries - 1) {
      // Retry from the beginning if we haven't exceeded max retries
      setCurrentIndex(0);
      setRetryCount((prev) => prev + 1);
    }
  }, [currentIndex, imageSources.length, retryCount, maxRetries]);

  const handleLoad = useCallback(() => {
    setHasError(false);
  }, []);

  return {
    src: imageSources[currentIndex],
    onError: handleError,
    onLoad: handleLoad,
    hasError,
    currentIndex,
    imageSources,
    retryCount,
  };
}
