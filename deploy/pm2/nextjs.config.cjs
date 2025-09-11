module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'npm',
      args: 'run start',
      cwd: './apps/nextjs',
      exec_mode: 'fork',
      max_memory_restart: '1G',
      env: {
        NODE_ENV: 'production',
      },
    },
  ],
};
