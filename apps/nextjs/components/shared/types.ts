// Shared type definitions for components

export interface ClientUser {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
  emailVerified: Date | null;
  role: string;
}

export interface MediaTypeColors {
  [key: string]: string;
}

export const MEDIA_TYPE_COLORS: MediaTypeColors = {
  Manga: 'bg-red-500 hover:bg-red-600',
  Manhua: 'bg-green-500 hover:bg-green-600',
  Manhwa: 'bg-blue-500 hover:bg-blue-600',
  BD: 'bg-purple-500 hover:bg-purple-500',
  TV: 'bg-yellow-500 hover:bg-yellow-600',
  OVA: 'bg-pink-500 hover:bg-pink-600',
  ONA: 'bg-indigo-500 hover:bg-indigo-600',
};

export interface BaseComponentProps {
  className?: string;
  children?: React.ReactNode;
}

export interface LoadingProps {
  loading?: boolean;
}

export interface ImageFallbackOptions {
  fallbackUrl?: string;
  useProxy?: boolean;
  useCdn?: boolean;
}
