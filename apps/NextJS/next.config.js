/** @type {import('next').NextConfig} */
const withPWA = require('next-pwa')({
  dest: 'public',
  register: true,
  skipWaiting: true,
  disable: process.env.NODE_ENV === 'development',
  maximumFileSizeToCacheInBytes: 2500000
})

const nextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: '**'
      }
    ],
    minimumCacheTTL: 86400,
    formats: ['image/webp', 'image/avif'],
    deviceSizes: [640, 750, 828, 1080, 1200, 1920, 2048, 3840]
  },
  env: {
    NEXT_PUBLIC_KOMIK: process.env.NEXT_PUBLIC_KOMIK,
    NEXT_PUBLIC_ANIME: process.env.NEXT_PUBLIC_ANIME,
    DATABASE_URL: process.env.DATABASE_URL,
    SECRET_KEY: process.env.SECRET_KEY
  },
  async redirects() {
    return [
      {
        source: '/komik/:slug',
        destination: '/komik/:slug/1',
        permanent: false
      },
      {
        source: '/anime/:slug',
        destination: '/anime/:slug/1',
        permanent: false
      }
    ]
  },
  reactStrictMode: true,
  compress: true,
  productionBrowserSourceMaps: true,
  experimental: {
    optimizeCss: true,
    nextScriptWorkers: true
  },
  webpack: (config) => {
    config.optimization.splitChunks = {
      chunks: 'all',
      maxSize: 256000,
      minSize: 20000
    }
    return config
  }
}

module.exports = withPWA(nextConfig)
