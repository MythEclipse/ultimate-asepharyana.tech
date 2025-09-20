import { NavLink } from '../types/types';

export const navLinks: NavLink[] = [
  { href: '/', label: 'Home' },
  // { href: '/docs', label: 'Docs' },
  { href: '/project', label: 'Project' },
];

export const PRODUCTION = process.env.NEXT_PUBLIC_PRODUCTION_URL || 'https://asepharyana.tech';
