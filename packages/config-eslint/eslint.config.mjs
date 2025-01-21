import { FlatCompat } from "@eslint/eslintrc";

const compat = new FlatCompat({
  baseDirectory: process.cwd(),
});

const loadConfig = async () => {
  const library = await import('./library.js');
  
  const eslintConfig = [
    ...compat.config({
      extends: library.default.extends,
      parser: "@typescript-eslint/parser",
      parserOptions: {
        project: "./tsconfig.json",
      },
      plugins: library.default.plugins,
      globals: library.default.globals,
      env: library.default.env,
      settings: library.default.settings,
      ignorePatterns: library.default.ignorePatterns,
      overrides: library.default.overrides,
    }),
  ];

  return eslintConfig;
};

export default await loadConfig();
