module.exports = {
  apps: [
    {
      name: 'asepharyana.cloud',
      script: 'bun',
      args: 'start',
      env: {
        NODE_ENV: 'production',
      },
      env_production: {
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
