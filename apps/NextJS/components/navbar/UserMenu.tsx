'use client';

import React, { useState, useRef, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { Session } from 'next-auth';
import { FcGoogle } from 'react-icons/fc';
import { signOut } from 'next-auth/react';

interface UserMenuProps {
  session: Session | null;
  loginUrl: string;
}

export default function UserMenu({ session, loginUrl }: UserMenuProps) {
  const [isOpen, setIsOpen] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);

  // Close dropdown if click is outside
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

    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  }, []);

  return (
    <div className='relative'>
      {session ? (
        <>
          <button
            ref={buttonRef}
            className='w-10 h-10 rounded-full border border-blue-500 overflow-hidden'
            onClick={() => setIsOpen(!isOpen)}
          >
            <Image
              src={session.user?.image || '/profile-circle-svgrepo-com.svg'}
              alt='Profile'
              width={40}
              height={40}
              className='rounded-full object-cover'
            />
          </button>
          {isOpen && (
            <div
              ref={menuRef}
              className='absolute right-0 mt-2 w-48 bg-white border border-gray-200 rounded-md shadow-lg'
            >
              <Link
                href='/dashboard'
                className='block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100'
              >
                Dashboard
              </Link>
              <Link
                href='/settings'
                className='block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100'
              >
                Settings
              </Link>
              <button
                onClick={() => signOut({ redirectTo: '/' })}
                className='flex items-center gap-3 px-4 py-2 text-sm text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors'
              >
                <FcGoogle className='text-2xl' />
                Sign out
              </button>
            </div>
          )}
        </>
      ) : (
        <Link href={loginUrl}>
          <button className='px-4 py-2 bg-blue-500 text-white rounded-full'>
            Login
          </button>
        </Link>
      )}
    </div>
  );
}
