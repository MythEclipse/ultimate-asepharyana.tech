'use client';

import React, { memo } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export interface CardLinkProps {
  href?: string;
  children: React.ReactNode;
  prefetch?: boolean;
  scroll?: boolean;
  onClick?: () => void;
  className?: string;
  target?: string;
  rel?: string;
}

export const CardLink = memo(({
  href,
  children,
  prefetch = true,
  scroll = true,
  onClick,
  className,
  target,
  rel,
}: CardLinkProps) => {
  const router = useRouter();

  const handleClick = (e: React.MouseEvent) => {
    if (onClick) {
      e.preventDefault();
      onClick();
    }
  };

  if (!href && !onClick) {
    return <>{children}</>;
  }

  if (onClick && !href) {
    return (
      <div className={className} onClick={handleClick} style={{ cursor: 'pointer' }}>
        {children}
      </div>
    );
  }

  if (target === '_blank' || (href && href.startsWith('http'))) {
    return (
      <a
        href={href}
        target={target}
        rel={rel || 'noopener noreferrer'}
        className={className}
        onClick={handleClick}
      >
        {children}
      </a>
    );
  }

  return (
    <Link
      href={href!}
      prefetch={prefetch}
      scroll={scroll}
      className={className}
      onClick={handleClick}
    >
      {children}
    </Link>
  );
});

CardLink.displayName = 'CardLink';
