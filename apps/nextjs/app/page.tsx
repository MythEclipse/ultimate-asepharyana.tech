// apps/NextJS/app/page.tsx  a

import React from 'react';
import Bg from '../components/background/Bg';
import AnimatedContent from '../components/landing/AnimatedContent';
import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Home',
};

export default function Home() {
  return (
    <main className="min-h-screen bg-background text-foreground">
      <div className="flex flex-col items-center justify-center min-h-[calc(100vh-64px)] p-4 md:p-8 lg:p-12">
        <Bg>
          <AnimatedContent />
        </Bg>
      </div>
    </main>
  );
}
