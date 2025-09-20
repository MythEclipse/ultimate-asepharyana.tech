// apps/NextJS/core/ui/BaseButton.tsx
'use client';

import { FC, ReactNode } from 'react';
import Link from 'next/link';

interface BaseButtonProps {
  children: ReactNode;
  className?: string;
  onClick?: () => void;
  disabled?: boolean;
  href?: string;
  type?: 'button' | 'submit' | 'reset';
  size?: 'sm' | 'md';
}

const BaseButton: FC<BaseButtonProps> = ({
  children,
  disabled,
  onClick,
  type = 'button',
  className,
  href,
  size = 'sm',
}) => {
  const sizeClasses =
    size === 'md'
      ? 'px-6 py-3 text-sm md:px-6 md:py-3 md:text-base'
      : 'px-3 py-1 text-sm md:px-6 md:py-3 md:text-base';

  const buttonContent = (
    <div
      className={`flex flex-col items-center justify-center text-center ${sizeClasses} text-blue-500 bg-transparent border border-blue-500 rounded-full shadow-lg shadow-blue-500/50 hover:bg-gradient-to-r hover:from-blue-500 hover:to-purple-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50 transition-colors duration-200 ease-in-out ${className}`}
    >
      {children}
    </div>
  );

  if (href) {
    return <Link href={href}>{buttonContent}</Link>;
  }

  return (
    <button
      className={`flex flex-col items-center justify-center text-center ${sizeClasses} text-blue-500 bg-transparent border border-blue-500 rounded-full shadow-lg shadow-blue-500/50 hover:bg-gradient-to-r hover:from-blue-500 hover:to-purple-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50 transition-colors duration-200 ease-in-out ${className}`}
      disabled={disabled}
      type={type}
      onClick={onClick}
    >
      {children}
    </button>
  );
};

export default BaseButton;
