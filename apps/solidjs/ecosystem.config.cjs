// PM2 Ecosystem Configuration for SolidStart
module.exports = {
    apps: [
        {
            name: 'ultimate-solidjs',
            script: '/home/asephs/.bun/bin/bun',
            args: 'run start',
            cwd: process.env.VPS_TARGET_DIR
                ? `${process.env.VPS_TARGET_DIR}/apps/solidjs`
                : '/home/asephs/ultimate-asepharyana.cloud/apps/solidjs',
            interpreter: 'none',
            exec_mode: 'fork',
            autorestart: true,
            watch: false,
            max_memory_restart: '500M',
            env_production: {
                NODE_ENV: 'production',
                PORT: 4090,
            },
            env_development: {
                NODE_ENV: 'development',
                PORT: 4090,
            },
            // Logging configuration
            error_file: './logs/error.log',
            out_file: './logs/out.log',
            log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
            merge_logs: true,
        },
    ],
};
