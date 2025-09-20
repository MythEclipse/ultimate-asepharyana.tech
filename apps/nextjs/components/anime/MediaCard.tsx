'use client';

import React, { useCallback, memo } from 'react';
import Image from 'next/image';
import { useRouter } from 'next/navigation';
import { cn } from '../../utils/utils';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '../ui/card';
import { Badge } from '../ui/badge';
import { Skeleton } from '../ui/skeleton';
import { useImageFallback } from '../../utils/hooks/useImageFallback';

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
  BD: 'bg-purple-500 hover:bg-purple-500',
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

const CardSkeleton = memo(() => (
  <Card className="w-full max-w-sm overflow-hidden">
    <div className="relative h-64">
      <Skeleton className="h-full w-full rounded-t-md rounded-b-none" />
    </div>
    <CardHeader>
      <Skeleton className="h-5 w-3/4" />
    </CardHeader>
    <CardContent>
      <Skeleton className="h-4 w-full" />
      <Skeleton className="h-4 w-5/6 mt-2" />
    </CardContent>
  </Card>
));
CardSkeleton.displayName = 'CardSkeleton';

function MediaCard(props: MediaCardProps) {
  const router = useRouter();
  const { src, onError } = useImageFallback({ imageUrl: props.imageUrl });

  const handleCardClick = useCallback(() => {
    router.push(props.linkUrl || '/');
  }, [router, props.linkUrl]);

  const { title, description, type, badge, loading } =
    props as DynamicCardProps;

  if (loading) {
    return <CardSkeleton />;
  }

  return (
    <Card
      onClick={handleCardClick}
      className="w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl"
    >
      <div className="relative h-64">
        <Skeleton className="absolute inset-0 h-full w-full rounded-t-md rounded-b-none" />
        <Image
          src={src}
          alt={title || 'Card Image'}
          fill
          sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw"
          className={cn('object-cover transition-opacity duration-300')}
          onError={onError}
          unoptimized
        />
        <TypeBadge type={type} badge={badge} />
      </div>
      <CardHeader>
        <CardTitle className="truncate">{title}</CardTitle>
        {description && (
          <CardDescription className="line-clamp-2">
            {description}
          </CardDescription>
        )}
      </CardHeader>
    </Card>
  );
}

export default memo(MediaCard);
