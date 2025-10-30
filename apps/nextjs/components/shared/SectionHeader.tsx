'use client';

import React from 'react';
import { LucideIcon } from 'lucide-react';
import { IconBadge, IconBadgeColor } from './IconBadge';
import { cn } from './utils';

export interface SectionHeaderProps {
  /** Icon component from lucide-react */
  icon: LucideIcon;
  /** Section title */
  title: string;
  /** Optional subtitle/description */
  subtitle?: string;
  /** Icon badge color */
  color?: IconBadgeColor;
  /** Title gradient colors (Tailwind classes) */
  gradientFrom?: string;
  gradientTo?: string;
  /** Additional CSS classes for container */
  className?: string;
  /** Additional content (e.g., action buttons) */
  action?: React.ReactNode;
}

/**
 * SectionHeader - Reusable section header component
 * Consistent header with icon badge, title, and optional subtitle
 * Used across anime, komik, and other pages
 */
export function SectionHeader({
  icon,
  title,
  subtitle,
  color = 'purple',
  gradientFrom = 'from-purple-600',
  gradientTo = 'to-pink-600',
  className = '',
  action,
}: SectionHeaderProps) {
  return (
    <div
      className={cn(
        'flex flex-col md:flex-row justify-between items-start md:items-center gap-4',
        className
      )}
    >
      <div className="flex items-center gap-4">
        <IconBadge icon={icon} color={color} />
        <div>
          <h1
            className={cn(
              'text-3xl font-bold bg-gradient-to-r bg-clip-text text-transparent',
              gradientFrom,
              gradientTo
            )}
          >
            {title}
          </h1>
          {subtitle && (
            <p className="text-zinc-600 dark:text-zinc-400 mt-1">{subtitle}</p>
          )}
        </div>
      </div>
      {action && <div>{action}</div>}
    </div>
  );
}

export default SectionHeader;
