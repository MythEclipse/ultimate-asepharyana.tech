// PM2 Ecosystem Configuration for ElysiaJS
module.exports = {
  apps: [
    {
      name: 'elysia-app',
      script: 'dist/index.js',
      interpreter: '/home/asephs/.bun/bin/bun',
      cwd: '/home/asephs/asepharyana.tech/apps/elysia',
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
