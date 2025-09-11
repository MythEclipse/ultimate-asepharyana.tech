module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: './target/release/rustexpress',
      cwd: './apps/rust',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      out_file: './deploy/pm2/logs/rust-out.log',
      error_file: './deploy/pm2/logs/rust-error.log',
      env: {
        NODE_ENV: 'production',
        PORT: 4099,
      },
    },
  ],
};
