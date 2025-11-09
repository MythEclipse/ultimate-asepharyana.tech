import { initializeDb, getDb, closeDb } from '@asepharyana/services';
import { config } from '../config';

let isInitialized = false;

export function getDatabase() {
  if (!isInitialized) {
    throw new Error('Database not initialized. Call connectDatabase first.');
  }
  return getDb();
}

export async function connectDatabase() {
  try {
    if (isInitialized) {
      console.log('✅ Database already connected');
      return true;
    }

    initializeDb(config.databaseUrl);
    isInitialized = true;
    console.log('✅ Database connected successfully');
    return true;
  } catch (error) {
    console.error('❌ Database connection failed:', error);
    throw error;
  }
}

export async function disconnectDatabase() {
  try {
    await closeDb();
    isInitialized = false;
    console.log('✅ Database disconnected successfully');
  } catch (error) {
    console.error('❌ Database disconnection failed:', error);
    throw error;
  }
}
