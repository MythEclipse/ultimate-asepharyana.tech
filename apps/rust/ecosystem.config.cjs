module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: './target/release/rust',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/rust-error.log',
      out_file: 'logs/rust-out.log',
      env: {
        NODE_ENV: 'production',
        PORT: 4091,
      },
    },
  ],
};
