// 'use client'
// apps/NextJS/app/layout.tsx

import type { Metadata } from 'next';
import './globals.css';
// import { Analytics } from '@vercel/analytics/react';
// import { SpeedInsights } from '@vercel/speed-insights/next';
import { ThemeProvider } from '../components/providers/theme-provider';
import Navbar from '../components/navbar/Navbar';
import { Toaster } from 'sonner';

export const metadata: Metadata = {
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
          <Navbar />
          {children}
          <Toaster />
        </ThemeProvider>
        {/* <Analytics /> */}
        {/* <SpeedInsights /> */}
      </body>
    </html>
  );
}
