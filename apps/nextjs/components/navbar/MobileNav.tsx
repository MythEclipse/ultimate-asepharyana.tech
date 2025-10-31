// apps/NextJS/components/navbar/MobileNav.tsx
'use client';

import React, { memo, useRef, useEffect } from 'react';
import Link from 'next/link';
import { motion, AnimatePresence } from 'framer-motion';
import { Menu, X } from 'lucide-react';

import { Button } from '../ui/button';
import { navLinks } from '../../utils/constants';
import { useGlobalStore } from '../../utils/hooks/useGlobalStore';
import Logo from '../logo/Logo';
import ThemeToggle from '../theme/ThemeToggle';
import { useAuth } from '../../lib/auth-context';

function MobileNav() {
  const isOpen = useGlobalStore((s) => s.isMobileNavOpen);
  const setIsOpen = useGlobalStore((s) => s.setMobileNavOpen);
  const { user, logout } = useAuth();

  const menuRef = useRef<HTMLDivElement>(null);

  const toggleMenu = () => setIsOpen(!isOpen);

  // Focus management and ESC key to close
  useEffect(() => {
    if (isOpen && menuRef.current) {
      menuRef.current.focus();
    }
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setIsOpen(false);
      }
    };
    if (isOpen) {
      document.addEventListener('keydown', handleKeyDown);
    }
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [isOpen, setIsOpen]);

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
    <nav className="md:hidden" aria-label="Mobile Navigation">
      <Button
        onClick={toggleMenu}
        variant="ghost"
        size="icon"
        aria-label="Open navigation menu"
        aria-controls="mobile-nav-menu"
        aria-expanded={isOpen}
        aria-haspopup="true"
      >
        <Menu className="h-6 w-6" aria-hidden="true" />
        <span className="sr-only">Open Menu</span>
      </Button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm"
            onClick={toggleMenu}
          >
            <motion.div
              ref={menuRef}
              variants={menuVariants}
              initial="hidden"
              animate="visible"
              exit="exit"
              className="fixed inset-x-0 top-0 z-50 bg-background shadow-lg rounded-b-lg md:rounded-b-2xl"
              onClick={(e) => e.stopPropagation()}
              id="mobile-nav-menu"
              tabIndex={-1}
              role="menu"
              aria-label="Main menu"
            >
              <div className="flex items-center justify-between p-4 border-b">
                <Logo />
                <Button
                  onClick={toggleMenu}
                  variant="ghost"
                  size="icon"
                  aria-label="Close navigation menu"
                >
                  <X className="h-6 w-6" aria-hidden="true" />
                  <span className="sr-only">Close Menu</span>
                </Button>
              </div>
              <div className="p-6">
                <motion.ul
                  variants={listVariants}
                  initial="hidden"
                  animate="visible"
                  className="flex flex-col items-center gap-4 sm:gap-6"
                  role="menu"
                  aria-label="Navigation links"
                >
                  {navLinks.map((link) => (
                    <motion.li
                      key={link.href}
                      variants={itemVariants}
                      role="none"
                    >
                      <Link
                        href={link.href}
                        onClick={toggleMenu}
                        className="text-xl font-medium text-muted-foreground hover:text-primary transition-colors"
                        role="menuitem"
                        tabIndex={0}
                        aria-label={link.label}
                      >
                        {link.label}
                      </Link>
                    </motion.li>
                  ))}
                </motion.ul>

                {/* Theme Toggle and Auth */}
                <div className="mt-6 flex flex-col items-center gap-4 border-t pt-6">
                  <div className="flex items-center gap-2">
                    <span className="text-sm text-muted-foreground">Theme:</span>
                    <ThemeToggle />
                  </div>
                  {user ? (
                    <div className="flex flex-col items-center gap-2">
                      <span className="text-sm text-muted-foreground">
                        {user.name}
                      </span>
                      <Button onClick={() => { logout(); toggleMenu(); }} variant="outline" size="sm" className="w-full">
                        Logout
                      </Button>
                    </div>
                  ) : (
                    <div className="flex flex-col gap-2 w-full">
                      <Link href="/login" onClick={toggleMenu} className="w-full">
                        <Button variant="outline" size="sm" className="w-full">
                          Login
                        </Button>
                      </Link>
                      <Link href="/register" onClick={toggleMenu} className="w-full">
                        <Button variant="default" size="sm" className="w-full">
                          Register
                        </Button>
                      </Link>
                    </div>
                  )}
                </div>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </nav>
  );
}

export default memo(MobileNav);
