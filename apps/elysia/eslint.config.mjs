import baseConfig from '../../eslint.config.mjs';

export default [
  ...baseConfig,
  {
    files: ['**/*.ts', '**/*.tsx'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'warn',
      // Removed @nx/enforce-module-boundaries as @nx is not installed
    },
  },
];
