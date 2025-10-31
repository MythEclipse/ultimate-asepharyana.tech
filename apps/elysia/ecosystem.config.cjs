// PM2 Ecosystem Configuration for ElysiaJS
module.exports = {
  apps: [
    {
      name: 'elysia-app',
      script: 'bun',
      args: 'run src/index.ts',
      cwd: '/opt/elysia-app/current',
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
      error_file: '/opt/elysia-app/logs/error.log',
      out_file: '/opt/elysia-app/logs/out.log',
      log_file: '/opt/elysia-app/logs/combined.log',
      time: true,
      merge_logs: true,
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
    },
  ],
};
