module.exports = {
  apps: [
    {
      name: 'ultimate-asepharyana.cloud',
      script: 'npm',
      args: 'start',
      env: {
        NODE_ENV: 'production',
      },
      instances: 1,
      autorestart: true,
      watch: false,
      max_memory_restart: '2G',
      env_production: {
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
