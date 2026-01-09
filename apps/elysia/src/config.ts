export const config = {
  port: 4092,
  env: process.env.NODE_ENV || 'development',
  isDevelopment: process.env.NODE_ENV !== 'production',
  isProduction: process.env.NODE_ENV === 'production',
  jwtSecret: process.env.JWT_SECRET || 'default_secret_change_this',
  databaseUrl: process.env.DATABASE_URL || '',
  redisUrl: process.env.REDIS_URL || 'redis://localhost:6379',
  googleClientId:
    process.env.GOOGLE_CLIENT_ID || process.env.GOOGLE_OAUTH_CLIENT_ID || '',
  // MinIO / S3-compatible storage configuration
  minio: {
    // Accept both host or full URL in MINIO_ENDPOINT
    ...((): { endPoint: string; port?: number; useSSL: boolean } => {
      const raw = process.env.MINIO_ENDPOINT || 'localhost';
      if (raw.startsWith('http://') || raw.startsWith('https://')) {
        try {
          const u = new URL(raw);
          return {
            endPoint: u.hostname,
            port: u.port ? Number(u.port) : undefined,
            useSSL: u.protocol === 'https:',
          };
        } catch {
          // Fallback to host parsing
        }
      }
      return {
        endPoint: raw,
        port: process.env.MINIO_PORT
          ? Number(process.env.MINIO_PORT)
          : undefined,
        useSSL:
          process.env.MINIO_USE_SSL === 'true' ||
          process.env.MINIO_SECURE === 'true',
      };
    })(),
    accessKey: process.env.MINIO_ACCESS_KEY || '',
    secretKey: process.env.MINIO_SECRET_KEY || '',
    bucket:
      process.env.MINIO_BUCKET || process.env.MINIO_BUCKET_NAME || 'avatars',
    region: process.env.MINIO_REGION || 'us-east-1',
    // Public base URL to serve files (e.g., https://cdn.example.com or http://host:port)
    publicUrl: process.env.MINIO_PUBLIC_URL || '',
    // Optional object prefix for avatars
    avatarPrefix: process.env.MINIO_AVATAR_PREFIX || 'avatars',
  },
};
