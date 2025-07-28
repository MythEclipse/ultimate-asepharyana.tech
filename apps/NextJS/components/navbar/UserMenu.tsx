'use client';

import React, { useState, useEffect, useRef, memo, KeyboardEvent } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { useSession, signOut } from 'next-auth/react'; // Changed import
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '../ui/dropdown-menu';
import { Avatar, AvatarFallback, AvatarImage } from '../ui/avatar';

// UserMenu component to display user information and provide a dropdown menu for navigation and sign-out.
const UserMenu = () => {
  const { data: session, status } = useSession(); // Changed to useSession
  const [isOpen, setIsOpen] = useState(false); // State to control dropdown visibility
  const dropdownRef = useRef<HTMLDivElement>(null); // Ref for the dropdown menu

  // Effect to handle clicks outside the dropdown menu, closing it when clicked elsewhere.
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside); // Add mousedown event listener
    return () => {
      document.removeEventListener('mousedown', handleClickOutside); // Clean up the event listener
    };
  }, [dropdownRef]); // Dependency array includes dropdownRef

  // Function to handle keydown events, specifically for the Escape key to close the dropdown.
  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      setIsOpen(false);
    }
  };

  // Display loading state while authentication status is being determined.
  if (status === 'loading') { // Changed condition
    return (
      <div className='flex items-center gap-2'>
        <div className='w-8 h-8 rounded-full bg-gray-300 animate-pulse' />
        <div className='w-20 h-4 bg-gray-300 animate-pulse' />
      </div>
    );
  }

  // If not authenticated, display a sign-in link.
  if (status === 'unauthenticated') { // Changed condition
    return (
      <Link href='/login' className='text-gray-700 hover:text-gray-900'>
        Sign In
      </Link>
    );
  }

  // If authenticated, display the user's avatar and a dropdown menu.
  return (
    <div className='relative' ref={dropdownRef}>
      <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
        <DropdownMenuTrigger asChild>
          <Avatar className='cursor-pointer w-8 h-8'>
            <AvatarImage src={session?.user?.image || 'https://github.com/shadcn.png'} alt='User Avatar' /> {/* Changed to session?.user?.image */}
            <AvatarFallback>{session?.user?.name?.[0] || 'CN'}</AvatarFallback> {/* Changed to session?.user?.name */}
          </Avatar>
        </DropdownMenuTrigger>
        <DropdownMenuContent className='w-56' align='end' forceMount>
          <DropdownMenuLabel>
            <div className='flex flex-col space-y-1'>
              <p className='text-sm font-medium leading-none'>{session?.user?.name}</p> {/* Changed to session?.user?.name */}
              <p className='text-xs leading-none text-muted-foreground'>
                {session?.user?.email} {/* Changed to session?.user?.email */}
              </p>
            </div>
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild>
            <Link href='/dashboard'>
              Dashboard
            </Link>
          </DropdownMenuItem>
          <DropdownMenuItem asChild>
            <Link href='/settings'>
              Settings
            </Link>
          </DropdownMenuItem>
          <DropdownMenuItem asChild>
            <Link href='/profile'>
              Profile
            </Link>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem onClick={() => signOut()} className='cursor-pointer'>
            Sign Out
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};

export default memo(UserMenu);
