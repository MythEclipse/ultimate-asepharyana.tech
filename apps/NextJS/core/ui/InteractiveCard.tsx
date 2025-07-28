'use client';

import { Card as ShadcnCard } from '@core/ui/ComponentCard';
import { ReactNode } from 'react';
import React, { memo } from 'react';
interface CardProps {
  children: ReactNode;
}

function CardB({ children }: CardProps) {
  return (
    <ShadcnCard className='w-full h-full dark:bg-black overflow-hidden text-blue-500 bg-transparent border rounded-lg shadow-lg shadow-blue-500/50 hover:bg-blue-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50'>
      {children}
    </ShadcnCard>
  );
}

export default memo(CardB);
