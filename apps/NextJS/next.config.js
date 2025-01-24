/** @type {import('next').NextConfig} */
const nextConfig = {
  // Konfigurasi gambar generik
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: '**',
      },
    ],
    minimumCacheTTL: 86400, // 1 hari
    formats: ['image/webp', 'image/avif'],
    deviceSizes: [640, 750, 828, 1080, 1200, 1920, 2048, 3840],
  },

  // Environment variables dasar
  env: {
    NEXT_PUBLIC_KOMIK: process.env.NEXT_PUBLIC_KOMIK,
    NEXT_PUBLIC_ANIME: process.env.NEXT_PUBLIC_ANIME,
    DATABASE_URL: process.env.DATABASE_URL,
    SECRET_KEY: process.env.SECRET_KEY,
  },

  // Security headers
  async headers() {
    return [
      {
      source: '/(.*)',
      headers: [
        { key: 'X-Frame-Options', value: 'DENY' },
        { key: 'X-Content-Type-Options', value: 'nosniff' },
        { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
        {
        key: 'Content-Security-Policy',
        value: [
          "default-src 'self'",
          "script-src 'self' 'unsafe-inline'",
          "style-src 'self' 'unsafe-inline'",
          "img-src * data:",
          "font-src 'self'",
          "connect-src '*'",
          "media-src *",
          "object-src 'none'"
        ].join('; ')
        },
        { 
        key: 'Permissions-Policy', 
        value: 'camera=(), microphone=(), geolocation=()' 
        }
      ],
      },
      {
      source: '/_next/image(.*)',
      headers: [
        { key: 'Cache-Control', value: 'public, max-age=31536000, immutable' }
      ]
      }
    ];
  },

  // Routing
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

  // Konfigurasi optimasi
  reactStrictMode: true,
  output: 'standalone',
  compress: true,
  swcMinify: true,
  productionBrowserSourceMaps: false,

  // Konfigurasi eksperimental
  experimental: {
    optimizeCss: true,
    nextScriptWorkers: true,
    isrMemoryCacheSize: 50,
  },

  // Webpack optimasi
  webpack: (config) => {
    config.optimization.splitChunks = {
      chunks: 'all',
      maxSize: 256000,
      minSize: 20000,
    };
    
    return config;
  }
};

module.exports = nextConfig;