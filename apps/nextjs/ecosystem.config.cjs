// PM2 Ecosystem Configuration for Next.js
module.exports = {
  apps: [
    {
      name: 'ultimate-nextjs',
      script: '/home/asephs/.bun/bin/bun',
      args: 'run start',
      cwd: process.env.VPS_TARGET_DIR
        ? `${process.env.VPS_TARGET_DIR}/apps/nextjs`
        : '/home/asephs/ultimate-asepharyana.cloud/apps/nextjs',
      interpreter: 'none',
      exec_mode: 'fork',
      autorestart: true,
      watch: false,
      max_memory_restart: '1G',
      env_production: {
        NODE_ENV: 'production',
        PORT: 4090,
      },
      env_development: {
        NODE_ENV: 'development',
        PORT: 4090,
      },
      // Logging configuration
      error_file: './logs/error.log',
      out_file: './logs/out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
      merge_logs: true,
    },
  ],
};
