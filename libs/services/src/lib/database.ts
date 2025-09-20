import { Kysely, MysqlDialect, MysqlPool } from 'kysely';
import { createPool } from 'mysql2';
import { DB } from './types';

let dbInstance: Kysely<DB> | null = null;

export function initializeDb(databaseUrl: string) {
  if (dbInstance) {
    return dbInstance;
  }

  const dialect = new MysqlDialect({
    pool: async () =>
      Promise.resolve(
        createPool({
          uri: databaseUrl,
        }) as unknown as MysqlPool,
      ),
  });

  dbInstance = new Kysely<DB>({
    dialect,
  });

  return dbInstance;
}

export function getDb() {
  if (!dbInstance) {
    throw new Error('Database not initialized. Call initializeDb first.');
  }
  return dbInstance;
}
