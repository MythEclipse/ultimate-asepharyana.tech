module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'npm',
      args: 'run start',
      cwd: './apps/nextjs',
      instances: 'max',
      exec_mode: 'cluster',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
      },
    },
  ],
};
