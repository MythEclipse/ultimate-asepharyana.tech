'use client';

import { ReactNode } from 'react';
import React from 'react';
interface CardProps {
  children: ReactNode;
}

export default function CardB({ children }: CardProps) {
  return (
    <div className='p-6 shadow-xl   border border-blue-500 dark:border-blue-500 rounded-lg focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-500'>
      {children}
    </div>
  );
}
