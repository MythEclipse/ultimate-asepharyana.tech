import DockKomik from '../../components/komik/DockKomik';
import React, { memo } from 'react';
import { HiBookmark, HiHome, HiOutlineSearch } from 'react-icons/hi';
import { MdUpdate } from 'react-icons/md';
import { FaCheckCircle } from 'react-icons/fa'; // Ganti FaDragon dengan FaCheckCircle

const anime = [
  {
    title: 'Home',
    icon: (
      <HiHome className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/anime',
  },
  {
    title: 'Ongoing Anime',
    icon: (
      <MdUpdate className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/anime/ongoing-anime/1',
  },
  {
    title: 'Complete Anime',
    icon: (
      <FaCheckCircle className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ), // Ganti dengan ikon ceklis
    href: '/anime/complete-anime/1',
  },
  {
    title: 'Search',
    icon: (
      <HiOutlineSearch className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: `/anime/search/${encodeURIComponent('a')}`,
  },
  {
    title: 'Bookmark',
    icon: (
      <HiBookmark className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/anime/bookmark/1',
  },
];

function Layout({ children }: { readonly children: React.ReactNode }) {
  return (
    <>
      <div className="">{children}</div>
      <DockKomik content={anime} />
    </>
  );
}

export default memo(Layout);
