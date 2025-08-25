import React from 'react';
import { FloatingDock } from './floating-dock';

interface FloatingDockDemoProps {
  content: Array<{ title: string; icon: React.ReactNode; href: string }>;
}

export default function FloatingDockDemo({ content }: FloatingDockDemoProps) {
  return <FloatingDock items={content} />;
}
