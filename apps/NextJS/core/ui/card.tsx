import * as React from 'react';

import { cn } from '@/lib/utils';

/**
 * Props for the Card component.
 * @typedef {Object} CardProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable Card component with accessibility improvements.
 * @param {CardProps & { 'aria-label'?: string }} props - The properties for the Card component.
 * @returns {JSX.Element} The rendered card element.
 */
function Card({ className, 'aria-label': ariaLabel, ...props }: React.ComponentProps<'div'> & { 'aria-label'?: string }) {
  return (
    <div
      data-slot='card'
      className={cn(
        'relative bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm',
        'focus-visible:outline-none',
        className
      )}
      role="region"
      aria-label={ariaLabel}
      {...props}
    />
  );
}

/**
 * Props for the CardHeader component.
 * @typedef {Object} CardHeaderProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable CardHeader component.
 * @param {CardHeaderProps} props - The properties for the CardHeader component.
 * @returns {JSX.Element} The rendered card header element.
 */
function CardHeader({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='card-header'
      className={cn(
        '@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6',
        'focus-visible:outline-none',
        className
      )}
      {...props}
    />
  );
}

/**
 * Props for the CardTitle component.
 * @typedef {Object} CardTitleProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable CardTitle component.
 * @param {CardTitleProps} props - The properties for the CardTitle component.
 * @returns {JSX.Element} The rendered card title element.
 */
function CardTitle({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='card-title'
      className={cn(
        'leading-none font-semibold',
        'focus-visible:outline-none',
        className
      )}
      {...props}
    />
  );
}

/**
 * Props for the CardDescription component.
 * @typedef {Object} CardDescriptionProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable CardDescription component.
 * @param {CardDescriptionProps} props - The properties for the CardDescription component.
 * @returns {JSX.Element} The rendered card description element.
 */
function CardDescription({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='card-description'
      className={cn(
        'text-muted-foreground text-sm',
        'focus-visible:outline-none',
        className
      )}
      {...props}
    />
  );
}

/**
 * Props for the CardAction component.
 * @typedef {Object} CardActionProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable CardAction component.
 * @param {CardActionProps} props - The properties for the CardAction component.
 * @returns {JSX.Element} The rendered card action element.
 */
function CardAction({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='card-action'
      className={cn(
        'col-start-2 row-span-2 row-start-1 self-start justify-self-end',
        'focus-visible:outline-none',
        className
      )}
      {...props}
    />
  );
}

/**
 * Props for the CardContent component.
 * @typedef {Object} CardContentProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable CardContent component.
 * @param {CardContentProps} props - The properties for the CardContent component.
 * @returns {JSX.Element} The rendered card content element.
 */
function CardContent({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='card-content'
      className={cn('px-6', 'focus-visible:outline-none', className)}
      {...props}
    />
  );
}

/**
 * Props for the CardFooter component.
 * @typedef {Object} CardFooterProps
 * @extends React.ComponentProps<'div'>
 */

/**
 * Renders a customizable CardFooter component.
 * @param {CardFooterProps} props - The properties for the CardFooter component.
 * @returns {JSX.Element} The rendered card footer element.
 */
function CardFooter({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='card-footer'
      className={cn(
        'flex items-center px-6 [.border-t]:pt-6',
        'focus-visible:outline-none',
        className
      )}
      {...props}
    />
  );
}

export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardAction,
  CardDescription,
  CardContent,
};
