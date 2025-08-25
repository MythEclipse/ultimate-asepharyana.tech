//@ts-check

const { composePlugins, withNx } = require('@nx/next');
const path = require('path');

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
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
      return config;
    },
  // Use this to set Nx-specific options
  // See: https://nx.dev/recipes/next/next-config-setup
  nx: {},
};

const plugins = [
  // Add more Next.js plugins to this list if needed.
  withNx,
];

module.exports = composePlugins(...plugins)(nextConfig);
