import React from 'react';
import { AuthProvider } from '@/hooks/AuthContext';
import { ReactNode } from 'react';

/**
 * Props for the SessionWrapper component.
 * @property {ReactNode} children - The child components to be wrapped with AuthProvider.
 */
interface SessionWrapperProps {
  /** React children to be rendered inside the AuthProvider */
  children: ReactNode;
}

export default function SessionWrapper({ children }: SessionWrapperProps) {
  return (
    <AuthProvider>
      {children}
    </AuthProvider>
  );
}
