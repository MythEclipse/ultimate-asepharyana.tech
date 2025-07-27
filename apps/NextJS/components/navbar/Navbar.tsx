'use client';

import React, { useState } from 'react';
import { navLinks } from '@/lib/constants';
import Link from 'next/link';
import Image from 'next/image';
import { usePathname } from 'next/navigation'; // Keep usePathname for DesktopNavLinks
import { motion, AnimatePresence } from 'framer-motion'; // Keep these for DesktopNavLinks
import { Menu, X } from 'lucide-react'; // Keep these for MobileNav

import { Button } from '@/components/ui/button'; // Keep this for MobileNav
import UserMenu from './UserMenu';
import DesktopNavLinks from './DesktopNavLinks';
import MobileNav from './MobileNav';

const Logo = () => (
  <Link href='/' className='flex items-center gap-2'>
    <Image src='/Logo.svg' alt='Logo' width={28} height={28} />
    <span className='hidden text-base font-semibold sm:inline-block md:text-lg'>
      Asep Haryana
    </span>
  </Link>
);

export default function Navbar() {
  const [isMobileNavOpen, setIsMobileNavOpen] = useState(false);

  return (
    <header className='sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur-sm'>
      <div className='flex h-16 items-center justify-between px-4 md:px-8 lg:px-12 w-full'>
        <div className='flex justify-start'>
          <Logo />
        </div>
        <div className='flex justify-center'>
          <DesktopNavLinks />
        </div>
        <div className='flex justify-end'>
          <div className='hidden md:block'>
            <UserMenu />
          </div>
          <MobileNav isOpen={isMobileNavOpen} setIsOpen={setIsMobileNavOpen} />
        </div>
      </div>
    </header>
  );
}
