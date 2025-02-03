import { nextJsConfig } from "@asepharyana/config-eslint/next-js";

const config = [
  ...nextJsConfig,
  {
    rules: {
      "@typescript-eslint/no-unused-vars": "off",
      "@typescript-eslint/no-explicit-any": "off",
    },
  },
];

export default config;
