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
      error_file: 'logs/rust-error.log',
      out_file: 'logs/rust-out.log',
      combine_output: true,
      env: {
        NODE_ENV: 'production',
        NODE_OPTIONS: '-r dotenv/config',
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
