export const preset = 'ts-jest';
export const testEnvironment = 'node';
export const moduleNameMapper = {
  '^@/(.*)$': '<rootDir>/src/$1',
};
export const setupFiles = ['dotenv/config'];