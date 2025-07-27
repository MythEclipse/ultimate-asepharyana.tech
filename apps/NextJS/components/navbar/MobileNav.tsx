'use client';

import React from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { motion, AnimatePresence } from 'framer-motion';
import { Menu, X } from 'lucide-react';

import { Button } from '@/components/ui/button';
import { navLinks } from '@/lib/constants';
import UserMenu from './UserMenu';

const Logo = () => (
  <Link href='/' className='flex items-center gap-2'>
    <Image src='/Logo.svg' alt='Logo' width={28} height={28} />
    <span className='hidden text-base font-semibold sm:inline-block md:text-lg'>
      Asep Haryana
    </span>
  </Link>
);

interface MobileNavProps {
  isOpen: boolean;
  setIsOpen: (isOpen: boolean) => void;
}

export default function MobileNav({
  isOpen,
  setIsOpen,
}: MobileNavProps) {
  const toggleMenu = () => setIsOpen(!isOpen);

  const menuVariants = {
    hidden: { y: '-100%', opacity: 0.8 },
    visible: {
      y: 0,
      opacity: 1,
      transition: { type: 'spring' as const, stiffness: 300, damping: 25 },
    },
    exit: {
      y: '-100%',
      opacity: 0,
      transition: { duration: 0.3, ease: 'easeOut' as const },
    },
  };

  const listVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: { staggerChildren: 0.08, delayChildren: 0.15 },
    },
  };

  const itemVariants = {
    hidden: { opacity: 0, y: 20, filter: 'blur(5px)' },
    visible: {
      opacity: 1,
      y: 0,
      filter: 'blur(0px)',
      transition: { type: 'spring' as const, stiffness: 200 },
    },
  };

  return (
    <div className='md:hidden'>
      <Button onClick={toggleMenu} variant='ghost' size='icon'>
        <Menu className='h-6 w-6' />
        <span className='sr-only'>Buka Menu</span>
      </Button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className='fixed inset-0 z-50 bg-background/80 backdrop-blur-sm'
            onClick={toggleMenu}
          >
            <motion.div
              variants={menuVariants}
              initial='hidden'
              animate='visible'
              exit='exit'
              className='fixed inset-x-0 top-0 z-50 bg-background shadow-lg rounded-b-lg md:rounded-b-2xl'
              onClick={(e) => e.stopPropagation()}
            >
              <div className='flex items-center justify-between p-4 border-b'>
                <Logo />
                <Button onClick={toggleMenu} variant='ghost' size='icon'>
                  <X className='h-6 w-6' />
                  <span className='sr-only'>Tutup Menu</span>
                </Button>
              </div>
              <div className='p-6'>
                <motion.ul
                  variants={listVariants}
                  initial='hidden'
                  animate='visible'
                  className='flex flex-col items-center gap-4 sm:gap-6'
                >
                  {navLinks.map((link) => (
                    <motion.li key={link.href} variants={itemVariants}>
                      <Link
                        href={link.href}
                        onClick={toggleMenu}
                        className='text-xl font-medium text-muted-foreground hover:text-primary transition-colors'
                      >
                        {link.label}
                      </Link>
                    </motion.li>
                  ))}
                </motion.ul>
                <div className='mt-6 pt-4 border-t flex justify-center sm:mt-8 sm:pt-6'>
                  <UserMenu />
                </div>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
