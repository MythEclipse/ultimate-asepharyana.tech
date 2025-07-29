import path from 'path';
import { NextConfig } from 'next';
import withFlowbiteReact from 'flowbite-react/plugin/nextjs';

/** @type {import('next').NextConfig} */
const nextConfig: NextConfig = {
  output: process.env.DOCKER === 'enable' ? 'standalone' : undefined,
  experimental: {
    nextScriptWorkers: true,
    webpackMemoryOptimizations: true,
  },
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'lh3.googleusercontent.com',
      },
      {
        protocol: 'http',
        hostname: 'localhost',
        port: '4090',
        pathname: '/api/imageproxy',
      },
      {
        protocol: 'https',
        hostname: 'asepharyana.tech',
      },
    ],
    minimumCacheTTL: 86400,
    formats: ['image/webp', 'image/avif'],
    deviceSizes: [640, 750, 828, 1080, 1200, 1920, 2048, 3840],
  },
  env: {
    NEXT_PUBLIC_KOMIK: process.env.NEXT_PUBLIC_KOMIK,
    NEXT_PUBLIC_ANIME: process.env.NEXT_PUBLIC_ANIME,
  },
  async redirects() {
    return [
      {
        source: '/komik/:slug',
        destination: '/komik/:slug/1',
        permanent: false,
      },
      {
        source: '/anime/:slug',
        destination: '/anime/:slug/1',
        permanent: false,
      },
    ];
  },
  async headers() {
    return [
      {
        source: '/api/:path*',
        headers: [
          { key: 'Access-Control-Allow-Origin', value: '*' },
          {
            key: 'Access-Control-Allow-Methods',
            value: 'GET, POST, PUT, DELETE, OPTIONS',
          },
          {
            key: 'Access-Control-Allow-Headers',
            value: 'Content-Type, Authorization',
          },
        ],
      },
    ];
  },
  reactStrictMode: true,
  compress: true,
  productionBrowserSourceMaps: true,
  transpilePackages: ['@asepharyana/ui', 'swagger-ui'],
  webpack: (config, { isServer }) => {
    // Exclude @prisma/client and .prisma/client from client-side bundle
    if (!isServer) {
      config.resolve.alias['@prisma/client'] = false;
      config.resolve.alias['.prisma/client'] = false;
    }
    // Ensure webpack resolves @hooks alias like tsconfig
    config.resolve.alias['@hooks'] = path.resolve(__dirname, 'core/hooks');
    return config;
  },
};

export default withFlowbiteReact(nextConfig);
