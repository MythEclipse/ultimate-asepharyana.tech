import DockKomik from '../../components/komik/DockKomik';
import React, { memo } from 'react';
import { HiBookmark, HiHome, HiOutlineSearch } from 'react-icons/hi';
import { FaPencilAlt, FaDragon, FaBook } from 'react-icons/fa';

const komik = [
  {
    title: 'Home',
    icon: (
      <HiHome className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/komik',
  },
  {
    title: 'Latest Manga',
    icon: (
      <FaPencilAlt className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/komik/manga/page/1',
  },
  {
    title: 'Latest Manhua',
    icon: (
      <FaDragon className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/komik/manhua/page/1',
  },
  {
    title: 'Latest Manhwa',
    icon: (
      <FaBook className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/komik/manhwa/page/1',
  },
  {
    title: 'Search',
    icon: (
      <HiOutlineSearch className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: `/komik/search/${encodeURIComponent('a')}`, // Placeholder link for the menu item
  },
  {
    title: 'Bookmark',
    icon: (
      <HiBookmark className="h-full w-full text-neutral-500 dark:text-neutral-300" />
    ),
    href: '/komik/bookmark/1',
  },
];

function Layout({ children }: { readonly children: React.ReactNode }) {
  return (
    <>
      <div className="">{children}</div>
      <DockKomik content={komik} />
    </>
  );
}

export default memo(Layout);
