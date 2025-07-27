'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { usePathname } from 'next/navigation';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Menu,
  X,
  LogIn,
  LogOut,
  LayoutDashboard,
  Settings,
} from 'lucide-react';

import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { useAuth } from '@/hooks/AuthContext';

const navLinks = [
  { href: '/', label: 'Home' },
  { href: '/docs', label: 'Docs' },
  { href: '/project', label: 'Project' },
];

const Logo = () => (
  <Link href='/' className='flex items-center gap-2'>
    <Image src='/Logo.svg' alt='Logo' width={28} height={28} />
    <span className='hidden text-base font-semibold sm:inline-block md:text-lg'>
      Asep Haryana
    </span>
  </Link>
);

const UserNav = () => {
  const { user, logout } = useAuth();
  const pathname = usePathname();

  const loginUrl = `/login?callbackUrl=${encodeURIComponent(pathname)}`;

  if (!user) {
    return (
      <Button asChild>
        <Link href={loginUrl}>
          <LogIn className='mr-2 h-4 w-4' />
          Login
        </Link>
      </Button>
    );
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
          <Button variant='ghost' className='relative h-10 w-10 rounded-full'>
            <Avatar className='h-10 w-10 border-2 border-transparent group-hover:border-primary'>
              <AvatarImage
                src={user.image || ''}
                alt={user.email || 'User'}
              />
              <AvatarFallback>
                {user.email?.charAt(0).toUpperCase()}
              </AvatarFallback>
            </Avatar>
          </Button>
        </motion.div>
      </DropdownMenuTrigger>
      <DropdownMenuContent className='w-full sm:w-56' align='end' forceMount>
        <DropdownMenuLabel className='font-normal'>
          <div className='flex flex-col space-y-1'>
            <p className='text-sm font-medium leading-none'>
              {user.email}
            </p>
            <p className='text-xs leading-none text-muted-foreground'>
              {user.email}
            </p>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem asChild>
          <Link href='/dashboard'>
            <LayoutDashboard className='mr-2 h-4 w-4' />
            Dashboard
          </Link>
        </DropdownMenuItem>
        <DropdownMenuItem asChild>
          <Link href='/settings'>
            <Settings className='mr-2 h-4 w-4' />
            Pengaturan
          </Link>
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem
          onClick={() => logout()}
          className='text-destructive focus:text-destructive cursor-pointer'
        >
          <LogOut className='mr-2 h-4 w-4' />
          Keluar
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

const DesktopNav = () => {
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
};

const MobileNav = () => {
  const [isOpen, setIsOpen] = useState(false);
  const toggleMenu = () => setIsOpen(!isOpen);;

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
                  <UserNav />
                </div>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

export default function Navbar() {
  return (
    <header className='sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur-sm'>
      <div className='flex h-16 items-center justify-between px-4 md:px-8 lg:px-12 w-full'>
        <div className='flex justify-start'>
          <Logo />
        </div>
        <div className='flex justify-center'>
          <DesktopNav />
        </div>
        <div className='flex justify-end'>
          <div className='hidden md:block'>
            <UserNav />
          </div>
          <MobileNav />
        </div>
      </div>
    </header>
  );
}
