import { resolve } from "node:path";
import { FlatCompat } from "@eslint/eslintrc";

const compat = new FlatCompat({
  baseDirectory: process.cwd(),
});

const project = resolve(process.cwd(), "tsconfig.json");

const baseConfig = {
  globals: {
    React: true,
    JSX: true,
  },
  env: {
    node: true,
  },
  settings: {
    "import/resolver": {
      typescript: {
        project,
      },
    },
  },
  ignorePatterns: [
    ".*.js",
    "node_modules/",
    "dist/",
  ],
  overrides: [
    {
      files: ["*.js?(x)", "*.ts?(x)"],
    },
  ],
};

const config = {
  extends: [
    "prettier",
  ],
  plugins: ["only-warn", "prettier"],
  ...baseConfig,
};

export default config;
