import { NavLink } from '../types/types';
import { getBaseUrlConfig } from './url-utils';

export const navLinks: NavLink[] = [
  { href: '/', label: 'Home' },
  // { href: '/docs', label: 'Docs' },
  { href: '/project', label: 'Project' },
];

// Use centralized URL configuration
export const PRODUCTION = getBaseUrlConfig().current;

// Re-export other URL constants for convenience
export {
  APIURLSERVER,
  APIURLCLIENT,
  APIURL,
  BaseUrl,
  KOMIK,
} from '../utils/url-utils';
