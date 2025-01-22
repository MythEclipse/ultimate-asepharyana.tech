module.exports = {
  apps: [
    {
      name: 'express',
      script: 'turbo',
      args: 'run express',
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
    {
      name: 'nextjs',
      script: 'turbo',
      args: 'run nextjs',
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
