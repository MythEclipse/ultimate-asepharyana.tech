module.exports = {
  apps: [
    {
      name: 'express',
      script: 'pnpm',
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
      script: 'pnpm',
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
