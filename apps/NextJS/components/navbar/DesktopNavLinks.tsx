'use client';

import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { motion } from 'framer-motion';
import { navLinks } from '@/lib/constants';

export default function DesktopNavLinks() {
  const pathname = usePathname();

  return (
    <nav className='hidden md:flex justify-center'>
      <ul className='flex items-center gap-2 rounded-full bg-muted/50 p-1'>
        {navLinks.map((link) => (
          <li key={link.href}>
            <Link
              href={link.href}
              className='relative px-3 py-1.5 text-sm font-medium transition-colors md:px-4 md:py-2'
            >
              <span
                className={`relative z-10 ${pathname === link.href ? 'text-primary' : 'text-muted-foreground hover:text-foreground'}`}
              >
                {link.label}
              </span>
              {pathname === link.href && (
                <motion.div
                  layoutId='active-pill'
                  transition={{ type: 'spring', stiffness: 400, damping: 30 }}
                  className='absolute inset-0 z-0 rounded-full bg-background shadow-sm'
                />
              )}
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}
