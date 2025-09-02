module.exports = {
  apps: [
    {
      name: 'asepharyana.tech',
      script: 'pnpm',
      args: 'rust:start', // cukup jalankan start saja
      interpreter: 'none',
      node_args: '-r dotenv/config', // <-- preload dotenv disini
      env: {
        NODE_ENV: 'production',
        DOTENV_CONFIG_PATH: './.env', // custom path kalau perlu
      },
    },
  ],
};
