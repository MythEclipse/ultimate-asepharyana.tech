module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'node',
      args: '-r dotenv/config start', // <-- ini load .env otomatis
      interpreter: 'none',
      cwd: __dirname,
      env: {
        NODE_ENV: 'production',
        DOTENV_CONFIG_PATH: './.env', // path ke file .env
      },
    },
  ],
};
