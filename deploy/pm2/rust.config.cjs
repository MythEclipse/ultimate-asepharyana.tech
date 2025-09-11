module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: 'bash',
      args: '-c "target/release/rust"',
      cwd: '../../apps/rust',
      interpreter: 'none',
      instances: 'max',
      exec_mode: 'cluster',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        PORT: 4099
      },
    },
  ],
};
