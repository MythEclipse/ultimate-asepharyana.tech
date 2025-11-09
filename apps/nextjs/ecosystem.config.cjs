module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'bun',
      args: 'run start',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/nextjs-error.log',
      out_file: 'logs/nextjs-out.log',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        PORT: 4090,
      },
      env_development: {
        NODE_ENV: 'development',
        PORT: 4090,
      },
    },
  ],
};
