'use client';

import React, { memo } from 'react';
import { cn } from '../../utils/utils';
import CardSystem, { CardBadge } from '../ui/CardSystem';

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

function MediaCard(props: MediaCardProps) {
  const { title, description, imageUrl, linkUrl, type, badge, loading } =
    props as DynamicCardProps;

  if (loading) {
    return (
      <CardSystem
        variant="media"
        loading={true}
        className="w-full max-w-sm"
      />
    );
  }

  const badgeConfig: CardBadge | undefined = badge || type ? {
    text: badge || type || '',
    color: type ? typeColors[type] : undefined,
    position: 'top-right'
  } : undefined;

  return (
    <CardSystem
      variant="media"
      title={title || ''}
      description={description || ''}
      image={{
        src: imageUrl || '',
        alt: title || 'Card image',
        priority: false,
        unoptimized: true,
        show: true
      }}
      linkUrl={linkUrl}
      className="w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl"
      badge={badgeConfig}
    />
  );
}

export default memo(MediaCard);
