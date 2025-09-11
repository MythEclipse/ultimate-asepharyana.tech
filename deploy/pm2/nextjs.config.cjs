module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'node_modules/.bin/next',
      args: 'start',
      cwd: 'apps/nextjs',
      instances: 'max',
      exec_mode: 'cluster',
      max_memory_restart: '1G',
      error_file: 'logs/nextjs-error.log',
      out_file: 'logs/nextjs-out.log',
      combine_output: true,
      env: {
        NODE_ENV: 'production',
      },
    },
  ],
};
