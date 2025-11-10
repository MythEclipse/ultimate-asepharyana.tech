// PM2 Ecosystem Configuration for ElysiaJS
module.exports = {
  apps: [
    {
      name: 'elysia-app',
      script: 'dist/index.js',
      interpreter: 'bun',
      cwd: process.env.VPS_TARGET_DIR ? `${process.env.VPS_TARGET_DIR}/apps/elysia` : '/home/asephs/asepharyana.tech/apps/elysia',
      instances: 1,
      exec_mode: 'fork',
      autorestart: true,
      watch: false,
      max_memory_restart: '500M',
      env: {
        NODE_ENV: 'production',
        PORT: 4092,
      },
      env_development: {
        NODE_ENV: 'development',
        PORT: 4092,
      },
    },
  ],
};
