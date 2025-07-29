'use client';

import React, { useState, useRef, memo } from 'react';
import Link from 'next/link';
import { useSession, signOut } from 'next-auth/react';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '../ui/dropdown-menu';
import { Avatar, AvatarFallback, AvatarImage } from '../ui/avatar';

// UserMenu component to display user information and provide a dropdown menu for navigation and sign-out.
const UserMenu = () => {
  const { data: session, status } = useSession();
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);


  if (status === 'loading') {
    return (
      <div className='flex items-center gap-2'>
        <div className='w-8 h-8 rounded-full bg-gray-300 animate-pulse' />
        <div className='w-20 h-4 bg-gray-300 animate-pulse' />
      </div>
    );
  }

  if (status === 'unauthenticated') {
    return (
   <Link
  href='/login'
  className='rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2'
>
  Sign In
</Link>
    );
  }

  return (
    <div className='relative' ref={dropdownRef}>
      <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
        <DropdownMenuTrigger asChild>
          <Avatar className='cursor-pointer w-8 h-8'>
            <AvatarImage src={session?.user?.image || 'https://github.com/shadcn.png'} alt='User Avatar' />
            <AvatarFallback>{session?.user?.name?.[0] || 'CN'}</AvatarFallback>
          </Avatar>
        </DropdownMenuTrigger>
        <DropdownMenuContent className='w-56' align='end' forceMount>
          <DropdownMenuLabel>
            <div className='flex flex-col space-y-1'>
              <p className='text-sm font-medium leading-none'>{session?.user?.name}</p>
              <p className='text-xs leading-none text-muted-foreground'>
                {session?.user?.email}
              </p>
            </div>
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild>
            <Link href="/dashboard">Dashboard</Link>
          </DropdownMenuItem>
          <DropdownMenuItem asChild>
            <Link href="/settings">Settings</Link>
          </DropdownMenuItem>
          <DropdownMenuItem asChild>
            <Link href="/profile">Profile</Link>
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
