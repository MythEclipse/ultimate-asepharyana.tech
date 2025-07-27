// apps/NextJS/components/navbar/Navbar.tsx
'use client';

import React from 'react';
import DesktopNavLinks from './DesktopNavLinks';
import MobileNav from './MobileNav';
import NavToggleButton from './NavToggleButton';

function Navbar() {
  return (
    <header className='w-full bg-background shadow-sm'>
      <nav className='container mx-auto flex items-center justify-between py-4 px-4'>
        <div className='flex items-center gap-4'>
          {/* Logo or Brand */}
          <span className='text-xl font-bold text-primary'>Asep Haryana</span>
        </div>
        <DesktopNavLinks />
        <div className='flex md:hidden items-center gap-2'>
          <NavToggleButton />
          <MobileNav />
        </div>
      </nav>
    </header>
  );
}

export default Navbar;
