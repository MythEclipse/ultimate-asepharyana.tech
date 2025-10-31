// PM2 Ecosystem Configuration for ElysiaJS
module.exports = {
  apps: [
    {
      name: 'elysia-app',
      script: 'bun',
      args: 'run src/index.ts',
      cwd: './apps/elysia',
      interpreter: 'none',
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
      error_file: './apps/elysia/logs/error.log',
      out_file: './apps/elysia/logs/out.log',
      log_file: './apps/elysia/logs/combined.log',
      time: true,
      merge_logs: true,
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
    },
  ],
};
