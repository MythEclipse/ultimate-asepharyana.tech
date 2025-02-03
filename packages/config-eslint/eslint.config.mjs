import { resolve } from "node:path";
// @ts-ignore
import { FlatCompat } from "@eslint/eslintrc";

const compat = new FlatCompat({
  baseDirectory: process.cwd(),
});

const project = resolve(process.cwd(), "tsconfig.eslint.json");

const baseConfig = {
  languageOptions: {
    globals: {
      React: "readonly",
      JSX: "readonly",
    },
  },
  settings: {
    "import/resolver": {
      typescript: {
        project,
      },
    },
  },
  ignores: ["*.js", "node_modules/", "dist/**"],
  rules: {},
};

const config = [
  {
    plugins: {
      "only-warn": {},
      prettier: {},
    },
    ...baseConfig,
  },
  {
    files: ["*.ts", "*.tsx"],
    languageOptions: {
      parserOptions: {
        project,
      },
    },
  },
];

export default config;
