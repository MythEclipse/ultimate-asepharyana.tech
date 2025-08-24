import DockKomik from '@/components/komik/DockKomik';
import React from 'react';
import { FaCheckCircle } from 'react-icons/fa';
import { HiHome, HiOutlineSearch } from 'react-icons/hi';
import { MdUpdate } from 'react-icons/md';

const anime = [
  {
    title: 'Home',
    icon: (
      <HiHome className='h-full w-full text-neutral-500 dark:text-neutral-300' />
    ),
    href: '/anime2',
  },
  {
    title: 'Ongoing Anime',
    icon: (
      <MdUpdate className='h-full w-full text-neutral-500 dark:text-neutral-300' />
    ),
    href: '/anime2/ongoing-anime/1',
  },
  {
    title: 'Complete Anime',
    icon: (
      <FaCheckCircle className='h-full w-full text-neutral-500 dark:text-neutral-300' />
    ), // Ganti dengan ikon ceklis
    href: '/anime2/complete-anime/1',
  },
  {
    title: 'Search',
    icon: (
      <HiOutlineSearch className='h-full w-full text-neutral-500 dark:text-neutral-300' />
    ),
    href: `/anime2/search/${encodeURIComponent('a')}`,
  },
  // {
  //   title: 'Bookmark',
  //   icon: (
  //     <HiBookmark className='h-full w-full text-neutral-500 dark:text-neutral-300' />
  //   ),
  //   href: '/anime2/bookmark/1',
  // },
];

export default function Layout({ children }: { readonly children: React.ReactNode }) {
  return (
    <>
      <div className=''>{children}</div>
      <DockKomik content={anime} />
    </>
  );
}
