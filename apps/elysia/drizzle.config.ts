import { defineConfig } from 'drizzle-kit';

export default defineConfig({
  schema: './src/services/lib/schema.ts',
  out: './drizzle',
  dialect: 'mysql',
  dbCredentials: {
    url: process.env.DATABASE_URL || '',
  },
  verbose: true,
  strict: true,
});
