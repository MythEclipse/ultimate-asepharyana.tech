'use client';

import React, { memo } from 'react';
import { cn } from '../../utils/utils';
import { Badge } from '../ui/badge';
import BaseCard from '../ui/BaseCard';

const typeColors: { [key: string]: string } = {
  Manga: 'bg-red-500 hover:bg-red-600',
  Manhua: 'bg-green-500 hover:bg-green-600',
  Manhwa: 'bg-blue-500 hover:bg-blue-600',
  BD: 'bg-purple-500 hover:bg-purple-500',
  TV: 'bg-yellow-500 hover:bg-yellow-600',
  OVA: 'bg-pink-500 hover:bg-pink-600',
  ONA: 'bg-indigo-500 hover:bg-indigo-600',
};

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

function MediaCard(props: MediaCardProps) {
  const { title, description, imageUrl, linkUrl, type, badge, loading } =
    props as DynamicCardProps;

  if (loading) {
    return (
      <BaseCard
        title="Loading..."
        description=""
        imageUrl=""
        loading={true}
        className="w-full max-w-sm"
      />
    );
  }

  return (
    <div className="relative">
      <BaseCard
        title={title || ''}
        description={description || ''}
        imageUrl={imageUrl || ''}
        linkUrl={linkUrl}
        className="w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl"
        imageClassName="object-cover transition-opacity duration-300"
        imagePriority={false}
        imageUnoptimized={true}
        showImage={true}
      />
      <TypeBadge type={type} badge={badge} />
    </div>
  );
}

export default memo(MediaCard);
