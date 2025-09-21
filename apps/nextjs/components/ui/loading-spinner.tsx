import { Loader2 } from 'lucide-react';
import { cn } from '../../utils/utils';

interface LoadingSpinnerProps {
  className?: string;
  size?: 'sm' | 'md' | 'lg';
}

export function LoadingSpinner({ className, size = 'md' }: LoadingSpinnerProps) {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-5 h-5',
    lg: 'w-6 h-6',
  };

  return (
    <Loader2
      className={cn('animate-spin', sizeClasses[size], className)}
      aria-hidden="true"
    />
  );
}
