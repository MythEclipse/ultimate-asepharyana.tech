'use client';

import { ThemeProvider } from '../providers/theme-provider';
import { AuthProvider } from '../../lib/auth-context';
import Navbar from '../navbar/Navbar';
import { Toaster } from 'sonner';

export function ClientLayout({ children }: { children: React.ReactNode }) {
  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange
    >
      <AuthProvider>
        <Navbar />
        {children}
        <Toaster />
      </AuthProvider>
    </ThemeProvider>
  );
}
