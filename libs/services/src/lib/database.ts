import { drizzle } from 'drizzle-orm/mysql2';
import { createPool, Pool } from 'mysql2/promise';
import * as schema from './schema';

let dbInstance: ReturnType<typeof drizzle> | null = null;
let poolInstance: Pool | null = null;

export function initializeDb(databaseUrl: string) {
  if (dbInstance) {
    return dbInstance;
  }

  poolInstance = createPool(databaseUrl);

  dbInstance = drizzle(poolInstance, { schema, mode: 'default' });

  return dbInstance;
}

export function getDb() {
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
