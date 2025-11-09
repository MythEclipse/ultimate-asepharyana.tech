module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'pnpm',
      args: 'run start',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/nextjs-error.log',
      out_file: 'logs/nextjs-out.log',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
      },
    },
  ],
};
