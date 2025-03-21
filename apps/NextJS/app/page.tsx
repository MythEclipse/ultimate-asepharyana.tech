import React from 'react';
import Bg from '@/components/background/Bg';
import AnimatedContent from '@/components/landing/AnimatedContent';
import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Home',
};

export default function Home() {
  return (
    <main>
      <div>
        <Bg>
          <AnimatedContent />
        </Bg>
      </div>
    </main>
  );
}
