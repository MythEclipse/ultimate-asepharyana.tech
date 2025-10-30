'use client';

import React, { memo } from 'react';
import { cn } from '../../utils/utils';

export interface CardSkeletonProps {
  variant?: 'default' | 'compact' | 'mini' | '3d';
  className?: string;
  showImage?: boolean;
  showTitle?: boolean;
  showDescription?: boolean;
  lines?: number;
}

export const CardSkeleton = memo(
  ({
    variant = 'default',
    className,
    showImage = true,
    showTitle = true,
    showDescription = true,
    lines = 2,
  }: CardSkeletonProps) => {
    const getSkeletonVariant = () => {
      switch (variant) {
        case 'compact':
          return 'w-full max-w-xs';
        case 'mini':
          return 'w-60 h-96';
        case '3d':
          return 'w-80 h-96';
        default:
          return 'w-full max-w-sm';
      }
    };

    const getImageHeight = () => {
      switch (variant) {
        case 'compact':
          return 'h-48';
        case 'mini':
          return 'h-40';
        case '3d':
          return 'h-60';
        default:
          return 'h-64';
      }
    };

    return (
      <div
        className={cn(
          'animate-pulse overflow-hidden rounded-lg',
          getSkeletonVariant(),
          className,
        )}
      >
        {showImage && (
          <div
            className={cn('bg-gray-200 dark:bg-gray-700', getImageHeight())}
          />
        )}
        <div className="p-4 space-y-2">
          {showTitle && (
            <div className="h-5 bg-gray-200 dark:bg-gray-700 rounded w-3/4" />
          )}
          {showDescription && (
            <>
              {Array.from({ length: lines }, (_, i) => (
                <div
                  key={i}
                  className={cn(
                    'h-4 bg-gray-200 dark:bg-gray-700 rounded',
                    i === lines - 1 ? 'w-5/6' : 'w-full',
                  )}
                />
              ))}
            </>
          )}
        </div>
      </div>
    );
  },
);

CardSkeleton.displayName = 'CardSkeleton';

// Specialized skeleton variants
export const CompactCardSkeleton = (
  props: Omit<CardSkeletonProps, 'variant'>,
) => <CardSkeleton variant="compact" {...props} />;

export const MiniCardSkeleton = (props: Omit<CardSkeletonProps, 'variant'>) => (
  <CardSkeleton variant="mini" {...props} />
);

export const Card3DSkeleton = (props: Omit<CardSkeletonProps, 'variant'>) => (
  <CardSkeleton variant="3d" {...props} />
);
