'use client';

import React, { useState, useEffect, useRef, memo, useMemo, useCallback } from 'react';
import Image from 'next/image';
import { useRouter } from 'next/navigation';
import { cn } from '@/lib/utils';
import { PRODUCTION } from '@/lib/url';

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';

interface CardProps {
  title?: string;
  description?: string;
  imageUrl?: string;
  linkUrl?: string;
  type?: string;
  badge?: string;
  loading?: boolean;
}

const typeColors: { [key: string]: string } = {
  Manga: 'bg-red-500 hover:bg-red-600',
  Manhua: 'bg-green-500 hover:bg-green-600',
  Manhwa: 'bg-blue-500 hover:bg-blue-600',
  BD: 'bg-purple-500 hover:bg-purple-600',
  TV: 'bg-yellow-500 hover:bg-yellow-600',
  OVA: 'bg-pink-500 hover:bg-pink-600',
  ONA: 'bg-indigo-500 hover:bg-indigo-600',
};

const TypeBadge = memo(({ type, badge }: { type?: string; badge?: string }) => {
  const label = badge || type;
  if (!label) return null;

  const colorClass = typeColors[label] || 'bg-gray-500 hover:bg-gray-600';

  return (
    <Badge
      className={cn('absolute top-2 right-2 text-white border-0', colorClass)}
    >
      {label}
    </Badge>
  );
});

TypeBadge.displayName = 'TypeBadge';

const CardSkeleton = memo(() => {
  return (
    <Card className='w-full max-w-sm overflow-hidden'>
      <div className='relative h-64'>
        <Skeleton className='h-full w-full rounded-t-md rounded-b-none' />
      </div>
      <CardHeader>
        <Skeleton className='h-5 w-3/4' />
      </CardHeader>
      <CardContent>
        <Skeleton className='h-4 w-full' />
        <Skeleton className='h-4 w-5/6 mt-2' />
      </CardContent>
    </Card>
  );
});

CardSkeleton.displayName = 'CardSkeleton';

function CardA({
  title,
  description,
  imageUrl,
  linkUrl,
  type,
  badge,
  loading,
}: CardProps) {
  const [isImageLoading, setIsImageLoading] = useState(true);
  const [currentImageIndex, setCurrentImageIndex] = useState(0);
  const router = useRouter();
  const fallbackImage = '/default.png';

  const imageSources = useMemo(
    () =>
      [
        imageUrl,
        imageUrl
          ? `https://imagecdn.app/v1/images/${encodeURIComponent(imageUrl)}`
          : null,
        imageUrl
          ? `${PRODUCTION}/api/img-compress2?url=${encodeURIComponent(imageUrl)}`
          : null,
        imageUrl
          ? `${PRODUCTION}/api/img-compress3?url=${encodeURIComponent(imageUrl)}`
          : null,
        imageUrl
          ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(imageUrl)}`
          : null,
        fallbackImage,
      ].filter(Boolean) as string[],
    // Remove PRODUCTION from dependency array to fix warning
    [imageUrl]
  );

  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    if (isImageLoading) {
      timeoutRef.current = setTimeout(() => {
        if (currentImageIndex < imageSources.length - 1) {
          setCurrentImageIndex((prev) => prev + 1);
        } else {
          setIsImageLoading(false);
        }
      }, 5000);
    }
    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, [currentImageIndex, isImageLoading, imageSources.length]);

  const handleImageError = useCallback(() => {
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    if (currentImageIndex < imageSources.length - 1) {
      setCurrentImageIndex(currentImageIndex + 1);
    } else {
      setIsImageLoading(false);
    }
  }, [currentImageIndex, imageSources.length]);

  const handleImageLoad = useCallback(() => {
    setIsImageLoading(false);
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
  }, []);

  const handleCardClick = useCallback(() => {
    router.push(linkUrl || '/');
  }, [router, linkUrl]);

  if (loading) {
    return <CardSkeleton />;
  }

  return (
    <Card
      onClick={handleCardClick}
      className='w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl'
    >
      <div className='relative h-64'>
        {isImageLoading && (
          <Skeleton className='absolute inset-0 h-full w-full rounded-t-md rounded-b-none' />
        )}
        <Image
          src={imageSources[currentImageIndex]}
          alt={title || 'Card Image'}
          fill
          sizes='(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw'
          className={cn(
            'object-cover transition-opacity duration-300',
            isImageLoading ? 'opacity-0' : 'opacity-100'
          )}
          onLoad={handleImageLoad}
          onError={handleImageError}
          unoptimized
        />
        <TypeBadge type={type} badge={badge} />
      </div>
      <CardHeader>
        <CardTitle className='truncate'>{title}</CardTitle>
        {description && (
          <CardDescription className='line-clamp-2'>
            {description}
          </CardDescription>
        )}
      </CardHeader>
    </Card>
  );
}

export default memo(CardA);
