'use client';

import React, { useState, useEffect, useRef, memo, KeyboardEvent } from 'react';
import Link from 'next/link';



function UserMenu() {
  // Remove useAuth usage
  const [isOpen, setIsOpen] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);

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

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
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

  // Only show login button, since user context is removed
  return (
    <div className='relative'>
      <Link href='/login'>
        <button
          className='px-4 py-2 bg-blue-500 text-white rounded-full hover:bg-blue-600 dark:bg-blue-700 dark:hover:bg-blue-800'
          aria-label="Login"
        >
          Login
        </button>
      </Link>
    </div>
  );
}

export default memo(UserMenu);
