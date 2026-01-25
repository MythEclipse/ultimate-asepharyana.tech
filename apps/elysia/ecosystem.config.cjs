/* global module, process */
// PM2 Ecosystem Configuration for ElysiaJS
module.exports = {
  apps: [
    {
      name: 'ultimate-elysia',
      script: 'dist/index.js',
      interpreter: '/home/asephs/.bun/bin/bun',
      cwd: process.env.VPS_TARGET_DIR
        ? `${process.env.VPS_TARGET_DIR}/apps/elysia`
        : '/home/asephs/ultimate-asepharyana.cloud/apps/elysia',
      instances: 1,
      exec_mode: 'fork',
      autorestart: true,
      watch: false,
      max_memory_restart: '500M',
      env_production: {
        NODE_ENV: 'production',
        PORT: 4092,
      },
      // Logging configuration
      error_file: './logs/error.log',
      out_file: './logs/out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
      merge_logs: true,
    },
  ],
};
