module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: './target/release/rustexpress',
      cwd: './apps/rust',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        PORT: 4099,
      },
    },
  ],
};
