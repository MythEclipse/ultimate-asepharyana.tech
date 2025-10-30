'use client';

import React from 'react';
import DesktopNavLinks from './DesktopNavLinks';
import MobileNav from './MobileNav';
import Logo from '../logo/Logo';

export default function Navbar() {
  return (
    <header className="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur-sm">
      <div className="flex h-16 items-center justify-between px-4 md:px-8 lg:px-12 w-full">
        <div className="flex justify-start">
          <Logo />
        </div>
        <div className="flex justify-center">
          <DesktopNavLinks />
        </div>
        <div className="flex justify-end">
          <MobileNav />
        </div>
      </div>
    </header>
  );
}
