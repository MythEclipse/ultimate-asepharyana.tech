module.exports = {
  apps: [
    {
      name: 'RustExpress',
      script: 'npm', // gunakan npm/pnpm/yarn untuk start
      args: 'start', // jalankan script "start" di package.json
      interpreter: 'none', // biar PM2 tidak salah pilih interpreter
      cwd: __dirname, // jalankan dari folder sekarang
      node_args: '-r dotenv/config', // preload dotenv
      env: {
        NODE_ENV: 'development',
      },
      env_production: {
        NODE_ENV: 'production',
        DOTENV_CONFIG_PATH: './.env', // path ke .env
      },
    },
  ],
};
