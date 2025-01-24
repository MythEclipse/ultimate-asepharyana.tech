module.exports = {
  apps: [
    {
      name: 'express',
      script: 'npm',
      interpreter: 'node',
      exec_mode: 'cluster',  // Mode cluster untuk load balancing
      instances: 'max',      // Gunakan semua core CPU
      args: 'run express',
      env: {
        NODE_ENV: 'production',
      },
      autorestart: true,
      watch: false,
      max_memory_restart: '1G',
      env_production: {
        DOTENV_CONFIG_PATH: './.env',
      },
    },
    {
      name: 'nextjs',
      script: 'npm',
      args: 'run nextjs',
      interpreter: 'node',
      exec_mode: 'cluster',  // Mode cluster untuk load balancing
      instances: 'max',      // Gunakan semua core CPU
      autorestart: true,
      watch: false,
      max_memory_restart: '1G',
      env_production: {
        DOTENV_CONFIG_PATH: './.env',
      },
    },
  ],
};
