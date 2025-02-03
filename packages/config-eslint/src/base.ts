import js from "@eslint/js";
import eslintConfigPrettier from "eslint-config-prettier";
// @ts-ignore
import onlyWarn from "eslint-plugin-only-warn";
import turboPlugin from "eslint-plugin-turbo";
import tseslint from "typescript-eslint";

export const config: any[] = [
  js.configs.recommended,
  eslintConfigPrettier,
  ...tseslint.configs.recommended,
  {
    plugins: {
      turbo: turboPlugin,
    },
    rules: {
      "turbo/no-undeclared-env-vars": "warn",
    },
  },
  {
    plugins: {
      onlyWarn,
    },
  },
  {
    ignores: ["dist/**"],
  },
  {
    overrides: [
      {
        // Terapkan parserOptions.project hanya untuk file TypeScript
        files: ['**/*.ts', '**/*.tsx'],
        parserOptions: {
          project: './tsconfig.json'
        }
      },
      {
        // Untuk file JavaScript, nonaktifkan parserOptions.project agar tidak terjadi error
        files: ['**/*.js', '**/*.jsx'],
        parserOptions: {
          project: null
        }
      }
    ],
  },
];
