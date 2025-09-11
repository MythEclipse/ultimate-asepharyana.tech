module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: 'bash',
      args: '-c "./target/release/rust"',
      interpreter: 'none',
      cwd: 'apps/rust',
      instances: 'max',
      exec_mode: 'cluster',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        NODE_OPTIONS: '-r dotenv/config',
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
