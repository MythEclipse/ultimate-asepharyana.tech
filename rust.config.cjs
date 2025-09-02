module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: 'pnpm',
      args: 'run rust:start',
      interpreter: 'none',
      env: {
        NODE_ENV: 'production',
        NODE_OPTIONS: '-r dotenv/config',
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
