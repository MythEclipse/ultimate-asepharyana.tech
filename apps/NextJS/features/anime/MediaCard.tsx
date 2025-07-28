// apps/NextJS/features/anime/MediaCard.tsx
'use client';

import React, { useState, useEffect, useRef, memo, useMemo, useCallback } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { cn } from '@/lib/utils';
import { PRODUCTION } from '@/lib/url';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { CardBody, CardContainer, CardItem } from '@core/ui/3d-card';

type DynamicCardProps = {
  title?: string;
  description?: string;
  imageUrl?: string;
  linkUrl?: string;
  type?: string;
  badge?: string;
  loading?: boolean;
  variant?: 'dynamic';
};

type StaticCardProps = {
  title: string;
  description: string;
  imageUrl: string;
  linkUrl: string;
  variant: 'static';
};

type MediaCardProps = DynamicCardProps | StaticCardProps;

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
    <Badge className={cn('absolute top-2 right-2 text-white border-0', colorClass)}>
      {label}
    </Badge>
  );
});
TypeBadge.displayName = 'TypeBadge';

const CardSkeleton = memo(() => (
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
));
CardSkeleton.displayName = 'CardSkeleton';

function MediaCard(props: MediaCardProps) {
  // Static 3D Card
  if (props.variant === 'static') {
    const { title, description, imageUrl, linkUrl } = props;
    return (
      <Link href={linkUrl} scroll={true} passHref>
        <CardContainer className='inter-var cursor-pointer'>
          <CardBody className='bg-gray-50 relative group/card dark:hover:shadow-2xl shadow-blue-500/50 dark:bg-black border-blue-500 w-auto sm:w-[30rem] h-auto rounded-xl p-6 border hover:ring-4 hover:ring-gradient-to-r hover:from-blue-500 hover:to-purple-500'>
            <CardItem
              translateZ='20'
              className='text-xl font-bold text-neutral-600 dark:text-white'
            >
              {title}
            </CardItem>
            <CardItem
              as='p'
              translateZ='20'
              className='text-neutral-500 text-sm max-w-sm mt-2 dark:text-neutral-300'
            >
              {description}
            </CardItem>
            <CardItem translateZ='20' className='w-full mt-4'>
              <Image
                src={imageUrl}
                width={600}
                height={400}
                className='w-full h-60 object-cover rounded-xl'
                alt='Card Thumbnail'
                priority
                placeholder='blur'
                sizes='(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 600px'
              />
            </CardItem>
          </CardBody>
        </CardContainer>
      </Link>
    );
  }

  // Dynamic Card
  const {
    title,
    description,
    imageUrl,
    linkUrl,
    type,
    badge,
    loading,
  } = props as DynamicCardProps;

  const [isImageLoading, setIsImageLoading] = useState(true);
  const [currentImageIndex, setCurrentImageIndex] = useState(0);
  const router = useRouter();
  const fallbackImage = '/default.png';

  const imageSources = useMemo(
    () =>
      [
        imageUrl,
        imageUrl ? `https://imagecdn.app/v1/images/${encodeURIComponent(imageUrl)}` : null,
        imageUrl ? `${PRODUCTION}/api/img-compress2?url=${encodeURIComponent(imageUrl)}` : null,
        imageUrl ? `${PRODUCTION}/api/img-compress3?url=${encodeURIComponent(imageUrl)}` : null,
        imageUrl ? `${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(imageUrl)}` : null,
        fallbackImage,
      ].filter(Boolean) as string[],
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

export default memo(MediaCard);
