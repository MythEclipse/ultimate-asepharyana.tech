module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: './target/release/rust',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        PORT: 4091,
      },
    },
  ],
};
