import { useAuth } from '../../lib/auth-context';
import type { User } from '../../types/auth';

export function useRequireAuth(): User {
  const { user } = useAuth();
  if (!user) {
    throw new Error('Authentication required');
  }
  return user;
}
