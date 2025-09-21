// Shared utility functions for components
import React from 'react';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * Combines class names with tailwind-merge for conflict resolution
 * This is a more robust version of the cn utility
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Generates skeleton variants based on component type
 */
export function getSkeletonVariant(variant: 'default' | 'compact' | 'mini' | '3d'): {
  container: string;
  imageHeight: string;
} {
  switch (variant) {
    case 'compact':
      return {
        container: 'w-full max-w-xs',
        imageHeight: 'h-48',
      };
    case 'mini':
      return {
        container: 'w-60 h-96',
        imageHeight: 'h-40',
      };
    case '3d':
      return {
        container: 'w-80 h-96',
        imageHeight: 'h-60',
      };
    default:
      return {
        container: 'w-full max-w-sm',
        imageHeight: 'h-64',
      };
  }
}

/**
 * Determines if a link should open in a new tab
 */
export function shouldOpenInNewTab(href?: string, target?: string): boolean {
  return target === '_blank' || (href !== undefined && href.startsWith('http'));
}

/**
 * Generates appropriate rel attribute for external links
 */
export function getRelAttribute(target?: string, rel?: string): string {
  if (target === '_blank') {
    return rel || 'noopener noreferrer';
  }
  return rel || '';
}

/**
 * Creates a media badge configuration
 */
export function createMediaBadge(text?: string, type?: string, position: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' = 'top-right') {
  if (!text && !type) return undefined;

  return {
    text: text || type || '',
    color: type ? MEDIA_TYPE_COLORS[type] : undefined,
    position,
  };
}

/**
 * Common animation classes for hover effects
 */
export const HOVER_ANIMATION_CLASSES = {
  scale: 'transition-transform duration-300 hover:scale-105',
  shadow: 'transition-shadow duration-300 hover:shadow-xl',
  combined: 'transition-all duration-300 hover:scale-105 hover:shadow-xl',
};

/**
 * Common image placeholder classes
 */
export const IMAGE_PLACEHOLDER_CLASSES = {
  light: 'bg-gray-200',
  dark: 'dark:bg-gray-700',
  combined: 'bg-gray-200 dark:bg-gray-700',
};

// Import media type colors from shared types
import { MEDIA_TYPE_COLORS } from '../shared/types';

/**
 * Common card styling classes
 */
export const CARD_STYLES = {
  base: 'w-full max-w-sm overflow-hidden cursor-pointer transition-transform duration-300 hover:scale-105 hover:shadow-xl',
  imageContainer: 'relative h-64',
  image: 'object-cover transition-opacity duration-300',
  title: 'text-lg font-semibold truncate',
  description: 'text-sm text-gray-600 line-clamp-2 mt-1 dark:text-gray-400',
  content: 'p-4',
};

/**
 * Generates image sizes attribute for responsive images
 */
export function getImageSizes(breakpoints?: {
  sm?: string;
  md?: string;
  lg?: string;
  default?: string;
}): string {
  const defaults = {
    sm: '(max-width: 640px) 100vw',
    md: '(max-width: 1024px) 50vw',
    lg: '33vw',
    default: '33vw',
  };

  const { sm, md, lg, default: defaultSize } = { ...defaults, ...breakpoints };
  return `${sm}, ${md}, ${lg || defaultSize}`;
}

/**
 * Common loading states
 */
export const LOADING_STATES = {
  spinner: 'animate-spin',
  pulse: 'animate-pulse',
  skeleton: 'animate-pulse overflow-hidden rounded-lg',
} as const;

/**
 * Type guard to check if value is a valid React node
 */
export function isValidReactNode(value: unknown): value is React.ReactNode {
  return (
    value === null ||
    value === undefined ||
    typeof value === 'string' ||
    typeof value === 'number' ||
    React.isValidElement(value) ||
    (Array.isArray(value) && value.every(isValidReactNode))
  );
}

/**
 * Safe JSON parse with fallback
 */
export function safeJsonParse<T>(str: string, fallback: T): T {
  try {
    return JSON.parse(str) as T;
  } catch {
    return fallback;
  }
}

/**
 * Debounce function for performance optimization
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}

/**
 * Throttle function for performance optimization
 */
export function throttle<T extends (...args: unknown[]) => unknown>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle = false;
  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args);
      inThrottle = true;
      setTimeout(() => (inThrottle = false), limit);
    }
  };
}
