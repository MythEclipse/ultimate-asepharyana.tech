module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'package.json',
      args: 'start',
      interpreter: 'bun',
      cwd: __dirname,
      env: {
        NODE_ENV: 'production',
      },
      env_production: {
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};