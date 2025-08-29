import { Kysely, MysqlDialect } from 'kysely';
// eslint-disable-next-line @nx/enforce-module-boundaries
import { createPool } from 'mysql2';
import type { DB } from './types'; // We will define this type later

const dialect = new MysqlDialect({
  pool: createPool({
    uri: process.env.DATABASE_URL,
  }),
});

export const db = new Kysely<DB>({
  dialect,
});
