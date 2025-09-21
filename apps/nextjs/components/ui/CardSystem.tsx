'use client';

import React, { memo, useCallback } from 'react';
import { StaticImageData } from 'next/image';
import { cn } from '../../utils/utils';
import { CardImage } from '../shared/CardImage';
import { CardLink } from '../shared/CardLink';
import { CardSkeleton } from '../shared/CardSkeleton';
import { CardBody, CardContainer, CardItem } from './3d-card';
import { Card as ShadcnCard } from './ComponentCard';

export type CardVariant = 'base' | '3d' | 'mini-3d' | 'interactive' | 'themed' | 'media';

export interface CardBadge {
  text: string;
  color?: string;
  position?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';
}

export interface CardImageConfig {
  src: string | StaticImageData;
  alt?: string;
  width?: number;
  height?: number;
  className?: string;
  priority?: boolean;
  unoptimized?: boolean;
  useProxy?: boolean;
  useCdn?: boolean;
  fallbackUrl?: string;
  fill?: boolean;
  sizes?: string;
  placeholder?: 'blur' | 'empty';
  show?: boolean;
}

export interface CardSystemProps {
  variant?: CardVariant;
  title?: string;
  description?: string;
  image?: CardImageConfig;
  linkUrl?: string;
  onClick?: () => void;
  className?: string;
  titleClassName?: string;
  descriptionClassName?: string;
  badge?: CardBadge;
  children?: React.ReactNode;
  loading?: boolean;
  skeleton?: boolean | Omit<React.ComponentProps<typeof CardSkeleton>, 'variant'>;
  containerClassName?: string;
  bodyClassName?: string;
  [key: string]: any;
}

