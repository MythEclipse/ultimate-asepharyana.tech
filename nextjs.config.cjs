module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'pnpm',
      args: 'run nextjs:start',
      interpreter: 'none',
      env: {
        NODE_ENV: 'production',
        NODE_OPTIONS: '-r dotenv/config',
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
