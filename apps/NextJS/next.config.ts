import { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "standalone",
  experimental: {
    optimizeCss: true,
    nextScriptWorkers: true,
  },
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "**",
      },
    ],
    minimumCacheTTL: 86400,
    formats: ["image/webp", "image/avif"],
    deviceSizes: [640, 750, 828, 1080, 1200, 1920, 2048, 3840],
  },
  env: {
    NEXT_PUBLIC_KOMIK: process.env.NEXT_PUBLIC_KOMIK,
    NEXT_PUBLIC_ANIME: process.env.NEXT_PUBLIC_ANIME,
    DATABASE_URL: process.env.DATABASE_URL,
    SECRET_KEY: process.env.SECRET_KEY,
  },
  async redirects() {
    return [
      {
        source: "/komik/:slug",
        destination: "/komik/:slug/1",
        permanent: false,
      },
      {
        source: "/anime/:slug",
        destination: "/anime/:slug/1",
        permanent: false,
      },
    ];
  },
  async headers() {
    return [
      {
        source: "/api/:path*",
        headers: [
          { key: "Access-Control-Allow-Origin", value: "*" },
          { key: "Access-Control-Allow-Methods", value: "GET, POST" },
          { key: "Access-Control-Allow-Headers", value: "Content-Type, Authorization" },
        ],
      },
    ];
  },
  reactStrictMode: true,
  compress: true,
  productionBrowserSourceMaps: true,
  webpack: (config) => {
    config.optimization.splitChunks = {
      chunks: "all",
      minSize: 20000,
      maxSize: 256000,
    };
    return config;
  },
  transpilePackages: ["@asepharyana/ui"],
};

export default nextConfig;
