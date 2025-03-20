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
      <div style={{ height: '100vh', width: '100vw', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
        <Bg>
          <AnimatedContent />
        </Bg>
      </div>
    </main>
  );
}
