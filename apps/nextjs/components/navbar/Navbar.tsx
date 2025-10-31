'use client';

import React from 'react';
import Link from 'next/link';
import DesktopNavLinks from './DesktopNavLinks';
import MobileNav from './MobileNav';
import Logo from '../logo/Logo';
import ThemeToggle from '../theme/ThemeToggle';
import { Button } from '../ui/button';
import { useAuth } from '../../lib/auth-context';

export default function Navbar() {
  const { user, logout } = useAuth();

  return (
    <header className="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur-sm">
      <div className="flex h-16 items-center justify-between px-4 md:px-8 lg:px-12 w-full">
        <div className="flex justify-start">
          <Logo />
        </div>
        <div className="flex justify-center">
          <DesktopNavLinks />
        </div>
        <div className="flex items-center gap-2 justify-end">
          <ThemeToggle />
          <div className="hidden md:flex items-center gap-2">
            {user ? (
              <>
                <span className="text-sm text-muted-foreground">
                  {user.name}
                </span>
                <Button onClick={logout} variant="outline" size="sm">
                  Logout
                </Button>
              </>
            ) : (
              <>
                <Link href="/login">
                  <Button variant="ghost" size="sm">
                    Login
                  </Button>
                </Link>
                <Link href="/register">
                  <Button variant="default" size="sm">
                    Register
                  </Button>
                </Link>
              </>
            )}
          </div>
          <MobileNav />
        </div>
      </div>
    </header>
  );
}
