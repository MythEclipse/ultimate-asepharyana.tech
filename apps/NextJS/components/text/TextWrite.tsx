'use client';
import React, { memo } from 'react';
import { cn } from '@/lib/utils';
import { useEffect, useRef, useState } from 'react';

export const AnimatedHeader = memo(({
  words,
  className,
  cursorClassName,
}: {
  words: {
    text: string;
    className?: string;
  }[];
  className?: string;
  cursorClassName?: string;
}) => {
  const wordsArray = words.map((word) => ({
    ...word,
    text: word.text.split(''),
  }));

  const [isInView, setIsInView] = useState(false);
  const headerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => setIsInView(entry.isIntersecting),
      { threshold: 0.1 }
    );

    const currentHeaderRef = headerRef.current;
    if (currentHeaderRef) observer.observe(currentHeaderRef);

    return () => {
      if (currentHeaderRef) observer.unobserve(currentHeaderRef);
    };
  }, []);

  const renderWords = () => {
    let globalIndex = 0;
    return (
      <div ref={headerRef} className='inline' aria-hidden="true">
        {wordsArray.map((word, wordIdx) => (
          <div key={`word-${wordIdx}`} className='inline-block'>
            {word.text.map((char, charIdx) => {
              const charDelay = globalIndex * 100;
              globalIndex++;
              return (
                <span
                  key={`char-${charIdx}`}
                  className={cn(
                    `dark:text-white text-foreground transition-opacity duration-300 ease-in-out`,
                    isInView
                      ? 'opacity-100 translate-y-0'
                      : 'opacity-0 translate-y-2',
                    word.className
                  )}
                  style={{ transitionDelay: `${charDelay}ms` }}
                  aria-hidden="true"
                >
                  {char}
                </span>
              );
            })}
            &nbsp;
          </div>
        ))}
      </div>
    );
  };

  // Compose the full text for screen readers
  const fullText = words.map(w => w.text).join(' ');

  return (
    <header className={cn('py-8', className)}>
      <h1 className='text-4xl sm:text-5xl md:text-6xl font-bold tracking-tight'>
        <span className="sr-only">{fullText}</span>
        {renderWords()}
        <span
          className={cn(
            'inline-block rounded-sm w-[3px] h-7 bg-blue-500 animate-blink ml-1',
            cursorClassName
          )}
          aria-hidden="true"
        ></span>
      </h1>
    </header>
  );
});

AnimatedHeader.displayName = 'AnimatedHeader';
