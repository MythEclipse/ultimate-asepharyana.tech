import { drizzle, MySql2Database } from 'drizzle-orm/mysql2';
import { createPool, Pool } from 'mysql2/promise';
import * as schema from './schema';

export type Database = MySql2Database<typeof schema>;

let dbInstance: Database | null = null;
let poolInstance: Pool | null = null;

export function initializeDb(databaseUrl: string): Database {
  if (dbInstance) {
    return dbInstance;
  }

  poolInstance = createPool(databaseUrl);

  dbInstance = drizzle(poolInstance, { schema, mode: 'default' });

  return dbInstance;
}

export function getDb(): Database {
  if (!dbInstance) {
    throw new Error('Database not initialized. Call initializeDb first.');
  }
  return dbInstance;
}

export async function closeDb() {
  if (poolInstance) {
    await poolInstance.end();
    poolInstance = null;
    dbInstance = null;
  }
}
