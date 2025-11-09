module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'bun',
      args: 'run start',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
        PORT: 4090,
      },
      env_development: {
        NODE_ENV: 'development',
        PORT: 4090,
      },
    },
  ],
};
