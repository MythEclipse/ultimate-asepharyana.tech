module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'npm',
      args: 'run start',
      cwd: './apps/nextjs',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      out_file: './deploy/pm2/logs/nextjs-out.log',
      error_file: './deploy/pm2/logs/nextjs-error.log',
      env: {
        NODE_ENV: 'production',
      },
    },
  ],
};
