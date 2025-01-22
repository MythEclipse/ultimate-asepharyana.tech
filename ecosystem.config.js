module.exports = {
  apps: [
    {
      name: 'turbo',
      script: 'node_modules/.bin/turbo', // Path ke Turbo CLI
      args: 'run start --parallel', // Jalankan perintah Turbo untuk production
      env: {
        NODE_ENV: 'production',
      },
      env_production: {
        NODE_ENV: 'production',
      },
      autorestart: true,
      watch: false,
      max_memory_restart: '2G',
    },
  ],
};
