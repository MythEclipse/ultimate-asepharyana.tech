import * as React from 'react';
import { memo } from 'react';

import { cn } from '@/utils/utils';

export type TextareaProps = React.TextareaHTMLAttributes<HTMLTextAreaElement>;

/**
 * Accessible Textarea component supporting aria-describedby and label association.
 */
const Textarea = memo(React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, 'aria-describedby': ariaDescribedBy, ...props }, ref) => {
    return (
      <textarea
        className={cn(
          'flex min-h-[80px] hover:border-white w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
          className
        )}
        ref={ref}
        aria-describedby={ariaDescribedBy}
        {...props}
      />
    );
  }
));
Textarea.displayName = 'Textarea';

export { Textarea };
