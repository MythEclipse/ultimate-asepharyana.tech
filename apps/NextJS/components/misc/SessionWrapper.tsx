import React from 'react';
import { AuthProvider } from '@/hooks/AuthContext';
import { ReactNode } from 'react';

interface SessionWrapperProps {
  children: ReactNode;
}

export default function SessionWrapper({ children }: SessionWrapperProps) {
  return (
    <AuthProvider>
      {children}
    </AuthProvider>
  );
}
