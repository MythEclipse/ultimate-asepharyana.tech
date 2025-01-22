module.exports = {
  apps: [
    {
      name: 'express',
      script: 'npm',
      interpreter: 'node',
      exec_mode: 'fork',
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
      script: 'npm',
      args: 'run nextjs',
      interpreter: 'node',
      exec_mode: 'fork',
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
