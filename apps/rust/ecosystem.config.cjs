// PM2 Ecosystem Configuration for Rust
module.exports = {
  apps: [
    {
      name: 'ultimate-rust',
      script: 'target/release/rust',
      cwd: process.env.VPS_TARGET_DIR
        ? `${process.env.VPS_TARGET_DIR}/apps/rust`
        : '/home/asephs/ultimate-asepharyana.cloud/apps/rust',
      interpreter: 'none',
      exec_mode: 'fork',
      autorestart: true,
      watch: false,
      max_memory_restart: '1G',
      env_production: {
        NODE_ENV: 'production',
        PORT: 4091,
      },
      // Logging configuration
      error_file: './logs/error.log',
      out_file: './logs/out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
      merge_logs: true,
    },
  ],
};
