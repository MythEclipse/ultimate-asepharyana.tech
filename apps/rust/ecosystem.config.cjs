module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: 'target/release/rust',
      cwd: process.env.VPS_TARGET_DIR ? `${process.env.VPS_TARGET_DIR}/apps/rust` : '/home/asephs/asepharyana.tech/apps/rust',
      interpreter: 'none',
      exec_mode: 'fork',
      autorestart: true,
      watch: false,
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        PORT: 4091,
      },
    },
  ],
};
