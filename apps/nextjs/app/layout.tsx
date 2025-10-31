// 'use client'
// apps/NextJS/app/layout.tsx

import type { Metadata } from 'next';

// Force dynamic rendering for all routes to avoid prerender issues
export const dynamic = 'force-dynamic';
import './globals.css';
// import { Analytics } from '@vercel/analytics/react';
// import { SpeedInsights } from '@vercel/speed-insights/next';
import { ThemeProvider } from '../components/providers/theme-provider';
import { AuthProvider } from '../lib/auth-context';
import Navbar from '../components/navbar/Navbar';
import { Toaster } from 'sonner';
export const metadata: Metadata = {
  metadataBase: new URL('https://asepharyana.tech'),
  title: 'Asepharyana',
  description: 'A personal website by Asep Haryayana',
  icons: {
    icon: '/favicon.png',
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`antialiased`}>
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
        {/* <Analytics /> */}
        {/* <SpeedInsights /> */}
      </body>
    </html>
  );
}
