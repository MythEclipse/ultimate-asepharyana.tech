'use client';
import { useEffect, useRef, memo } from 'react';
import { cn } from '@/utils/utils';
import React from 'react';

export const TextGenerateEffect = memo(({
  words,
  className,
  filter = true,
  duration = 0.5,
}: {
  words: string;
  className?: string;
  filter?: boolean;
  duration?: number;
}) => {
  const wordsArray = words.split(' ');
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (containerRef.current) {
      const spans = containerRef.current.querySelectorAll('span');
      spans.forEach((span, idx) => {
        span.style.transition = `opacity ${duration}s ease ${idx * 0.2}s, filter ${duration}s ease ${idx * 0.2}s`;
        span.style.opacity = '1';
        span.style.filter = filter ? 'blur(0px)' : 'none';
      });
    }
  }, [duration, filter]);

  const renderWords = () => {
    return (
      <div ref={containerRef} aria-hidden="true">
        {wordsArray.map((word, idx) => {
          return (
            <span
              key={word + idx}
              className='dark:text-white text-foreground opacity-0'
              style={{
                filter: filter ? 'blur(10px)' : 'none',
              }}
              aria-hidden="true"
            >
              {word}{' '}
            </span>
          );
        })}
      </div>
    );
  };

  return (
    <div className={cn('font-bold', className)}>
      <div className='mt-4'>
        <div className='dark:text-white text-foreground text-2xl leading-snug tracking-wide'>
          {/* Visually hidden text for screen readers */}
          <span className="sr-only">{words}</span>
          {renderWords()}
        </div>
      </div>
    </div>
  );
});

TextGenerateEffect.displayName = 'TextGenerateEffect';
