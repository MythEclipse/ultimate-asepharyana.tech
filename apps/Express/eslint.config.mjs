import { FlatCompat } from '@eslint/eslintrc';
import pkg from '@eslint/js';
const { configs } = pkg;

const compat = new FlatCompat({
  baseDirectory: process.cwd(),
  recommendedConfig: configs.recommended,
});

const eslintConfig = [
  ...compat.config({
    extends: [
      'eslint:recommended',
      'plugin:@typescript-eslint/recommended',
      'prettier',
    ],
    env: {
      node: true,
      es2021: true,
    },
    parser: '@typescript-eslint/parser',
    parserOptions: {
      ecmaVersion: 2021,
      sourceType: 'module',
    },
    plugins: ['@typescript-eslint'], // Properly use an array for plugins
    rules: {
      '@typescript-eslint/no-unused-vars': 'warn',
      '@typescript-eslint/no-explicit-any': 'off',
    },
  }),
  {
    ignores: [
      '**/node_modules/**',
      '**/.git/**',
      '**/build/**',
      '**/coverage/**',
      '**/dist/**',
    ],
  },
];

export default eslintConfig;
