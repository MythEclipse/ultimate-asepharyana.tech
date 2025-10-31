// apps/NextJS/app/layout.tsx

import type { Metadata } from 'next';

import './globals.css';
// import { Analytics } from '@vercel/analytics/react';
// import { SpeedInsights } from '@vercel/speed-insights/next';
import { ClientLayout } from '../components/layout/ClientLayout';

export const dynamic = 'force-dynamic';

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
        <ClientLayout>{children}</ClientLayout>
        {/* <Analytics /> */}
        {/* <SpeedInsights /> */}
      </body>
    </html>
  );
}
