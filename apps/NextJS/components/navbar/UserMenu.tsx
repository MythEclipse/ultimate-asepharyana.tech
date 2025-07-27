'use client';

import React, { useState, useEffect, useRef, memo, KeyboardEvent } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { FcGoogle } from 'react-icons/fc';
import { useAuth } from '@/hooks/AuthContext';

function UserMenu() {
  const { user, logout } = useAuth();
  const [isOpen, setIsOpen] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);

  // Focus management for menu
  useEffect(() => {
    if (isOpen && menuRef.current) {
      menuRef.current.focus();
    }
  }, [isOpen]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        menuRef.current &&
        !menuRef.current.contains(event.target as Node) &&
        buttonRef.current &&
        !buttonRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    const handleEscape = (event: globalThis.KeyboardEvent) => {
      if (event.key === 'Escape') {
        setIsOpen(false);
        buttonRef.current?.focus();
      }
    };

    document.addEventListener('click', handleClickOutside);
    document.addEventListener('keydown', handleEscape);
    return () => {
      document.removeEventListener('click', handleClickOutside);
      document.removeEventListener('keydown', handleEscape);
    };
  }, []);

  const handleButtonKeyDown = (event: KeyboardEvent<HTMLButtonElement>) => {
    if (event.key === 'Enter' || event.key === ' ') {
      setIsOpen((prev) => !prev);
    }
    if (event.key === 'ArrowDown' && !isOpen) {
      setIsOpen(true);
      setTimeout(() => {
        menuRef.current?.focus();
      }, 0);
    }
  };

  return (
    <div className='relative'>
      {user ? (
        <>
          <button
            ref={buttonRef}
            className='w-10 h-10 rounded-full border border-blue-500 overflow-hidden'
            onClick={() => setIsOpen(!isOpen)}
            aria-haspopup="menu"
            aria-expanded={isOpen}
            aria-controls="user-menu-dropdown"
            aria-label="Open user menu"
            onKeyDown={handleButtonKeyDown}
          >
            <Image
              src={user.image || '/profile-circle-svgrepo-com.svg'}
              width={40}
              height={40}
              className='w-10 h-10 rounded-full object-cover'
              alt='User Avatar'
            />
          </button>
          {isOpen && (
            <div
              ref={menuRef}
              id="user-menu-dropdown"
              className='absolute right-0 mt-2 w-36 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md shadow-lg'
              tabIndex={-1}
              role="menu"
              aria-label="User menu"
            >
              <Link
                prefetch={true}
                href='/dashboard'
                className='block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 text-center'
                role="menuitem"
                tabIndex={0}
              >
                Dashboard
              </Link>
              <Link
                prefetch={true}
                href='/settings'
                className='block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 text-center'
                role="menuitem"
                tabIndex={0}
              >
                Settings
              </Link>
              <button
                onClick={() => logout()}
                className='flex items-center gap-1 px-8 py-2 text-sm text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors'
                role="menuitem"
                tabIndex={0}
                aria-label="Sign out"
              >
                <FcGoogle className='text-xl' aria-hidden="true" />
                Sign out
              </button>
            </div>
          )}
        </>
      ) : (
        <Link href='/login'>
          <button
            className='px-4 py-2 bg-blue-500 text-white rounded-full hover:bg-blue-600 dark:bg-blue-700 dark:hover:bg-blue-800'
            aria-label="Login"
          >
            Login
          </button>
        </Link>
      )}
    </div>
  );
}

export default memo(UserMenu);
