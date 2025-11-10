module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: '/home/asephs/.bun/bin/bun',
      args: 'run start',
      cwd: process.env.VPS_TARGET_DIR ? `${process.env.VPS_TARGET_DIR}/apps/nextjs` : '/home/asephs/asepharyana.tech/apps/nextjs',
      interpreter: 'none',
      exec_mode: 'fork',
      autorestart: true,
      watch: false,
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
