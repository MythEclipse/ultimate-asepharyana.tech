import { initializeDb, getDb, closeDb } from '@asepharyana/services';
import { config } from '../src/config';

async function main() {
  try {
    console.log('Connecting to database...');
    initializeDb(config.databaseUrl);
    const db = getDb();

    console.log('Listing tables...');
    const rows = await db.execute<any>('SHOW TABLES');
    console.log('Tables:', rows);
  } catch (err) {
    console.error('Error querying tables:', err);
  } finally {
    await closeDb();
  }
}

main();
