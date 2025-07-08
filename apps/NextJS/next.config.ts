import { NextConfig } from 'next';
import withFlowbiteReact from 'flowbite-react/plugin/nextjs';

/** @type {import('next').NextConfig} */
const nextConfig: NextConfig = {
  // Menghasilkan output 'standalone' untuk optimasi dalam lingkungan Docker.
  // Ini adalah praktik terbaik untuk mengurangi ukuran image Docker.
  output: process.env.DOCKER === 'enable' ? 'standalone' : undefined,

  experimental: {
    // Flag 'optimizeCss' sudah usang dan digantikan oleh optimasi bawaan Next.js.
    // 'nextScriptWorkers' tidak kompatibel dengan App Router dan harus digunakan dengan hati-hati.
    nextScriptWorkers: true,
    // Optimasi memori Webpack, dianggap berisiko rendah dan bermanfaat untuk proyek besar.
    webpackMemoryOptimizations: true,
  },

  images: {
    // GANTI 'hostname: '**'' DENGAN DAFTAR HOSTNAME TEPERCAYA.
    // Menggunakan wildcard '**' adalah risiko keamanan yang sangat serius (Open Image Proxy).
    // Ganti dengan domain spesifik tempat gambar Anda di-hosting.
    remotePatterns: [
      // {
      //   protocol: 'https',
      //   hostname: '**',
      // },
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
    ],

    // Cache Time To Live (TTL) selama 1 hari. Nilai yang wajar untuk kebanyakan kasus.
    minimumCacheTTL: 86400,
    // Mengaktifkan format gambar modern untuk performa yang lebih baik.
    formats: ['image/webp', 'image/avif'],
    // Ukuran perangkat default. Sesuaikan jika desain Anda memiliki breakpoint gambar yang spesifik.
    deviceSizes: [640, 750, 828, 1080, 1200, 1920, 2048, 3840],
  },

  // KESALAHAN KRITIS PADA KONFIGURASI LAMA: JANGAN PERNAH MENARUH SECRET DI BLOK 'env'.
  // Variabel di sini akan diekspos ke sisi klien.
  // 'DATABASE_URL' dan 'SECRET' harus diakses hanya di sisi server melalui 'process.env'.
  // Praktik terbaik modern adalah menggunakan file .env untuk semua variabel.
  env: {
    NEXT_PUBLIC_KOMIK: process.env.NEXT_PUBLIC_KOMIK,
    NEXT_PUBLIC_ANIME: process.env.NEXT_PUBLIC_ANIME,
  },

  async redirects() {
    return [
      // Menggunakan 'permanent: false' (307 redirect) cocok jika struktur URL mungkin berubah.
      // Jika URL '/1' adalah tujuan kanonis dan stabil, pertimbangkan 'permanent: true' (308 redirect) untuk SEO yang lebih baik.
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

  // Mengaktifkan React Strict Mode adalah praktik terbaik untuk pengembangan.
  reactStrictMode: true,

  // Mengaktifkan kompresi Gzip. Jika proxy (misalnya Nginx) sudah menangani kompresi, setel ke 'false'.
  compress: true,

  compiler: {
    // Mengaktifkan optimasi SWC untuk styled-components.
    styledComponents: true,
    // Menghapus 'console.*' hanya pada build produksi untuk menjaga kebersihan konsol.
    removeConsole: process.env.NODE_ENV === 'production',
  },

  // Menghasilkan source map di produksi untuk mempermudah debugging.
  // PERHATIAN: Ini akan mengekspos kode sumber sisi klien Anda. Nonaktifkan jika ini menjadi masalah keamanan.
  productionBrowserSourceMaps: true,

  // Transpilasi paket dari node_modules atau monorepo.
  // '@asepharyana/ui' adalah contoh umum untuk paket UI lokal.
  transpilePackages: ['@asepharyana/ui', 'swagger-ui', '@asepharyana/database'],
};

export default withFlowbiteReact(nextConfig);
