import baseConfig from '../../eslint.config.mjs';

export default [
  ...baseConfig,
  {
    files: ['**/*.js'],
    languageOptions: {
      globals: {
        console: 'readonly',
      },
    },
  },
  {
    files: ['**/*.json'],
    rules: {
      // Removed @nx/dependency-checks as @nx is not installed
    },
    languageOptions: {
      parser: await import('jsonc-eslint-parser/lib/index.js'),
    },
  },
];
