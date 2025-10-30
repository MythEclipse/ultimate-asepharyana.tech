'use client';

import React from 'react';
import { LucideIcon } from 'lucide-react';
import { cn } from './utils';

export type IconBadgeColor = 'purple' | 'blue' | 'green' | 'red' | 'yellow' | 'orange';

export interface IconBadgeProps {
  /** Icon component from lucide-react */
  icon: LucideIcon;
  /** Color theme */
  color?: IconBadgeColor;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Additional CSS classes */
  className?: string;
}

const colorConfig: Record<IconBadgeColor, string> = {
  purple: 'bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400',
  blue: 'bg-blue-100 dark:bg-blue-900/50 text-blue-600 dark:text-blue-400',
  green: 'bg-green-100 dark:bg-green-900/50 text-green-600 dark:text-green-400',
  red: 'bg-red-100 dark:bg-red-900/50 text-red-600 dark:text-red-400',
  yellow: 'bg-yellow-100 dark:bg-yellow-900/50 text-yellow-600 dark:text-yellow-400',
  orange: 'bg-orange-100 dark:bg-orange-900/50 text-orange-600 dark:text-orange-400',
};

const sizeConfig = {
  sm: 'p-2',
  md: 'p-3',
  lg: 'p-4',
};

const iconSizeConfig = {
  sm: 'w-5 h-5',
  md: 'w-8 h-8',
  lg: 'w-10 h-10',
};

/**
 * IconBadge - Reusable icon badge component
 * Used for section headers, page titles, etc.
 * Replaces 25+ duplicated icon badge patterns
 */
export function IconBadge({
  icon: Icon,
  color = 'purple',
  size = 'md',
  className = '',
}: IconBadgeProps) {
  return (
    <div
      className={cn(
        'rounded-xl',
        colorConfig[color],
        sizeConfig[size],
        className
      )}
    >
      <Icon className={iconSizeConfig[size]} />
    </div>
  );
}

export default IconBadge;
