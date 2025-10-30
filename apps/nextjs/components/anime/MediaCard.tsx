'use client';

import React, { memo } from 'react';
import CardSystem, { CardBadge } from '../ui/CardSystem';
import { MEDIA_TYPE_COLORS } from '../shared/types';

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
      <CardSystem variant="media" loading={true} className="w-full max-w-sm" />
    );
  }

  const badgeConfig: CardBadge | undefined =
    badge || type
      ? {
          text: badge || type || '',
          color: type ? MEDIA_TYPE_COLORS[type] : undefined,
          position: 'top-right',
        }
      : undefined;

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
        show: true,
      }}
      linkUrl={linkUrl}
      className="w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl"
      badge={badgeConfig}
    />
  );
}

export default memo(MediaCard);