const CardSystem = memo(({
  variant = 'base',
  title,
  description,
  image,
  linkUrl,
  onClick,
  className = '',
  titleClassName = '',
  descriptionClassName = '',
  badge,
  children,
  loading = false,
  skeleton = false,
  containerClassName = '',
  bodyClassName = '',
  ...props
}: CardSystemProps) => {
  const handleClick = useCallback(() => {
    if (onClick) {
      onClick();
    }
  }, [onClick]);

  // Handle loading state
  if (loading || skeleton) {
    const skeletonProps = typeof skeleton === 'object' ? skeleton : {};
    const skeletonVariant = variant === '3d' || variant === 'mini-3d' ? '3d' : 'default';
    return <CardSkeleton variant={skeletonVariant} className={className} {...skeletonProps} />;
  }

  // Handle 3D variants
  if (variant === '3d' || variant === 'mini-3d') {
    const isMini = variant === 'mini-3d';

    return (
      <CardLink href={linkUrl} onClick={onClick} className={containerClassName}>
        <CardContainer className="inter-var cursor-pointer">
          <CardBody className={cn(
            isMini
              ? 'bg-gray-50 relative group/card dark:hover:shadow-2xl dark:hover:shadow-emerald-500/[0.1] dark:bg-black dark:border-white/[0.2] border-black/[0.1] w-[15rem] h-[25rem] rounded-xl p-4 border overflow-hidden'
              : 'relative group/card dark:hover:shadow-2xl shadow-blue-500/50 border-blue-500 w-auto sm:w-[30rem] h-auto rounded-xl p-6 border hover:ring-4 hover:ring-gradient-to-r hover:from-blue-500 hover:to-purple-500',
            bodyClassName
          )}>
            {title && (
              <CardItem
                translateZ={isMini ? '50' : '20'}
                className={cn(
                  isMini ? 'text-lg font-semibold' : 'text-xl font-bold',
                  'text-neutral-600 dark:text-white truncate',
                  titleClassName
                )}
              >
                {title}
              </CardItem>
            )}
            {description && (
              <CardItem
                as="p"
                translateZ={isMini ? '60' : '20'}
                className={cn(
                  isMini
                    ? 'text-neutral-500 text-xs mt-2 overflow-hidden overflow-ellipsis whitespace-nowrap dark:text-neutral-300'
                    : 'text-neutral-500 text-sm max-w-sm mt-2 dark:text-neutral-300',
                  descriptionClassName
                )}
              >
                {description}
              </CardItem>
            )}
            {image?.show !== false && image?.src && (
              <CardItem translateZ={isMini ? '100' : '20'} className={cn('w-full mt-4', isMini && 'h-[60%]')}>
                <CardImage
                  src={image.src}
                  alt={image.alt || title || 'Card image'}
                  width={isMini ? 300 : 600}
                  height={isMini ? 400 : 400}
                  className={cn(
                    'w-full object-cover rounded-xl',
                    isMini && 'h-full group-hover/card:shadow-xl',
                    image.className
                  )}
                  priority={image.priority}
                  unoptimized={image.unoptimized ?? true}
                  useProxy={image.useProxy}
                  useCdn={image.useCdn}
                  fallbackUrl={image.fallbackUrl}
                />
              </CardItem>
            )}
            {children}
          </CardBody>
        </CardContainer>
      </CardLink>
    );
  }

  // Handle interactive variant
  if (variant === 'interactive') {
    return (
      <ShadcnCard className={cn(
        'w-full h-full dark:bg-black overflow-hidden text-blue-500 bg-transparent border rounded-lg shadow-lg shadow-blue-500/50 hover:bg-blue-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50',
        className
      )} {...props}>
        {children}
      </ShadcnCard>
    );
  }

  // Handle themed variant
  if (variant === 'themed') {
    return (
      <div className={cn(
        'p-6 shadow-xl border border-blue-500 dark:border-blue-500 rounded-lg focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-500',
        className
      )} {...props}>
        {children}
      </div>
    );
  }

  // Handle media variant (wrapper with badge support)
  if (variant === 'media') {
    const typeColors: { [key: string]: string } = {
      Manga: 'bg-red-500 hover:bg-red-600',
      Manhua: 'bg-green-500 hover:bg-green-600',
      Manhwa: 'bg-blue-500 hover:bg-blue-600',
      BD: 'bg-purple-500 hover:bg-purple-500',
      TV: 'bg-yellow-500 hover:bg-yellow-600',
      OVA: 'bg-pink-500 hover:bg-pink-600',
      ONA: 'bg-indigo-500 hover:bg-indigo-600',
    };

    const badgeColor = badge?.color || typeColors[badge?.text || ''] || 'bg-gray-500 hover:bg-gray-600';
    const badgePosition = badge?.position || 'top-right';

    return (
      <div className="relative">
        <CardSystem
          variant="base"
          title={title}
          description={description}
          image={image}
          linkUrl={linkUrl}
          onClick={onClick}
          className={className}
          titleClassName={titleClassName}
          descriptionClassName={descriptionClassName}
          containerClassName={containerClassName}
          bodyClassName={bodyClassName}
          {...props}
        >
          {children}
        </CardSystem>
        {badge && (
          <div className={cn(
            'absolute text-white text-xs px-2 py-1 rounded border-0',
            badgePosition === 'top-right' && 'top-2 right-2',
            badgePosition === 'top-left' && 'top-2 left-2',
            badgePosition === 'bottom-right' && 'bottom-2 right-2',
            badgePosition === 'bottom-left' && 'bottom-2 left-2',
            badgeColor
          )}>
            {badge.text}
          </div>
        )}
      </div>
    );
  }

  // Default base variant
  const cardContent = (
    <div
      className={cn(
        'w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl',
        className
      )}
      onClick={handleClick}
      {...props}
    >
      {image?.show !== false && image?.src && (
        <div className="relative h-64">
          <CardImage
            src={image.src}
            alt={image.alt || title || 'Card image'}
            fill
            sizes={image.sizes || '(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw'}
            className={cn('object-cover transition-opacity duration-300', image.className)}
            priority={image.priority}
            unoptimized={image.unoptimized}
            useProxy={image.useProxy}
            useCdn={image.useCdn}
            fallbackUrl={image.fallbackUrl}
            placeholder={image.placeholder}
          />
        </div>
      )}
      <div className="p-4">
        {title && (
          <h3 className={cn('text-lg font-semibold truncate', titleClassName)}>
            {title}
          </h3>
        )}
        {description && (
          <p className={cn('text-sm text-gray-600 line-clamp-2 mt-1 dark:text-gray-400', descriptionClassName)}>
            {description}
          </p>
        )}
        {children}
      </div>
    </div>
  );

  if (linkUrl && !onClick) {
    return (
      <CardLink href={linkUrl} className={containerClassName}>
        {cardContent}
      </CardLink>
    );
  }

  return cardContent;
});

CardSystem.displayName = 'CardSystem';

// Export individual components for advanced usage
export { CardImage, CardLink, CardSkeleton };

// Export 3D card components for direct usage
export { CardBody, CardContainer, CardItem } from './3d-card';

// Export convenience components
export const BaseCard = (props: Omit<CardSystemProps, 'variant'>) => (
  <CardSystem variant="base" {...props} />
);

export const TildCard = (props: Omit<CardSystemProps, 'variant'>) => (
  <CardSystem variant="3d" {...props} />
);

export const MiniTildCard = (props: Omit<CardSystemProps, 'variant'>) => (
  <CardSystem variant="mini-3d" {...props} />
);

export const InteractiveCard = (props: Omit<CardSystemProps, 'variant'>) => (
  <CardSystem variant="interactive" {...props} />
);

export const ThemedCard = (props: Omit<CardSystemProps, 'variant'>) => (
  <CardSystem variant="themed" {...props} />
);

export const MediaCard = (props: Omit<CardSystemProps, 'variant'>) => (
  <CardSystem variant="media" {...props} />
);

export default CardSystem;
