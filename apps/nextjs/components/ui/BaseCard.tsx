'use client';

import React, { memo, useState, useCallback } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { cn } from '../../utils/utils';
import { PRODUCTION } from '../../utils/url-utils';

export interface BaseCardProps {
  title: string;
  description?: string;
  imageUrl: string;
  linkUrl?: string;
  onClick?: () => void;
  className?: string;
  imageClassName?: string;
  titleClassName?: string;
  descriptionClassName?: string;
  showImage?: boolean;
  imageHeight?: number;
  imageWidth?: number;
  imagePriority?: boolean;
  imageUnoptimized?: boolean;
  imageFallback?: boolean;
  children?: React.ReactNode;
  loading?: boolean;
  [key: string]: any;
}

const BaseCard = memo(({
  title,
  description,
  imageUrl,
  linkUrl,
  onClick,
  className = '',
  imageClassName = '',
  titleClassName = '',
  descriptionClassName = '',
  showImage = true,
  imageHeight = 400,
  imageWidth = 300,
  imagePriority = false,
  imageUnoptimized = false,
  imageFallback = false,
  children,
  loading = false,
  ...props
}: BaseCardProps) => {
  const router = useRouter();
  const [currentImageSrc, setCurrentImageSrc] = useState(imageUrl);
  const [hasError, setHasError] = useState(false);

  const handleImageError = useCallback(() => {
    if (imageFallback && !hasError) {
      setCurrentImageSrc(`${PRODUCTION}/api/imageproxy?url=${encodeURIComponent(imageUrl)}`);
      setHasError(true);
    }
  }, [imageFallback, hasError, imageUrl]);

  const handleClick = useCallback(() => {
    if (onClick) {
      onClick();
    } else if (linkUrl) {
      router.push(linkUrl);
    }
  }, [onClick, linkUrl, router]);

  if (loading) {
    return (
      <div className={cn('w-full max-w-sm overflow-hidden animate-pulse', className)} {...props}>
        <div className="relative h-64 bg-gray-200 rounded-t-md"></div>
        <div className="p-4 space-y-2">
          <div className="h-5 bg-gray-200 rounded w-3/4"></div>
          <div className="h-4 bg-gray-200 rounded w-full"></div>
          <div className="h-4 bg-gray-200 rounded w-5/6"></div>
        </div>
      </div>
    );
  }

  const cardContent = (
    <div
      className={cn('w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl', className)}
      onClick={handleClick}
      {...props}
    >
      {showImage && (
        <div className="relative h-64">
          <Image
            src={currentImageSrc}
            alt={title || 'Card Image'}
            fill
            sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw"
            className={cn('object-cover transition-opacity duration-300', imageClassName)}
            onError={handleImageError}
            priority={imagePriority}
            unoptimized={imageUnoptimized}
          />
        </div>
      )}
      <div className="p-4">
        <h3 className={cn('text-lg font-semibold truncate', titleClassName)}>
          {title}
        </h3>
        {description && (
          <p className={cn('text-sm text-gray-600 line-clamp-2 mt-1', descriptionClassName)}>
            {description}
          </p>
        )}
        {children}
      </div>
    </div>
  );

  if (linkUrl && !onClick) {
    return (
      <Link href={linkUrl} prefetch={true} scroll={true} passHref>
        {cardContent}
      </Link>
    );
  }

  return cardContent;
});

BaseCard.displayName = 'BaseCard';

export default BaseCard;
