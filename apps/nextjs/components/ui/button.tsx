import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';
import Link from 'next/link';
import { BackgroundGradient } from '../background/background-gradient';

import { cn } from '../../utils/utils';

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default:
          'bg-primary text-primary-foreground shadow-xs hover:bg-primary/90',
        destructive:
          'bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60',
        outline:
          'border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50',
        secondary:
          'bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80',
        ghost:
          'hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50',
        link: 'text-primary underline-offset-4 hover:underline',
        gradient:
          'text-blue-500 bg-transparent border border-blue-500 rounded-full shadow-lg shadow-blue-500/50 hover:bg-gradient-to-r hover:from-blue-500 hover:to-purple-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50 transition-colors duration-200 ease-in-out',
        chapter: 'relative flex flex-col items-center justify-center p-4 border-none cursor-pointer w-full h-full bg-transparent',
      },
      size: {
        default: 'h-9 px-4 py-2 has-[>svg]:px-3',
        sm: 'h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5',
        lg: 'h-10 rounded-md px-6 has-[>svg]:px-4',
        icon: 'size-9',
        gradientSm: 'px-3 py-1 text-sm md:px-6 md:py-3 md:text-base',
        gradientMd: 'px-6 py-3 text-sm md:px-6 md:py-3 md:text-base',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
);

/**
 * Renders a customizable Button component with accessibility improvements.
 * Supports aria-label and aria-pressed for toggle buttons.
 */
function Button({
  className,
  variant,
  size,
  asChild = false,
  'aria-label': ariaLabel,
  'aria-pressed': ariaPressed,
  href,
  scrollToTop = false,
  ...props
}: React.ComponentProps<'button'> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
    'aria-label'?: string;
    'aria-pressed'?: boolean;
    href?: string;
    scrollToTop?: boolean;
  }) {
  const Comp = asChild ? Slot : 'button';

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    if (props.onClick) {
      props.onClick(event);
    }
    if (scrollToTop) {
      window.scrollTo(0, 0);
    }
  };

  const buttonElement = (
    <Comp
      data-slot="button"
      className={cn(
        buttonVariants({ variant, size, className }),
        'focus-visible:outline-none',
      )}
      aria-label={ariaLabel}
      aria-pressed={ariaPressed}
      onClick={handleClick}
      {...props}
    />
  );

  if (href) {
    return <Link href={href}>{buttonElement}</Link>;
  }

  if (variant === 'chapter') {
    return (
      <div className="relative max-w-xs transform transition-transform duration-200 hover:scale-105 active:scale-95">
        <BackgroundGradient className="rounded-[22px] overflow-hidden w-full h-full bg-white dark:bg-zinc-900">
          {buttonElement}
        </BackgroundGradient>
      </div>
    );
  }

  return buttonElement;
}

export { Button, buttonVariants };
