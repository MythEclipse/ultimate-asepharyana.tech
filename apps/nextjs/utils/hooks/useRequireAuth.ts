import { useSession } from 'next-auth/react';
import type { Session } from 'next-auth';

export function useRequireAuth(): Session {
  const { data: session } = useSession();
  if (!session?.user) {
    throw new Error('Authentication required');
  }
  return session;
}
