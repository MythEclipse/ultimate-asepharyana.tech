module.exports = {
    apps: [
        {
            name: 'express',
            script: './apps/Express/dist/index.js',
            instances: 'max',
            exec_mode: 'cluster',
            env: {
                NODE_ENV: 'development',
            },
            env_production: {
                NODE_ENV: 'production',
            },
        },
    ],
};