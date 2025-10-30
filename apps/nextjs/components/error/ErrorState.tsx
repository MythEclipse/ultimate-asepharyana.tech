'use client';

import React from 'react';
import { LucideIcon } from 'lucide-react';

export type ErrorStateType = 'error' | 'info' | 'success' | 'warning';

export interface ErrorStateProps {
  /** Icon component from lucide-react */
  icon: LucideIcon;
  /** Title of the error/info message */
  title: string;
  /** Optional description/message */
  message?: string;
  /** Type of state (error, info, success, warning) */
  type?: ErrorStateType;
  /** Additional CSS classes */
  className?: string;
  /** Whether to use full screen layout */
  fullScreen?: boolean;
}

const colorConfig = {
  error: {
    bg: 'bg-red-100 dark:bg-red-900/30',
    icon: 'text-red-600 dark:text-red-400',
    title: 'text-red-800 dark:text-red-200',
    message: 'text-red-700 dark:text-red-300',
  },
  info: {
    bg: 'bg-blue-100 dark:bg-blue-900/30',
    icon: 'text-blue-600 dark:text-blue-400',
    title: 'text-blue-800 dark:text-blue-200',
    message: 'text-blue-700 dark:text-blue-300',
  },
  success: {
    bg: 'bg-green-100 dark:bg-green-900/30',
    icon: 'text-green-600 dark:text-green-400',
    title: 'text-green-800 dark:text-green-200',
    message: 'text-green-700 dark:text-green-300',
  },
  warning: {
    bg: 'bg-yellow-100 dark:bg-yellow-900/30',
    icon: 'text-yellow-600 dark:text-yellow-400',
    title: 'text-yellow-800 dark:text-yellow-200',
    message: 'text-yellow-700 dark:text-yellow-300',
  },
};

/**
 * ErrorState Component - Reusable error/info/success state display
 * Replaces duplicated error state UI across the application
 */
export function ErrorState({
  icon: Icon,
  title,
  message,
  type = 'error',
  className = '',
  fullScreen = true,
}: ErrorStateProps) {
  const colors = colorConfig[type];

  const content = (
    <div className={`p-6 ${colors.bg} rounded-2xl flex items-center gap-4`}>
      <Icon className={`w-8 h-8 flex-shrink-0 ${colors.icon}`} />
      <div>
        <h1 className={`text-2xl font-bold ${colors.title} mb-2`}>{title}</h1>
        {message && <p className={colors.message}>{message}</p>}
      </div>
    </div>
  );

  if (fullScreen) {
    return (
      <main className={`min-h-screen p-6 bg-background dark:bg-dark ${className}`}>
        <div className="max-w-7xl mx-auto mt-12">{content}</div>
      </main>
    );
  }

  return <div className={className}>{content}</div>;
}

/**
 * ErrorStateCenter - Centered version for loading/error states in specific sections
 */
export function ErrorStateCenter({
  icon: Icon,
  title,
  message,
  type = 'error',
  className = '',
}: ErrorStateProps) {
  const colors = colorConfig[type];

  return (
    <div className={`min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center ${className}`}>
      <div className={`p-6 ${colors.bg} rounded-2xl flex items-center gap-4 max-w-2xl`}>
        <Icon className={`w-8 h-8 flex-shrink-0 ${colors.icon}`} />
        <div>
          <h1 className={`text-2xl font-bold ${colors.title} mb-2`}>{title}</h1>
          {message && <p className={colors.message}>{message}</p>}
        </div>
      </div>
    </div>
  );
}

export default ErrorState;
