import type { Config } from 'jest';

export default async (): Promise<Config> => ({
  projects: ['packages/services'],
});
