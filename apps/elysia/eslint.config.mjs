import baseConfig from '../../eslint.config.mjs';

export default [
  ...baseConfig,
  {
    files: ['**/*.ts', '**/*.tsx'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'warn',
      '@nx/enforce-module-boundaries': [
        'error',
        {
          allow: [
            'elysia',
            '@elysiajs/cors',
            '@elysiajs/jwt',
            'mysql2',
            'bcryptjs',
            'uuid',
            'ioredis',
            'nodemailer',
          ],
        },
      ],
    },
  },
];
