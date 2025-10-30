'use client';

import React from 'react';
import Link from 'next/link';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { cn } from '../shared/utils';

export interface PaginationInfo {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page?: number | null;
  has_previous_page: boolean;
  previous_page?: number | null;
}

export interface PaginationControlsProps {
  pagination: PaginationInfo;
  /** Base URL for pagination links (e.g., '/anime/complete-anime') */
  baseUrl?: string;
  /** Callback for page changes (alternative to baseUrl) */
  onPageChange?: (page: number) => void;
  /** Additional CSS classes */
  className?: string;
  /** Style variant */
  variant?: 'default' | 'compact';
}

/**
 * Reusable pagination controls component
 * Supports both link-based and callback-based navigation
 */
export function PaginationControls({
  pagination,
  baseUrl,
  onPageChange,
  className = '',
  variant = 'default',
}: PaginationControlsProps) {
  const {
    current_page,
    last_visible_page,
    has_next_page,
    has_previous_page,
    next_page,
    previous_page,
  } = pagination;

  const buttonClass = cn(
    'px-6 py-2 rounded-lg transition-colors flex items-center gap-2',
    'bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300',
    'hover:bg-zinc-200 dark:hover:bg-zinc-700',
    variant === 'compact' && 'px-4 py-1.5 text-sm',
  );

  const renderButton = (
    direction: 'prev' | 'next',
    enabled: boolean,
    pageNumber?: number | null,
  ) => {
    if (!enabled || !pageNumber) return null;

    const content = (
      <>
        {direction === 'prev' && (
          <ChevronLeft
            className={variant === 'compact' ? 'w-4 h-4' : 'w-5 h-5'}
          />
        )}
        {direction === 'prev' ? 'Previous' : 'Next'}
        {direction === 'next' && (
          <ChevronRight
            className={variant === 'compact' ? 'w-4 h-4' : 'w-5 h-5'}
          />
        )}
      </>
    );

    if (baseUrl) {
      return (
        <Link href={`${baseUrl}/${pageNumber}`} className={buttonClass}>
          {content}
        </Link>
      );
    }

    if (onPageChange) {
      return (
        <button
          onClick={() => onPageChange(pageNumber)}
          className={buttonClass}
        >
          {content}
        </button>
      );
    }

    return null;
  };

  return (
    <div
      className={cn(
        'flex flex-wrap gap-4 justify-between items-center',
        className,
      )}
    >
      <div className="flex gap-4">
        {renderButton(
          'prev',
          has_previous_page,
          previous_page || current_page - 1,
        )}
        {renderButton('next', has_next_page, next_page || current_page + 1)}
      </div>

      <span
        className={cn(
          'font-medium text-zinc-600 dark:text-zinc-400 mx-4',
          variant === 'compact' ? 'text-xs' : 'text-sm',
        )}
      >
        Page {current_page} of {last_visible_page}
      </span>
    </div>
  );
}

export default PaginationControls;
