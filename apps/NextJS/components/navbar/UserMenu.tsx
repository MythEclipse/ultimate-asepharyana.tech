'use client';

import React from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { FcGoogle } from 'react-icons/fc';
import { useAuth } from '@/hooks/AuthContext';

export default function UserMenu() {
  const { user, logout } = useAuth();
  const [isOpen, setIsOpen] = React.useState(false);
  const buttonRef = React.useRef<HTMLButtonElement>(null);
  const menuRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
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

    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  }, []);

  return (
    <div className='relative'>
      {user ? (
        <>
          <button
            ref={buttonRef}
            className='w-10 h-10 rounded-full border border-blue-500 overflow-hidden'
            onClick={() => setIsOpen(!isOpen)}
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
              className='absolute right-0 mt-2 w-36 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md shadow-lg'
            >
              <Link
                prefetch={true}
                href='/dashboard'
                className='block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 text-center'
              >
                Dashboard
              </Link>
              <Link
                prefetch={true}
                href='/settings'
                className='block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 text-center'
              >
                Settings
              </Link>
              <button
                onClick={() => logout()}
                className='flex items-center gap-1 px-8 py-2 text-sm text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors'
              >
                <FcGoogle className='text-xl' />
                Sign out
              </button>
            </div>
          )}
        </>
      ) : (
        <Link href='/login'>
          <button className='px-4 py-2 bg-blue-500 text-white rounded-full hover:bg-blue-600 dark:bg-blue-700 dark:hover:bg-blue-800'>
            Login
          </button>
        </Link>
      )}
    </div>
  );
}
